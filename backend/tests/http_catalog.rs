use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use backend::api;
use backend::api::state::AppState;
use backend::catalog::CatalogService;
use serde_json::Value;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use study_core::catalog::normalize_search_text;
use study_storage_sqlite::catalog::SqliteCatalogRepository;
use tower::ServiceExt;

async fn fixture() -> (Router, SqlitePool) {
    let options = SqliteConnectOptions::new()
        .in_memory(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("./crates/study-storage-sqlite/migrations")
        .run(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO teachers VALUES (1, 'Иванова Мария', 'Иванова М.')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO disciplines (id, slug, name, course, semester, teacher_id) VALUES (1, 'database', 'Базы данных', 3, 5, 1)")
        .execute(&pool)
        .await
        .unwrap();
    for (id, kind, title) in [
        (1001, "book", "Учёт транзакций"),
        (1002, "video", "Разбор транзакций"),
        (1003, "lecture", "Индексы"),
    ] {
        sqlx::query("INSERT INTO materials (id, title, material_type, discipline_id, teacher_id, course, year, created_at, search_text, sort_title) VALUES (?, ?, ?, 1, 1, 3, 2026, '2026-09-01', ?, ?)")
            .bind(id)
            .bind(title)
            .bind(kind)
            .bind(normalize_search_text(title))
            .bind(normalize_search_text(title))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO material_tags VALUES (?, 'SQL')")
            .bind(id)
            .execute(&pool)
            .await
            .unwrap();
    }
    let repository = SqliteCatalogRepository::new(pool.clone());
    let router = api::router(AppState::new(CatalogService::new(repository)));
    (router, pool)
}

async fn request(app: Router, method: Method, uri: &str) -> (StatusCode, Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body = serde_json::from_slice(&bytes).unwrap();
    (status, body)
}

#[tokio::test]
async fn catalog_filters_pagination_and_json_contract() {
    let (app, _) = fixture().await;
    let (status, body) = request(
        app.clone(),
        Method::GET,
        "/api/catalog?type=book&type=video&tag=SQL&pageSize=1&page=99",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["total"], 2);
    assert_eq!(body["page"], 2);
    assert_eq!(body["pageCount"], 2);
    assert_eq!(body["items"].as_array().unwrap().len(), 1);
    assert_eq!(body["items"][0]["tags"][0], "SQL");
    assert_eq!(body["items"][0]["teacher"]["shortName"], "Иванова М.");
    assert_eq!(body["facets"]["types"]["lecture"], 1);

    let query = serde_urlencoded::to_string([("q", "УЧЕТ")]).unwrap();
    let (_, body) = request(app.clone(), Method::GET, &format!("/api/catalog?{query}")).await;
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"][0]["id"], 1001);

    let (_, body) = request(app, Method::GET, "/api/catalog?q=%25&page=99").await;
    assert_eq!(body["total"], 0);
    assert_eq!(body["page"], 1);
    assert_eq!(body["pageCount"], 1);
}

#[tokio::test]
async fn invalid_queries_return_bad_request() {
    let (app, _) = fixture().await;
    for uri in [
        "/api/catalog?pageSize=5&pageSize=10",
        "/api/catalog?pageSize=0",
        "/api/catalog?pageSize=101",
        "/api/catalog?page=0",
        "/api/catalog?type=unknown",
        "/api/catalog?discipline=-1",
        "/api/catalog?unknown=1",
    ] {
        let (status, body) = request(app.clone(), Method::GET, uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri}");
        assert_eq!(body["error"]["code"], "bad_request", "{uri}");
    }
}

#[tokio::test]
async fn material_health_and_error_routes() {
    let (app, _) = fixture().await;
    let (status, body) = request(app.clone(), Method::GET, "/api/materials/1001").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], 1001);
    assert_eq!(body["type"], "book");
    assert_eq!(body["discipline"]["slug"], "database");
    let (status, _) = request(app.clone(), Method::GET, "/api/materials/99999").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = request(app.clone(), Method::GET, "/api/materials/abc").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let (status, body) = request(app.clone(), Method::GET, "/api/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    let (status, _) = request(app.clone(), Method::GET, "/api/missing").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, body) = request(app, Method::POST, "/api/catalog").await;
    assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(body["error"]["code"], "method_not_allowed");
}

#[tokio::test]
async fn unavailable_database_returns_generic_service_unavailable() {
    let (app, pool) = fixture().await;
    pool.close().await;
    for uri in ["/api/catalog", "/api/materials/1001"] {
        let (status, body) = request(app.clone(), Method::GET, uri).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(body["error"]["code"], "service_unavailable");
        assert_eq!(
            body["error"]["message"],
            "Хранилище временно недоступно. Попробуйте позже"
        );
    }
}

#[tokio::test]
async fn invalid_stored_material_returns_generic_internal_error() {
    let (app, pool) = fixture().await;
    // Simulate a damaged database without modifying the production migration.
    sqlx::query("PRAGMA ignore_check_constraints = ON")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE materials SET course = 0 WHERE id = 1001")
        .execute(&pool)
        .await
        .unwrap();
    for uri in ["/api/catalog", "/api/materials/1001"] {
        assert_internal_error(app.clone(), uri).await;
    }
}

#[tokio::test]
async fn missing_required_reference_returns_internal_error_instead_of_not_found() {
    for statement in [
        "UPDATE materials SET teacher_id = 99999 WHERE id = 1001",
        "UPDATE materials SET discipline_id = 99999 WHERE id = 1001",
    ] {
        let (app, pool) = fixture().await;
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(statement).execute(&pool).await.unwrap();
        for uri in ["/api/catalog", "/api/materials/1001"] {
            assert_internal_error(app.clone(), uri).await;
        }
    }
}

#[tokio::test]
async fn unexpected_database_failure_returns_generic_internal_error() {
    let (app, pool) = fixture().await;
    sqlx::query("DROP TABLE material_tags")
        .execute(&pool)
        .await
        .unwrap();
    assert_internal_error(app, "/api/catalog").await;
}

async fn assert_internal_error(app: Router, uri: &str) {
    let (status, body) = request(app, Method::GET, uri).await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        body,
        serde_json::json!({
            "error": {
                "code": "internal_error",
                "message": "Не удалось обработать запрос",
            }
        })
    );
}

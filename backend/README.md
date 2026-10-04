# Локальный Rust/Axum backend — PostgreSQL

Активный workspace: backend, study-core, study-storage-postgres. PostgreSQL 18 запускается Docker Compose, Rust и Vite работают на Windows. SQLite-адаптер сохранён в archive/backend-sqlite и не входит в сборку.

## Первый запуск

В корне проекта нужен игнорируемый .env с POSTGRES_PASSWORD и DATABASE_URL. Во время настройки файл уже создан; не заменяйте его другим паролем для существующего volume.

Для нового окружения задайте собственный пароль вместо PLACEHOLDER:

~~~dotenv
POSTGRES_PASSWORD=PLACEHOLDER
DATABASE_URL=postgres://study_dev:PLACEHOLDER@127.0.0.1:5433/study_dev
~~~

При специальных символах пароль в DATABASE_URL должен быть URL-encoded. Пример дополнительных настроек находится в backend/.env.example. Секреты в Git не добавляются.

Из корня проекта:

~~~powershell
docker compose up -d postgres
docker compose ps
~~~

Из backend:

~~~powershell
cargo run --bin seed
cargo run --bin backend
~~~

Обычный запуск backend применяет migrations, но не добавляет демоданные. Seed запускается явно. .env загружается dotenvy из текущей папки или родительской; существующие переменные окружения имеют приоритет.

База: study_dev, роль: study_dev, адрес: 127.0.0.1:5433. Данные хранятся в named volume study_postgres. docker compose stop останавливает БД без удаления данных. Для остановки не требуется удалять volume.

## Команды

~~~powershell
cargo run --bin seed
cargo run --bin catalog_preview
cargo run --bin catalog_preview -- "транзакции"
cargo run --bin rebuild_search
~~~

- seed: 840 вымышленных материалов, 3 учреждения, 8 пользователей, 11 contributors, 7 дисциплин. Уже существующие fixture_key пропускаются; публикации и отозванные права не перезаписываются.
- catalog_preview: первая публичная страница, необязательный текст поиска одним аргументом.
- rebuild_search: явная пересборка производного поискового индекса. На время операции блокирует записи materials; не запускается автоматически.

Не все 840 материалов доступны публично: seed содержит Private и неопубликованные Draft. Часть публикаций имеет новую редакцию Draft, которая не меняет публичную карточку.

## Конфигурация

| Переменная | Значение |
| --- | --- |
| DATABASE_URL | Обязательная PostgreSQL URL |
| BIND_ADDRESS | По умолчанию 127.0.0.1:3000 |
| DATABASE_MAX_CONNECTIONS | По умолчанию 10, диапазон 1..100 |
| DATABASE_ACQUIRE_TIMEOUT_SECONDS | По умолчанию 5, диапазон 1..60 |
| RUST_LOG | Фильтр логов сервера |

Соединения используют UTC, statement_timeout=15s, lock_timeout=5s. DATABASE_PATH больше не используется. SQLite fallback и импорт старого файла отсутствуют.

## HTTP и frontend

Внутренние write-команды имеют ограниченный retry всей транзакции для PostgreSQL 40001/40P01: максимум три попытки, паузы 25/50 мс. Business conflict, сетевой отказ и неизвестный результат COMMIT не повторяются. Публичных write endpoints и HTTP-idempotency пока нет.

- GET /api/health — ответ HTTP-процесса, без отдельного запроса readiness к БД.
- GET /api/catalog — query q; повторяемые type, discipline, teacher, course, year, tag; sort, page, pageSize.
- GET /api/materials/{id} — текущая публичная публикация.

DTO включает publishedRevisionId, credits с ролями и snapshot имени, tags, placements, nullable sourceYear/preview/durationSeconds/fileSizeBytes, publishedAt и firstPublishedAt. Материал может быть самостоятельным, без placement.

teacher в query обозначает ответственного пользователя учебного размещения. Этот пользователь не становится автором материала автоматически. Несколько учебных фильтров должны совпадать в одном размещении.

pageSize=1..100; пустой результат имеет pageCount=0. Страница вне диапазона возвращает пустые items без изменения запрошенного page. Facets считаются по всему публичному каталогу.

400 — неверный запрос; 404 — отсутствие публичного материала; 405 — неподдерживаемый метод; 503 — недоступность/временный отказ хранения; 500 — некорректные сохранённые данные или неожиданная ошибка. Ответ: { "error": { "code": "...", "message": "..." } }.

Из frontend запустите npm run dev, откройте http://127.0.0.1:5173/catalog?all=1. Vite проксирует /api на 127.0.0.1:3000. Дисциплины/подборки ещё используют отдельные demo-данные до этапа 11.

## Проверки сборки

~~~powershell
cargo check --workspace
cargo clippy --workspace --bins --lib -- -D warnings
cargo fmt --all --check
~~~

В этой реализации автоматические тесты не добавлялись и не запускались. Старые SQLite HTTP-тесты находятся в архиве; они не проверяют новую PostgreSQL-модель. Конкурентные команды, повторный seed, rollback и браузерные сценарии требуют отдельного запроса на проверку.

## Документация

Подробные решения сохранены локально:
- .codex/docs/backend-storage-stage.md — объём этапа 10 и следующий этап.
- .codex/docs/backend-function-semantics.md — функции, входы/результаты, ошибки, блокировки и аллокации.
- .codex/docs/backend-postgres-contract.md — текущий HTTP-контракт.

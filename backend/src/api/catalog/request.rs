use std::collections::HashSet;
use std::str::FromStr;

use study_core::catalog::{CatalogQuery, CatalogSort, MaterialType};
use study_core::ids::{DisciplineId, TeacherId};

use crate::api::error::ApiError;

pub fn parse_catalog_query(raw_query: Option<&str>) -> Result<CatalogQuery, ApiError> {
    let raw_query = raw_query.unwrap_or_default();

    if raw_query.len() > 8 * 1024 {
        return Err(ApiError::bad_request("Слишком длинный запрос"));
    }

    let pairs: Vec<(String, String)> = serde_urlencoded::from_str(raw_query)
        .map_err(|_| ApiError::bad_request("Не удалось разобрать параметры запроса"))?;

    if pairs.len() > 100 {
        return Err(ApiError::bad_request("Слишком много параметров запроса"));
    }

    let mut query = CatalogQuery {
        text: String::new(),
        material_types: Vec::new(),
        discipline_ids: Vec::new(),
        teacher_ids: Vec::new(),
        courses: Vec::new(),
        years: Vec::new(),
        tags: Vec::new(),
        sort: CatalogSort::Newest,
        page: 1,
        page_size: 5,
    };

    let mut scalar_fields = HashSet::new();

    for (key, value) in pairs {
        let is_scalar = matches!(key.as_str(), "q" | "sort" | "page" | "pageSize");

        if is_scalar && !scalar_fields.insert(key.clone()) {
            return Err(ApiError::bad_request(format!(
                "Параметр {key} указан несколько раз"
            )));
        }

        match key.as_str() {
            "q" => {
                if value.chars().count() > 256 {
                    return Err(ApiError::bad_request(
                        "Поисковый запрос длиннее 256 символов",
                    ));
                }

                query.text = value.trim().to_owned();
            }

            "type" => {
                let material_type = MaterialType::from_db_value(&value).ok_or_else(|| {
                    ApiError::bad_request(format!("Неизвестный тип материала: {value}"))
                })?;

                query.material_types.push(material_type);
            }

            "discipline" => {
                let id = positive_id(&value, "discipline")?;
                query.discipline_ids.push(DisciplineId(id));
            }

            "teacher" => {
                let id = positive_id(&value, "teacher")?;
                query.teacher_ids.push(TeacherId(id));
            }

            "course" => {
                let course: u8 = number(&value, "course")?;

                if course == 0 {
                    return Err(ApiError::bad_request("course должен быть от 1 до 255"));
                }

                query.courses.push(course);
            }

            "year" => {
                let year: i32 = number(&value, "year")?;

                if !(1..=9999).contains(&year) {
                    return Err(ApiError::bad_request("год(year) должен быть от 1 до 9999"));
                }

                query.years.push(year);
            }

            "tag" => {
                let tag = value.trim();

                if tag.is_empty() || tag.chars().count() > 80 {
                    return Err(ApiError::bad_request(
                        "Тег должен содержать от 1 до 80 символов",
                    ));
                }

                query.tags.push(tag.to_owned());
            }

            "sort" => {
                query.sort = match value.as_str() {
                    "newest" => CatalogSort::Newest,
                    "oldest" => CatalogSort::Oldest,
                    "title" => CatalogSort::Title,

                    _ => {
                        return Err(ApiError::bad_request(
                            "sort должен быть newest, oldest или title",
                        ));
                    }
                };
            }

            "page" => {
                let page: u32 = number(&value, "page")?;

                if page == 0 {
                    return Err(ApiError::bad_request("page должен быть больше нуля"));
                }

                query.page = page;
            }

            "pageSize" => {
                let page_size: u32 = number(&value, "pageSize")?;

                // Ограничение на размер страницы, чтобы избежать слишком больших запросов
                if !(1..=100).contains(&page_size) {
                    return Err(ApiError::bad_request("pageSize должен быть от 1 до 100"));
                }

                query.page_size = page_size;
            }

            _ => {
                return Err(ApiError::bad_request(format!(
                    "Неизвестный параметр: {key}"
                )));
            }
        }
    }

    Ok(query)
}

fn number<T: FromStr>(value: &str, field: &str) -> Result<T, ApiError> {
    value
        .parse()
        .map_err(|_| ApiError::bad_request(format!("Некорректное числовое значение {field}")))
}

fn positive_id(value: &str, field: &str) -> Result<i64, ApiError> {
    let id: i64 = number(value, field)?;

    if id <= 0 {
        return Err(ApiError::bad_request(format!(
            "{field} должен быть положительным ID"
        )));
    }

    Ok(id)
}

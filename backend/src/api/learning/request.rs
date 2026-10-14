use crate::api::error::ApiError;
use std::collections::HashSet;
use study_core::ids::{DisciplineId, InstitutionId};
use study_core::learning::{LearningQuery, LearningSort};

pub(super) fn parse(raw: Option<&str>, collection: bool) -> Result<LearningQuery, ApiError> {
    let raw = raw.unwrap_or("");
    if raw.len() > 8192 {
        return Err(ApiError::bad_request("Запрос слишком длинный"));
    }
    let mut query = LearningQuery::default();
    let mut scalars = HashSet::new();
    let mut count = 0;
    let pairs: Vec<(String, String)> = serde_urlencoded::from_str(raw)
        .map_err(|_| ApiError::bad_request("Некорректные параметры запроса"))?;
    for (name, value) in pairs {
        count += 1;
        if count > 100 {
            return Err(ApiError::bad_request("Слишком много параметров"));
        }
        let name = name.as_str();
        if name != "discipline" && name != "course" && !scalars.insert(name.to_owned()) {
            return Err(ApiError::bad_request("Одиночный параметр указан повторно"));
        }
        match name {
            "q" => query.text = value,
            "page" => query.pagination.page = number(&value)?,
            "pageSize" => query.pagination.page_size = number(&value)?,
            "institution" => query.institution_id = Some(InstitutionId(number(&value)?)),
            "discipline" if collection => query.discipline_ids.push(DisciplineId(number(&value)?)),
            "course" if collection => query.courses.push(number(&value)?),
            "sort" => {
                query.sort = match value.as_str() {
                    "newest" if collection => LearningSort::Newest,
                    "oldest" if collection => LearningSort::Oldest,
                    "title" => LearningSort::Title,
                    _ => return Err(ApiError::bad_request("Неизвестная сортировка")),
                }
            }
            _ => return Err(ApiError::bad_request("Неизвестный параметр запроса")),
        }
    }
    query
        .validate()
        .map_err(|error| ApiError::bad_request(error.to_string()))?;
    Ok(query)
}
fn number<T: std::str::FromStr>(value: &str) -> Result<T, ApiError> {
    value
        .parse()
        .map_err(|_| ApiError::bad_request("Некорректное числовое значение"))
}
pub(super) fn id(value: &str) -> Result<i64, ApiError> {
    let id: i64 = number(value)?;
    if id <= 0 {
        return Err(ApiError::bad_request("ID должен быть положительным"));
    }
    Ok(id)
}

pub(super) fn pagination(raw: Option<&str>) -> Result<study_core::learning::PageRequest, ApiError> {
    let raw = raw.unwrap_or("");
    if raw.len() > 256 {
        return Err(ApiError::bad_request("Запрос страницы слишком длинный"));
    }
    let pairs: Vec<(String, String)> = serde_urlencoded::from_str(raw)
        .map_err(|_| ApiError::bad_request("Некорректные параметры страницы"))?;
    let mut page = study_core::learning::PageRequest::default();
    let mut seen = 0_u8;
    for (name, value) in pairs {
        let mask = match name.as_str() {
            "page" => 1,
            "pageSize" => 2,
            _ => return Err(ApiError::bad_request("Неизвестный параметр страницы")),
        };
        if seen & mask != 0 {
            return Err(ApiError::bad_request("Повтор параметра страницы"));
        }
        seen |= mask;
        if mask == 1 {
            page.page = number(&value)?;
        } else {
            page.page_size = number(&value)?;
        }
    }
    page.validate()
        .map_err(|error| ApiError::bad_request(error.to_string()))?;
    Ok(page)
}

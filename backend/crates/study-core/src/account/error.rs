use crate::error::RepositoryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("{0}")]
    Validation(&'static str),
    #[error("Неверный email или пароль")]
    InvalidCredentials,
    #[error("Необходимо войти в аккаунт")]
    Unauthenticated,
    #[error("Сессия истекла или отозвана")]
    SessionExpired,
    #[error("Действие не разрешено")]
    Forbidden,
    #[error("Проверка источника запроса не пройдена")]
    Csrf,
    #[error("Email уже занят")]
    EmailTaken,
    #[error("Запись не найдена или недоступна")]
    NotFound,
    #[error("Данные уже изменились. Обновите страницу")]
    VersionConflict,
    #[error("Автор уже связан с другим аккаунтом")]
    ContributorAlreadyLinked,
    #[error("Слишком много запросов. Попробуйте позже")]
    RateLimited,
    #[error("{0}")]
    Storage(#[from] RepositoryError),
}
pub type AccountResult<T> = Result<T, AccountError>;

pub mod admin;
pub mod collections;
mod credentials;
pub mod library;
mod profile;
mod requests;
mod session;
pub use credentials::Credential;
pub use requests::{
    IdentityRequests, InstitutionPage, InstitutionReference, LinkRequest, MembershipRequest,
};
pub use session::Session;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresAccountRepository {
    pub(crate) pool: PgPool,
}
impl PostgresAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

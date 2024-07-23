use async_trait::async_trait;

use crate::{domain::entities::user::User, presentation::handlers::user_handler::NewUser};

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Option<User>;

    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error>;
}

use crate::{
    domain::{entities::user::User, repositories::user_repository::UserRepository},
    presentation::handlers::user_handler::NewUser,
};

#[derive(Clone)]
pub struct UserService<T: UserRepository> {
    user_repo: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(user_repo: T) -> Self {
        UserService { user_repo }
    }

    pub async fn register_user(&self, user: &NewUser) -> Result<(), diesel::result::Error> {
        self.user_repo.save(user).await?;

        Ok(())
    }

    pub async fn get_by_email(&self, email: &str) -> Option<User> {
        self.user_repo.find_by_email(email).await
    }
}

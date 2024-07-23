use std::sync::Arc;

use async_trait::async_trait;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::{
    domain::{entities::user::User, repositories::user_repository::UserRepository},
    infrastructure::db::connection::{establish_connection, DBPool},
    presentation::handlers::user_handler::NewUser,
    schema::{self},
};

pub struct PostgresUserRepository {
    pool: DBPool,
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        PostgresUserRepository {
            pool: establish_connection(&database_url),
        }
    }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
    async fn find_by_email(&self, input_email: &str) -> Option<User> {
        use schema::users::dsl::*;

        users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading user")
    }

    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error> {
        diesel::insert_into(schema::users::table)
            .values(user)
            .execute(&mut self.pool.get().unwrap())
            .unwrap();

        Ok(())
    }
}

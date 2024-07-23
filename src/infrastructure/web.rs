use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;

use crate::presentation::routers;

use super::repositories::postgres_user_repositorv::PostgresUserRepository;

pub async fn run() -> std::io::Result<()> {
    let repo = PostgresUserRepository::new();
    let app_data = web::Data::new(repo);

    info!("Web server Starting...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .configure(routers::user_routes::routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

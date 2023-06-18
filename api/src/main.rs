mod film_repository;
mod health;
mod v1;

use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_aws_rds::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // create a film repository. In this case for postgres.
    let film_repository = film_repository::PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);

    // start the service
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(film_repository)
            .configure(health::service)
            .configure(v1::service::<film_repository::PostgresFilmRepository>);
    };

    Ok(config.into())
}

mod config;
mod db;
mod errors;
mod handler;
mod models;

use crate::config::Config;
use actix_rt;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use slog::info;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set up the configuration
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let log = config.configure_log();
    let pool = config.configure_pool();
    let tera = config.configure_tera();
    info!(
        log,
        "Starting server at http://{}:{}/", config.server.host, config.server.port
    );

    // Launch the app
    HttpServer::new(move || {
        App::new()
            .data(models::AppState {
                pool: pool.clone(),
                log: log.clone(),
                tera: tera.clone(),
            })
            .service(handler::add_experiment)
            .service(handler::get_experiments)
            .service(handler::get_experiment_by_author)
            .service(handler::add_granule)
            .service(handler::get_granules)
            .service(handler::mark_granule_valid)
            .service(handler::status)
    })
    .keep_alive(10)
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration")]
mod integration_tests;

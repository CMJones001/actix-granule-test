mod config;
mod db;
mod errors;
mod handler;
mod models;

use actix_rt;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use slog::{info, o, Drain, Logger};
use slog_async;
use slog_term;
use tokio_postgres::NoTls;

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set up the configuration
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let log = configure_log();
    info!(
        log,
        "Starting server at http://{}:{}/", config.server.host, config.server.port
    );

    let pool = config.pg.create_pool(NoTls).unwrap();

    // Launch the app
    HttpServer::new(move || {
        App::new()
            .data(models::AppState {
                pool: pool.clone(),
                log: log.clone(),
            })
            .service(handler::get_experiments)
            .service(handler::add_experiment)
            .service(handler::get_experiment_by_author)
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
mod integratiotests {
    use super::*;
    use actix_web::test;
    use models::AppState;
    use serde_json;

    #[actix_rt::test]
    async fn test_response() {
        dotenv().ok();
        let config = crate::config::Config::from_env().unwrap();
        let log = configure_log();
        let pool = config.pg.create_pool(NoTls).unwrap();

        let app_state = models::AppState { pool, log };
        let app = App::new().data(app_state).service(handler::get_experiments);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/exp/").to_request();
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 200, "GET /Exp should return 200");

        let body = test::read_body(response).await;

        let try_experiments: Result<Vec<models::Experiment>, serde_json::error::Error> =
            serde_json::from_slice(&body);

        assert!(try_experiments.is_ok(), "Response could not be parsed");
    }
}

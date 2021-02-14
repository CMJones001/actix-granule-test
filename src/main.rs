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
use slog_async;
use slog_term;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Set up the configuration
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let log = config.configure_log();
    let pool = config.configure_pool();
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

    use lazy_static::lazy_static;

    lazy_static! {
        static ref APP_STATE: AppState = {
            dotenv().ok();
            let config = Config::from_env().unwrap();
            let log = config.configure_log();
            let pool = config.configure_pool();

            models::AppState { pool, log }
        };
    }

    #[actix_rt::test]
    async fn test_experiment_response() {
        let app = App::new()
            .data(APP_STATE.clone())
            .service(handler::get_experiments);
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/exp/").to_request();
        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(), 200, "GET /exp should return 200");

        let body = test::read_body(response).await;

        let try_experiments: Result<Vec<models::Experiment>, serde_json::error::Error> =
            serde_json::from_slice(&body);

        assert!(try_experiments.is_ok(), "Response could not be parsed");
    }

    #[actix_rt::test]
    async fn test_response() {
        // Start app instance
        let app = App::new()
            .data(APP_STATE.clone())
            .service(handler::get_experiments)
            .service(handler::add_experiment);
        let mut app = test::init_service(app).await;

        // Create a request to create a new experiment
        let title = "New Experiment".to_string();
        let author = "Test Author".to_string();
        let new_experiment = models::CreateExperiment { title, author };
        let new_experiment_expected = serde_json::to_string(&new_experiment).unwrap();

        // Send this new request
        let req = test::TestRequest::post()
            .uri("/exp/")
            .header("Content-Type", "application/json")
            .set_payload(new_experiment_expected.clone())
            .to_request();
        let response = test::call_service(&mut app, req).await;

        // Can we parse the response?
        assert_eq!(response.status(), 200, "POST /exp should return 200");
        let body = test::read_body(response).await;
        let try_new_experiment = serde_json::from_slice::<models::Experiment>(&body);
        assert!(
            try_new_experiment.is_ok(),
            "Unable to unpack returned experiment"
        );
        let new_experiment = try_new_experiment.unwrap();

        let req = test::TestRequest::get().uri("/exp/").to_request();
        let experiments: Vec<models::Experiment> = test::read_response_json(&mut app, req).await;

        let try_experiment = experiments
            .iter()
            .find(|experiment| experiment.id == new_experiment.id);

        assert!(
            try_experiment.is_some(),
            "Unable to find created experiment"
        )
    }
}

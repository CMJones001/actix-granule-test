mod config;
mod db;
mod errors;
mod handler;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the configuration
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    println!(
        "Starting server at http://{}:{}/",
        config.server.host, config.server.port
    );

    let pool = config.pg.create_pool(NoTls).unwrap();

    // Launch the app
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handler::status)
            .service(handler::get_experiments)
            .service(handler::add_experiment)
            .service(handler::get_experiment_by_author)
            .service(handler::get_granules)
            .service(handler::mark_granule_valid)
    })
    .keep_alive(10)
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

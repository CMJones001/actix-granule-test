//! Manange the incoming requests
//!
//! These functions are called by the server when a GET/PUT/POST request are sent

use crate::db;
use crate::models::Status;
use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body(r#"{"status":"up"}"#)
}

#[get("/exp{_:/?}")]
pub async fn get_experiments(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::get_experiments(&client).await;

    if let Ok(experiments) = result {
        HttpResponse::Ok().json(experiments)
    } else {
        HttpResponse::InternalServerError().into()
    }
}

#[get("/exp/{experiment_id}/granules")]
pub async fn get_granules(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // Unpack the experiment_Name variable
    let web::Path((experiment_name,)) = path;
    let result = db::get_granules(&client, experiment_name).await;

    if let Ok(experiments) = result {
        HttpResponse::Ok().json(experiments)
    } else {
        HttpResponse::InternalServerError().into()
    }
}

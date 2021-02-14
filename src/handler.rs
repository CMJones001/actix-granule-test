//! Manange the incoming requests
//!
//! These functions are called by the server when a GET/PUT/POST request are sent

use crate::db;
use crate::models::*;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use std::io::ErrorKind;

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
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

#[get("/exp/{experiment_id}/granules{_:/?}")]
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

#[put("/exp/{experiment_id}/granules/{granule_id}{_:/?}")]
pub async fn mark_granule_valid(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // Unpack the experiment_Name variable
    let web::Path((experiment_id, granule_id)) = path;
    let result = db::mark_granule_valid(&client, experiment_id, granule_id).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse { success: true }),
        Err(ref e) if e.kind() == ErrorKind::Other => {
            HttpResponse::Ok().json(ResultResponse { success: false })
        }
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/exp/author/{author_name}{_:/?}")]
pub async fn get_experiment_by_author(
    db_pool: web::Data<Pool>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // Unpack the experiment_Name variable
    let web::Path((author_name,)) = path;
    println!("author_name = {}", author_name);
    let result = db::get_authors_experiment(&client, author_name).await;

    if let Ok(experiments) = result {
        HttpResponse::Ok().json(experiments)
    } else {
        HttpResponse::InternalServerError().into()
    }
}

#[post("/exp{_:/?}")]
pub async fn add_experiment(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateExperiment>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let CreateExperiment { title, author } = json.into_inner();
    let result = db::create_experiment(&client, title, author).await;

    match result {
        Ok(experiment) => HttpResponse::Ok().json(experiment),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

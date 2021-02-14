//! Manange the incoming requests
//!
//! These functions are called by the server when a GET/PUT/POST request are sent

use crate::db;
use crate::errors::AppError;
use crate::models::*;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[get("/exp{_:/?}")]
pub async fn get_experiments(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError::db_error(err))?;

    let result = db::get_experiments(&client).await;

    result.map(|experiment| HttpResponse::Ok().json(experiment))
}

#[get("/exp/{experiment_id}/granules{_:/?}")]
pub async fn get_granules(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError::db_error(err))?;

    // Unpack the experiment_Name variable
    let web::Path((experiment_name,)) = path;
    let result = db::get_granules(&client, experiment_name).await;

    result.map(|granule| HttpResponse::Ok().json(granule))
}

#[put("/exp/{experiment_id}/granules/{granule_id}{_:/?}")]
pub async fn mark_granule_valid(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError::db_error(err))?;

    // Unpack the variables from the path/url
    let web::Path((experiment_id, granule_id)) = path;
    let result = db::mark_granule_valid(&client, experiment_id, granule_id).await;

    result.map(|updated: bool| HttpResponse::Ok().json(ResultResponse { success: updated }))
}

#[get("/exp/author/{author_name}{_:/?}")]
pub async fn get_experiment_by_author(
    db_pool: web::Data<Pool>,
    path: web::Path<(String,)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError::db_error(err))?;
    // Unpack the variables from the path/url
    let web::Path((author_name,)) = path;
    let result = db::get_authors_experiment(&client, author_name).await;
    result.map(|granule| HttpResponse::Ok().json(granule))
}

#[post("/exp{_:/?}")]
pub async fn add_experiment(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateExperiment>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(|err| AppError::db_error(err))?;

    let CreateExperiment { title, author } = json.into_inner();
    let result = db::create_experiment(&client, title, author).await;

    result.map(|experiment| HttpResponse::Ok().json(experiment))
}

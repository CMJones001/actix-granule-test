//! Manange the incoming requests
//!
//! These functions are called by the server when a GET/PUT/POST request are sent

use crate::db;
use crate::errors::AppError;
use crate::models::*;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use serde::Serialize;
use slog::{crit, error, o, Logger};

pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let sublog = log.new(o!("cause"=>err.cause.clone()));
        error!(sublog, "{}", err.message());
        err
    })
}

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error creating client");
        AppError::db_error(err)
    })
}

// Return json or raise an error
pub fn json_or_err<T: Serialize>(
    res: Result<T, AppError>,
    log: Logger,
) -> Result<impl Responder, AppError> {
    res.map(|val| HttpResponse::Ok().json(val))
        .map_err(log_error(log))
}

#[get("/")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[get("/exp{_:/?}")]
pub async fn get_experiments(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_experiments"));
    let client = get_client(state.pool.clone(), log.clone()).await?;
    let result = db::get_experiments(&client).await;

    json_or_err(result, log)
}

#[get("/exp/{experiment_id}/granules{_:/?}")]
pub async fn get_granules(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_granules"));
    let client = get_client(state.pool.clone(), log.clone()).await?;

    // Unpack the experiment_Name variable
    let web::Path((experiment_name,)) = path;
    let result = db::get_granules(&client, experiment_name).await;

    json_or_err(result, log)
}

#[put("/exp/{experiment_id}/granules/{granule_id}{_:/?}")]
pub async fn mark_granule_valid(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "mark_granule_valid"));
    let client = get_client(state.pool.clone(), log.clone()).await?;

    // Unpack the variables from the path/url
    let web::Path((experiment_id, granule_id)) = path;
    let result = db::mark_granule_valid(&client, experiment_id, granule_id).await;

    result.map(|updated: bool| HttpResponse::Ok().json(ResultResponse { success: updated }))
}

#[get("/exp/author/{author_name}{_:/?}")]
pub async fn get_experiment_by_author(
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_experiment_by_author"));
    let client = get_client(state.pool.clone(), log.clone()).await?;

    // Unpack the variables from the path/url
    let web::Path((author_name,)) = path;
    let result = db::get_authors_experiment(&client, author_name).await;
    json_or_err(result, log)
}

#[post("/exp{_:/?}")]
pub async fn add_experiment(
    state: web::Data<AppState>,
    json: web::Json<CreateExperiment>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "add_experiment"));
    let client = get_client(state.pool.clone(), log.clone()).await?;

    let CreateExperiment { title, author } = json.into_inner();
    let result = db::create_experiment(&client, title, author).await;
    json_or_err(result, log)
}

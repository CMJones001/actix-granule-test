//! Models for the data structures within the database

use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "granules")]
pub struct Experiment {
    pub id: i32,
    pub title: String,
    pub author: String,
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "granules")]
pub struct Granule {
    pub id: i32,
    pub valid: bool,
    pub area: f32,
    pub experiment_id: i32,
}

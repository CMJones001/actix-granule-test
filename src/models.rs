//! Models for the data structures within the database

use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use slog::Logger;
use tera::Tera;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: Logger,
    pub tera: Tera,
}

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

#[derive(Deserialize, Serialize)]
pub struct CreateExperiment {
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

#[derive(Deserialize, Serialize, PostgresMapper, Debug)]
#[pg_mapper(table = "granules")]
pub struct CreateGranule {
    pub valid: bool,
    pub area: f32,
}

#[derive(Deserialize, Serialize)]
pub struct ResultResponse {
    pub success: bool,
}

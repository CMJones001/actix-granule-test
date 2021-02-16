//! Handle the gathering of data from the postgres database
use crate::errors::{AppError, AppErrorType};
use crate::models::{CreateGranule, Experiment, Granule};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_experiments(client: &Client) -> Result<Vec<Experiment>, AppError> {
    let statement = client
        .prepare("select * from experiment order by id desc")
        .await
        .map_err(|err| AppError::db_error(err))?;
    let experiment = client
        .query(&statement, &[])
        .await
        .expect("Error getting experiment")
        .iter()
        .map(|row| Experiment::from_row_ref(row).expect("Unable to unwrap experiment"))
        .collect::<Vec<Experiment>>();

    Ok(experiment)
}

pub async fn get_granules(client: &Client, experiment_id: i32) -> Result<Vec<Granule>, AppError> {
    let statement = client
        .prepare("select * from granule where experiment_id = $1 order by id")
        .await
        .map_err(|err| AppError::db_error(err))?;

    let granule = client
        .query(&statement, &[&experiment_id])
        .await
        .map_err(|err| AppError::db_error(err))?
        .iter()
        .map(|row| Granule::from_row_ref(row).expect("Unable to unwrap granule"))
        .collect::<Vec<Granule>>();

    Ok(granule)
}

pub async fn create_experiment(
    client: &Client,
    title: String,
    author: String,
) -> Result<Experiment, AppError> {
    let statement = client
        .prepare(
            "insert into experiment (title, author) values ($1, $2) returning id, title, author",
        )
        .await
        .map_err(|err| AppError::db_error(err))?;

    let experiment = client
        .query(&statement, &[&title, &author])
        .await
        .map_err(|err| AppError::db_error(err))?
        .iter()
        .map(|row| Experiment::from_row_ref(row).unwrap())
        .collect::<Vec<Experiment>>()
        .pop()
        .ok_or(AppError {
            message: Some("Unable to make create experiment".to_string()),
            cause: None,
            error_type: AppErrorType::DbError,
        })?;

    Ok(experiment)
}

pub async fn create_granule(
    client: &Client,
    granule_cmd: CreateGranule,
    experiment_id: i32,
) -> Result<Granule, AppError> {
    let CreateGranule { valid, area } = granule_cmd;
    let statement = client
        .prepare(
            "insert into granule (valid, area, experiment_id) values ($1, $2, $3) returning id, valid, area, experiment_id",
        )
        .await
        .map_err(|err| AppError::db_error(err))?;

    let granule = client
        .query(&statement, &[&valid, &area, &experiment_id])
        .await
        .map_err(|err| AppError::db_error(err))?
        .iter()
        .map(|row| Granule::from_row_ref(row).unwrap())
        .collect::<Vec<Granule>>()
        .pop()
        .ok_or(AppError {
            message: Some("Unable to add granule".to_string()),
            cause: None,
            error_type: AppErrorType::DbError,
        })?;

    Ok(granule)
}

pub async fn mark_granule_valid(
    client: &Client,
    experiment_id: i32,
    granule_id: i32,
) -> Result<bool, AppError> {
    let query =
        "update granule set valid = true where experiment_id = $1 and id = $2 and valid = false";
    let statement = client
        .prepare(query)
        .await
        .map_err(|err| AppError::db_error(err))?;

    let result = client
        .execute(&statement, &[&experiment_id, &granule_id])
        .await
        .map_err(|err| AppError::db_error(err))?;

    Ok(result == 1)
}

pub async fn get_authors_experiment(
    client: &Client,
    author: String,
) -> Result<Vec<Experiment>, AppError> {
    let statement = client
        .prepare("select * from experiment where lower(author) = lower($1) order by id")
        .await
        .map_err(|err| AppError::db_error(err))?;

    let experiments = client
        .query(&statement, &[&author])
        .await
        .map_err(|err| AppError::db_error(err))?
        .iter()
        .map(|row| Experiment::from_row_ref(row).expect("Unable to unwrap experiments"))
        .collect::<Vec<Experiment>>();

    Ok(experiments)
}

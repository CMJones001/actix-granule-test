//! Handle the gathering of data from the postgres database
use crate::models::{Experiment, Granule};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_experiments(client: &Client) -> Result<Vec<Experiment>, io::Error> {
    let statement = client
        .prepare("select * from experiment order by id desc")
        .await
        .unwrap();
    let experiment = client
        .query(&statement, &[])
        .await
        .expect("Error getting experiment")
        .iter()
        .map(|row| Experiment::from_row_ref(row).expect("Unable to unwrap experiment"))
        .collect::<Vec<Experiment>>();

    Ok(experiment)
}

pub async fn get_granules(client: &Client, experiment_id: i32) -> Result<Vec<Granule>, io::Error> {
    let statement = client
        .prepare("select * from granule where experiment_id = $1 order by id")
        .await
        .expect("Unable to make get_granule request");

    let granule = client
        .query(&statement, &[&experiment_id])
        .await
        .expect("Error getting granule")
        .iter()
        .map(|row| Granule::from_row_ref(row).expect("Unable to unwrap granule"))
        .collect::<Vec<Granule>>();

    Ok(granule)
}

pub async fn create_experiment(
    client: &Client,
    title: String,
    author: String,
) -> Result<Experiment, io::Error> {
    let statement = client
        .prepare(
            "insert into experiment (title, author) values ($1, $2) returning id, title, author",
        )
        .await
        .expect("Unable to make create_experiment request");

    client
        .query(&statement, &[&title, &author])
        .await
        .expect("Error creating experiment")
        .iter()
        .map(|row| Experiment::from_row_ref(row).unwrap())
        .collect::<Vec<Experiment>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating todo list",
        ))
}

pub async fn mark_granule_valid(
    client: &Client,
    experiment_id: i32,
    granule_id: i32,
) -> Result<(), io::Error> {
    let statement = client
        .prepare(
            "update granule set valid = true where experiment_id = $1 and id = $2 and valid = false",
        )
        .await
        .expect("Unable to make mark_granule_valid request");

    let result = client
        .execute(&statement, &[&experiment_id, &granule_id])
        .await
        .expect("Error making granule valid.");

    if result == 1 {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to make granule valid",
        ))
    }
    // match result {
    //     ref updated if *updated == 1 => Ok(()),
    //     _ => Err(io::Error::new(
    //         io::ErrorKind::Other,
    //         "Failed to make granule valid",
    //     )),
    // }
}

pub async fn get_authors_experiment(
    client: &Client,
    author: String,
) -> Result<Vec<Experiment>, io::Error> {
    let statement = client
        .prepare("select * from experiment where lower(author) = lower($1) order by id")
        .await
        .expect("Unable to make get_authors_experiment request");

    let experiments = client
        .query(&statement, &[&author])
        .await
        .expect("Error getting experiments")
        .iter()
        .map(|row| Experiment::from_row_ref(row).expect("Unable to unwrap experiments"))
        .collect::<Vec<Experiment>>();

    Ok(experiments)
}

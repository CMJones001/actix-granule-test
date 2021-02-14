//! Handle the gathering of data from the postgres database
use crate::models::{Experiment, Granule};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_experiments(client: &Client) -> Result<Vec<Experiment>, std::io::Error> {
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

pub async fn get_granules(
    client: &Client,
    experiment_id: i32,
) -> Result<Vec<Granule>, std::io::Error> {
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

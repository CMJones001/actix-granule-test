use super::*;
use actix_web::test;
use models::AppState;
use serde_json;
use tera::Tera;

use lazy_static::lazy_static;

lazy_static! {
    static ref APP_STATE: AppState = {
        dotenv().ok();
        let config = Config::from_env().unwrap();
        let log = config.configure_log();
        let pool = config.configure_pool();
        let tera = config.configure_tera();
        models::AppState { pool, log, tera }
    };
}

#[actix_rt::test]
async fn test_experiment_response() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(handler::get_experiments);
    let mut app = test::init_service(app).await;

    let req = test::TestRequest::get().uri("/exp/").to_request();
    let response = test::call_service(&mut app, req).await;

    assert_eq!(response.status(), 200, "GET /exp should return 200");

    let body = test::read_body(response).await;

    let try_experiments: Result<Vec<models::Experiment>, serde_json::error::Error> =
        serde_json::from_slice(&body);

    assert!(try_experiments.is_ok(), "Response could not be parsed");
}

#[actix_rt::test]
async fn test_response() {
    // Start app instance
    let app = App::new()
        .data(APP_STATE.clone())
        .service(handler::get_experiments)
        .service(handler::add_experiment);
    let mut app = test::init_service(app).await;

    // Create a request to create a new experiment
    let title = "New Experiment".to_string();
    let author = "Test Author".to_string();
    let new_experiment = models::CreateExperiment { title, author };
    let new_experiment_expected = serde_json::to_string(&new_experiment).unwrap();

    // Send this new request
    let req = test::TestRequest::post()
        .uri("/exp/")
        .header("Content-Type", "application/json")
        .set_payload(new_experiment_expected.clone())
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Can we parse the response?
    assert_eq!(response.status(), 200, "POST /exp should return 200");
    let body = test::read_body(response).await;
    let try_new_experiment = serde_json::from_slice::<models::Experiment>(&body);
    assert!(
        try_new_experiment.is_ok(),
        "Unable to unpack returned experiment"
    );
    let new_experiment = try_new_experiment.unwrap();

    // Does this experiment exist in the table
    let req = test::TestRequest::get().uri("/exp/").to_request();
    let experiments: Vec<models::Experiment> = test::read_response_json(&mut app, req).await;
    let try_experiment = experiments
        .iter()
        .find(|experiment| experiment.id == new_experiment.id);

    assert!(
        try_experiment.is_some(),
        "Unable to find created experiment"
    )
}

#[actix_rt::test]
async fn test_granule_add() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(handler::add_experiment)
        .service(handler::add_granule)
        .service(handler::get_granules);
    let mut app = test::init_service(app).await;

    // First we have to create a new experiment
    let title = "New Experiment".to_string();
    let author = "Test Author".to_string();
    let new_experiment = models::CreateExperiment { title, author };
    let new_experiment_json = serde_json::to_string(&new_experiment).unwrap();
    let req = test::TestRequest::post()
        .uri("/exp/")
        .header("Content-Type", "application/json")
        .set_payload(new_experiment_json.clone())
        .to_request();
    let response = test::call_service(&mut app, req).await;
    let body = test::read_body(response).await;
    let new_experiment = serde_json::from_slice::<models::Experiment>(&body).expect("Unable to create experiment in create granule test");
    let experiment_id = new_experiment.id;

    // Use this to create a granule
    let new_granule = models::CreateGranule {valid:false, area:1.0};
    let new_granule_json = serde_json::to_string(&new_granule).unwrap();
    let uri = format!("/exp/{}/granules", experiment_id);
    let req = test::TestRequest::post()
        .uri(&uri)
        .header("Content-Type", "application/json")
        .set_payload(new_granule_json.clone())
        .to_request();

    let response = test::call_service(&mut app, req).await;

    // Can we parse the response?
    assert_eq!(response.status(), 200, "POST {} should return 200", uri);
    let body = test::read_body(response).await;
    let try_new_granule = serde_json::from_slice::<models::Granule>(&body);
    assert!(
        try_new_granule.is_ok(),
        "Unable to unpack returned granule"
    );
    let granule_id = try_new_granule.unwrap().id;

    // Does this granule then exist in the table?
    let req = test::TestRequest::get().uri(&uri).to_request();
    let response = test::call_service(&mut app, req).await;
    assert_eq!(response.status(), 200, "GET {} should return 200", uri);
    let body = test::read_body(response).await;

    let granules: Vec<models::Granule> = serde_json::from_slice(&body).expect("Unable to parse granule list");
    let try_granule = granules.iter().find(|granule| granule.id == granule.id);
    assert!(
        try_granule.is_some(),
        "Unable to find created granule"
    )
}

#[actix_rt::test]
async fn test_granule_mark() {
    let app = App::new()
        .data(APP_STATE.clone())
        .service(handler::add_experiment)
        .service(handler::add_granule)
        .service(handler::get_granules)
        .service(handler::mark_granule_valid);

    let mut app = test::init_service(app).await;

    // First we have to create a new experiment
    let title = "New Experiment".to_string();
    let author = "Test Author".to_string();
    let new_experiment = models::CreateExperiment { title, author };
    let new_experiment_json = serde_json::to_string(&new_experiment).unwrap();
    let req = test::TestRequest::post()
        .uri("/exp/")
        .header("Content-Type", "application/json")
        .set_payload(new_experiment_json.clone())
        .to_request();
    let response = test::call_service(&mut app, req).await;
    let body = test::read_body(response).await;
    let new_experiment = serde_json::from_slice::<models::Experiment>(&body).expect("Unable to create experiment in create granule test");
    let experiment_id = new_experiment.id;

    // Use this to create a granule
    let new_granule = models::CreateGranule {valid:false, area:1.0};
    let new_granule_json = serde_json::to_string(&new_granule).unwrap();
    let uri = format!("/exp/{}/granules", experiment_id);
    let req = test::TestRequest::post()
        .uri(&uri)
        .header("Content-Type", "application/json")
        .set_payload(new_granule_json.clone())
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Get the ID of this new granule
    assert_eq!(response.status(), 200, "POST {} should return 200", uri);
    let body = test::read_body(response).await;
    let try_new_granule = serde_json::from_slice::<models::Granule>(&body);
    assert!(
        try_new_granule.is_ok(),
        "Unable to unpack returned granule"
    );
    let granule_id = try_new_granule.unwrap().id;

    // Mark this granule
    let mark_uri = format!("/exp/{}/granules/{}", experiment_id, granule_id);
    let req = test::TestRequest::put().uri(&mark_uri).to_request();
    let response = test::call_service(&mut app, req).await;
    assert_eq!(response.status(), 200, "GET {} should return 200", mark_uri);
    let body = test::read_body(response).await;
    let models::ResultResponse{success} = serde_json::from_slice(&body).unwrap();

    assert!(success, "First marking of granule should succeed");

    // Attemp to mark this granule again
    let req = test::TestRequest::put().uri(&mark_uri).to_request();
    let response = test::call_service(&mut app, req).await;
    assert_eq!(response.status(), 200, "GET {} should return 200", mark_uri);
    let body = test::read_body(response).await;
    let models::ResultResponse{success} = serde_json::from_slice(&body).unwrap();

    assert!(!success, "Second marking of granule should fail");
}

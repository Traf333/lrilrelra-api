use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use db::{connect_db, DB};
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use surrealdb::sql::Thing;

mod db;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Scenario {
    pub id: Thing,
    pub title: String,
    pub author: String,
    pub content: String,
    pub actors_number: u32,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateScenario {
    title: String,
    author: String,
    content: String,
    actors_number: u32,
}

#[derive(Deserialize, Serialize)]
struct DeleteScenario {
    id: String,
}

async fn get_scenarios() -> (StatusCode, Json<Vec<Scenario>>) {
    let records = DB.select("scenario").await.unwrap();
    (StatusCode::OK, Json(records))
}

async fn create_scenario(
    Json(payload): Json<CreateScenario>,
) -> (StatusCode, Json<Option<Scenario>>) {
    let record: Option<Scenario> = DB.create("scenario").content(payload).await.unwrap();
    match record {
        Some(record) => (StatusCode::CREATED, Json(Some(record))),
        None => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

async fn delete_scenario(Path(id): Path<String>) -> impl IntoResponse {
    let _: Scenario = DB.delete(("scenario", id)).await.unwrap().unwrap();

    StatusCode::OK
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    connect_db(secrets)
        .await
        .expect("Database connection fails");

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/scenarios", get(get_scenarios))
        .route("/scenarios", post(create_scenario))
        .route("/scenarios/:id", delete(delete_scenario));

    Ok(router.into())
}

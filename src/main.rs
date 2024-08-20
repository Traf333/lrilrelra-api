use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use config::Settings;
use db::{connect_db, DB};
use serde::{Deserialize, Serialize};

mod config;
mod db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scenario {
    pub id: String,
    pub title: String,
    pub content: String,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize, Serialize)]
struct CreateScenario {
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize)]
struct UpdateScenario {
    id: String,
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize)]
struct DeleteScenario {
    id: String,
}

async fn create_scenario(Json(payload): Json<CreateScenario>) -> (StatusCode, Json<Vec<Scenario>>) {
    let record = DB.create("scenario").content(payload).await.unwrap();
    (StatusCode::CREATED, Json(record))
}

async fn delete_scenario(Path(id): Path<String>) -> impl IntoResponse {
    let _: Scenario = DB.delete(("scenario", id)).await.unwrap().unwrap();

    StatusCode::OK
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let settings = Settings::new().expect("Failed to load config");

    connect_db(settings.database).await.unwrap();

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/scenarios", post(create_scenario))
        .route("/scenarios/:id", delete(delete_scenario));

    Ok(router.into())
}

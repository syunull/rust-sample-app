use axum::response::IntoResponse;
use axum::{Extension, Json};

use crate::Person;

pub async fn list_persons(Extension(database): Extension<sled::Db>) -> impl IntoResponse {
    let people: Vec<Person> = database
        .iter()
        .map(|res| Person::try_from(res.unwrap()).unwrap())
        .collect();
    Json(people)
}

#[tracing::instrument(name = "creating a new person", skip(person, database))]
pub async fn create_person(
    Json(person): Json<Person>,
    Extension(database): Extension<sled::Db>,
) -> impl IntoResponse {
    match database.insert(person.firstname.as_bytes(), person.lastname.as_bytes()) {
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
        _ => http::StatusCode::OK,
    }
}

#[tracing::instrument]
pub async fn fast() -> impl IntoResponse {
    http::StatusCode::OK
}

#[tracing::instrument]
pub async fn slow() -> impl IntoResponse {
    sleep_seconds(5).await;
}

#[tracing::instrument(name = "sleeping")]
pub async fn sleep_seconds(seconds: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(seconds)).await;
}

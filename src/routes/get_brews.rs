use lambda_http::{http::StatusCode, Response};

use crate::repository::Repository;

pub async fn handler(repository: &Repository) -> Response<String> {
    let brews = repository.get_brews().await;

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&brews).unwrap())
        .unwrap()
}

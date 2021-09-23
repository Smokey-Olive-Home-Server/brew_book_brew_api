use lambda_http::{http::StatusCode, Body, Request, Response};
use serde_json;

use crate::repository::{Brew, Repository};

pub async fn handler(request: Request, repository: &Repository) -> Response<String> {
    match request.body() {
        Body::Text(string) => {
            log::info!("{}", string);

            let brew: Result<Brew, serde_json::Error> = serde_json::from_str(string);

            match brew {
                Ok(value) => {
                    log::info!("Got the values");

                    repository.post_brew(&value).await;

                    Response::builder()
                        .status(StatusCode::OK)
                        .body(serde_json::to_string(&value).unwrap())
                        .unwrap()
                }
                _ => not_accepted(),
            }
        }
        _ => not_accepted(),
    }
}

pub fn not_accepted() -> Response<String> {
    Response::builder()
        .status(StatusCode::NOT_ACCEPTABLE)
        .body(String::new())
        .unwrap()
}

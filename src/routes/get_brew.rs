use lambda_http::{http::StatusCode, Request, RequestExt, Response};
use serde::Serialize;

use crate::repository::Repository;

#[derive(Serialize)]
struct Brew {
    id: String,
    brew_title: String,
}

pub async fn handler(request: Request, repository: &Repository) -> Response<String> {
    let query_string = request.query_string_parameters();
    let brew_id = query_string.get("brew_id");

    let brew_id = match brew_id {
        Some(brew_id) => brew_id,
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(String::new())
                .unwrap()
        }
    };

    let brew = repository.get_brew(brew_id).await;

    match brew {
        Some(brew) => Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&brew).unwrap())
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(String::new())
            .unwrap(),
    }
}

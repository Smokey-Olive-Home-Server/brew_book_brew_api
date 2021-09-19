use aws_config;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_http::{http::StatusCode, lambda_runtime::Error, Body, Context, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Brew {
    id: String,
    brew_title: String,
}

pub async fn handler(request: Request, _: Context) -> Result<Response<String>, Error> {
    match request.body() {
        Body::Text(string) => {
            log::info!("{}", string);

            let brew: Result<Brew, serde_json::Error> = serde_json::from_str(string);

            match brew {
                Ok(value) => {
                    log::info!("Got the values");

                    let results = save_in_db(&value).await;

                    Ok(results)
                }
                _ => Ok(not_accepted()),
            }
        }
        _ => Ok(not_accepted()),
    }
}

pub async fn save_in_db(brew: &Brew) -> Response<String> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let response = client
        .put_item()
        .table_name("brews")
        .item("id", AttributeValue::S(brew.id.to_string()))
        .item("brew_title", AttributeValue::S(brew.brew_title.to_string()));

    let results = response.send().await;

    match results {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(&brew).unwrap())
            .unwrap(),
        Err(_) => not_accepted(),
    }
}

pub fn not_accepted() -> Response<String> {
    Response::builder()
        .status(StatusCode::NOT_ACCEPTABLE)
        .body(String::new())
        .unwrap()
}

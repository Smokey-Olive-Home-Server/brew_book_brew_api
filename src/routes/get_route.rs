use aws_config;
use aws_sdk_dynamodb::{
    model::{AttributeValue, Select},
    Client,
};
use lambda_http::{
    http::StatusCode, lambda_runtime::Error, Context, Request, RequestExt, Response,
};
use serde::Serialize;

#[derive(Serialize)]
struct Brew {
    id: String,
    brew_title: String,
}

pub async fn handler(request: Request, _: Context) -> Result<Response<String>, Error> {
    let query_string = request.query_string_parameters();
    let brew = query_string.get("brew_id");

    match brew {
        Some(brew_id) => {
            log::info!("Got successful get request for id {}", brew_id);

            let result = get_brew(brew_id).await;

            Ok(result)
        }
        None => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(String::new())
            .unwrap()),
    }
}

async fn get_brew(brew_id: &str) -> Response<String> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let response = client
        .query()
        .table_name("brews")
        .key_condition_expression("#key = :value")
        .expression_attribute_names("#key", "id")
        .expression_attribute_values(":value", AttributeValue::S(brew_id.to_string()))
        .limit(1)
        .select(Select::AllAttributes)
        .send()
        .await
        .unwrap();

    match response.items {
        Some(items) => {
            let optional_item = items.get(0);

            match optional_item {
                Some(item) => {
                    let brew = Brew {
                        id: if let AttributeValue::S(id) = item.get("id").unwrap() {
                            id.to_string()
                        } else {
                            "unknown".to_string()
                        },
                        brew_title: if let AttributeValue::S(brew_title) =
                            item.get("brew_title").unwrap()
                        {
                            brew_title.to_string()
                        } else {
                            "unknown".to_string()
                        },
                    };

                    Response::builder()
                        .status(StatusCode::OK)
                        .body(serde_json::to_string(&brew).unwrap())
                        .unwrap()
                }
                None => Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body(String::new())
                    .unwrap(),
            }
        }
        None => Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(String::new())
            .unwrap(),
    }
}

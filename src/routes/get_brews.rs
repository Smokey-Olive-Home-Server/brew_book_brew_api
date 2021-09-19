use aws_config;
use aws_sdk_dynamodb::{
    model::{AttributeValue, Select},
    Client,
};
use lambda_http::{http::StatusCode, lambda_runtime::Error, Context, Request, Response};
use serde::Serialize;

#[derive(Serialize)]
struct Brew {
    id: String,
    brew_title: String,
}

pub async fn handler(_request: Request, _: Context) -> Result<Response<String>, Error> {
    let result = get_brews().await;

    Ok(result)
}

async fn get_brews() -> Response<String> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let response = client
        .scan()
        .table_name("brews")
        .select(Select::AllAttributes)
        .send()
        .await
        .unwrap();

    match response.items {
        Some(items) => {
            let mut response_list = Vec::new();

            for item in items {
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

                response_list.push(brew);
            }

            Response::builder()
                .status(StatusCode::OK)
                .body(serde_json::to_string(&response_list).unwrap())
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::OK)
            .body(String::from("[]"))
            .unwrap(),
    }
}

use lambda_http::{http::StatusCode, lambda_runtime::Error, Body, Context, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Brew {
    name: String,
    brew_type: String,
}

pub fn handler(request: Request, _: Context) -> Result<Response<String>, Error> {
    match request.body() {
        Body::Text(string) => {
            log::info!("{}", string);

            let brew: Result<Brew, serde_json::Error> = serde_json::from_str(string);

            match brew {
                Ok(value) => {
                    log::info!("Got the values");

                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(serde_json::to_string(&value).unwrap())
                        .unwrap())
                }
                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_ACCEPTABLE)
                    .body(String::new())
                    .unwrap()),
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body(String::new())
            .unwrap()),
    }
}

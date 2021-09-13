use lambda_http::{
    handler,
    http::{Method, StatusCode},
    lambda_runtime::{self, Context, Error},
    Body, Request, RequestExt, Response,
};
use log;
use serde::{Deserialize, Serialize};
use serde_json;
use simple_logger::SimpleLogger;

type MethodResponses = Result<Response<String>, Error>;

#[derive(Serialize, Deserialize)]
struct Brew {
    name: String,
    brew_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

async fn handler_func(request: Request, context: Context) -> MethodResponses {
    let method = request.method();

    match *method {
        Method::GET => get_handler(request, context),
        Method::POST => post_handler(request, context),
        _ => unknown_method_handler(request, context),
    }
}

fn get_handler(request: Request, _: Context) -> MethodResponses {
    let query_string = request.query_string_parameters();
    let brew = query_string.get("brew_id");

    match brew {
        Some(brew_id) => {
            log::info!("Got successful get request for id {}", brew_id);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(String::from(brew_id))
                .unwrap())
        }
        None => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(String::new())
            .unwrap()),
    }
}

fn post_handler(request: Request, _: Context) -> MethodResponses {
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

fn unknown_method_handler(request: Request, _: Context) -> MethodResponses {
    log::warn!(
        "Unknown method request made to lambda. Details: {:?}",
        request
    );
    Ok(Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(String::new())
        .unwrap())
}

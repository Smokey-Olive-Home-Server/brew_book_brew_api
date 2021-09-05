use lambda_http::{
    handler,
    http::StatusCode,
    lambda_runtime::{self, Context, Error},
    Request, RequestExt, Response,
};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(request: Request, _: Context) -> Result<Response<String>, Error> {
    let query_string = request.query_string_parameters();

    log::info!("{:?}", query_string);

    let first_name = match query_string.get("firstName") {
        Some(name) => name,
        None => {
            let default_name = "Stranger";
            log::warn!(
                "User did not enter firstName defaulting to: {}",
                default_name
            );

            default_name
        }
    };

    let response = Response::builder().status(StatusCode::OK);

    Ok(response
        .body(format!("Hello, my name is {}!", first_name))
        .unwrap())
}

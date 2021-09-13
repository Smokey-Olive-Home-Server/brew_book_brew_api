use lambda_http::{http::StatusCode, lambda_runtime::Error, Context, Request, Response};
use log;

pub fn handler(request: Request, _: Context) -> Result<Response<String>, Error> {
    log::warn!(
        "Unknown method request made to lambda. Details: {:?}",
        request
    );
    Ok(Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(String::new())
        .unwrap())
}

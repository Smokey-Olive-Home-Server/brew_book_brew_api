use lambda_http::{http::StatusCode, Context, Request, Response};
use log;

pub async fn handler(request: Request, _: Context) -> Response<String> {
    log::warn!(
        "Unknown method request made to lambda. Details: {:?}",
        request
    );
    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(String::new())
        .unwrap()
}

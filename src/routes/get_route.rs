use lambda_http::{
    http::StatusCode, lambda_runtime::Error, Context, Request, RequestExt, Response,
};

pub fn handler(request: Request, _: Context) -> Result<Response<String>, Error> {
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

use lambda_http::{
    handler,
    http::Method,
    lambda_runtime::{self, Context, Error},
    Request, Response,
};
mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // SimpleLogger::new().init().unwrap();
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

async fn handler_func(request: Request, context: Context) -> Result<Response<String>, Error> {
    let method = request.method();

    log::info!("URI: {}", request.uri());
    log::info!("Extensions: {:?}", request.extensions());

    match *method {
        Method::GET => routes::get_route::handler(request, context).await,
        Method::POST => routes::post_route::handler(request, context).await,
        _ => routes::unknown_route::handler(request, context).await,
    }
}

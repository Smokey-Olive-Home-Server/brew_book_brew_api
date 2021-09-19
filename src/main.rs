use lambda_http::{
    handler,
    http::Method,
    lambda_runtime::{self, Context, Error},
    Request, Response,
};
use log;
use simplelog::{Config, LevelFilter, SimpleLogger};

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // SimpleLogger::new().init().unwrap();
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

async fn handler_func(request: Request, context: Context) -> Result<Response<String>, Error> {
    SimpleLogger::new(LevelFilter::Info, Config::default());
    let method = request.method();

    let path = request.uri().path();

    log::info!("The path: {}", path);

    match (method, path) {
        (&Method::GET, "/ApiGateway_stage/brew") => {
            routes::get_brew::handler(request, context).await
        }
        (&Method::POST, "/ApiGateway_stage/brew") => {
            routes::post_brew::handler(request, context).await
        }
        (&Method::GET, "/ApiGateway_stage/brews") => {
            routes::get_brews::handler(request, context).await
        }
        (_, _) => routes::unknown_route::handler(request, context).await,
    }
}

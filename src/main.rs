use lambda_http::{
    handler,
    http::Method,
    lambda_runtime::{self, Context, Error},
    Request, Response,
};
use lazy_static::lazy_static;
use log::Level;
use simple_logger;
mod repository;
use repository::Repository;
mod routes;

lazy_static! {
    static ref BREW_REPOSITORY: Repository = {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { Repository::new("brews").await })
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

async fn handler_func(
    request: Request,
    context: Context,
) -> Result<Response<String>, lambda_runtime::Error> {
    simple_logger::init_with_level(Level::Info).unwrap();

    log::info!("After log instance");

    log::info!("Log right after first");

    let method = request.method();

    let path = request.uri().path();

    log::info!("The path: {}", path);

    let response = match (method, path) {
        (&Method::GET, "/ApiGateway_stage/brew") => {
            routes::get_brew::handler(request, &BREW_REPOSITORY).await
        }
        (&Method::POST, "/ApiGateway_stage/brew") => {
            routes::post_brew::handler(request, &BREW_REPOSITORY).await
        }
        (&Method::GET, "/ApiGateway_stage/brews") => {
            routes::get_brews::handler(&BREW_REPOSITORY).await
        }
        (_, _) => routes::unknown_route::handler(request, context).await,
    };

    Ok(response)
}

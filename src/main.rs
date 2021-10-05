use lambda_http::{
    handler,
    http::Method,
    lambda_runtime::{self, Context, Error},
    Request, Response,
};
use log::Level;
use simple_logger;
mod repository;
use repository::Repository;
mod routes;
use tokio::sync::OnceCell;

static REPOSITORY: OnceCell<Repository> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(Level::Info).unwrap();
    lambda_runtime::run(handler(handler_func)).await?;
    Ok(())
}

async fn handler_func(
    request: Request,
    context: Context,
) -> Result<Response<String>, lambda_runtime::Error> {
    log::info!("After log instance");

    log::info!("Log right after first");

    let method = request.method();

    let path = request.uri().path();

    log::info!("The path: {}", path);

    let repository = REPOSITORY.get_or_init(|| Repository::new("brews")).await;

    let response = match (method, path) {
        (&Method::GET, "/ApiGateway_stage/brew") => {
            routes::get_brew::handler(request, repository).await
        }
        (&Method::POST, "/ApiGateway_stage/brew") => {
            routes::post_brew::handler(request, repository).await
        }
        (&Method::GET, "/ApiGateway_stage/brews") => routes::get_brews::handler(repository).await,
        (_, _) => routes::unknown_route::handler(request, context).await,
    };

    Ok(response)
}

use axum::{
    body::{Body, HttpBody},
    handler::Handler,
    http,
    response::Html,
    routing, BoxError, Router,
};

use hyper::StatusCode;
use tower::{util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;

use tracing_unwrap::ResultExt;

use crate::config::Config;

pub type Request = axum::http::Request<RequestBody>;
pub type RequestBody = axum::body::Body;
pub type Response = axum::response::Response;

async fn hello_handler(_req: Request) -> Html<&'static str> {
    Html("<h2>Hello World!</h2>")
}

async fn debug_handler(req: Request) -> Response {
    let body = format!("headers: {:#?}", req.headers());
    http::Response::builder()
        .status(200)
        .body(Body::from(body).map_err(axum::Error::new).boxed_unsync())
        .unwrap_or_log()
}

async fn fallback_handler(method: hyper::Method, uri: hyper::Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("the reqeusted resource does not exist\n{} {}", method, uri),
    )
}

pub fn new_service(cfg: &Config) -> BoxCloneService<Request, Response, BoxError> {
    let static_files_handler = routing::get_service(ServeDir::new(&cfg.static_files_dir))
        .handle_error(|err| async move {
            tracing::error!("failed to serve static file: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        });

    let router = Router::new()
        .route("/_hello", routing::get(hello_handler))
        .route("/_debug", routing::get(debug_handler))
        // if non of the above routes match, defer to the static file handler
        .fallback(
            Router::new()
                .route("/*path", static_files_handler)
                .fallback(fallback_handler.into_service()),
        );

    ServiceBuilder::new()
        .concurrency_limit(10)
        .buffer(100)
        .service(router)
        .boxed_clone()
}

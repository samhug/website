mod redirect;

use axum::{
    body::{Body, HttpBody},
    handler::Handler,
    http::{self, StatusCode, Uri},
    routing, BoxError, Router,
};
use http_redirect::RedirectLayer;
use tower::{util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::{
    compression::{CompressionBody, CompressionLayer},
    services::ServeDir,
};
use tracing_unwrap::ResultExt;

use std::sync::Arc;

use crate::config::Config;

pub type Request = axum::http::Request<RequestBody>;
pub type RequestBody = axum::body::Body;
pub type Response = axum::response::Response;

const HEALTH_CHECK_ENDPOINT: &str = "/_health";

async fn health_check_handler(_req: Request) -> &'static str {
    "not dead yet..."
}

async fn debug_handler(req: Request) -> Response {
    let body = format!("headers: {:#?}", req.headers());
    http::Response::builder()
        .status(200)
        .body(Body::from(body).map_err(axum::Error::new).boxed_unsync())
        .unwrap_or_log()
}

async fn fallback_handler(method: http::Method, uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("the reqeusted resource does not exist\n{} {}", method, uri),
    )
}

pub fn new(cfg: Arc<Config>) -> BoxCloneService<Request, Response, BoxError> {
    let static_files_handler = routing::get_service(ServeDir::new(&cfg.static_files_dir))
        .handle_error(|err| async move {
            tracing::error!("failed to serve static file: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        });

    let router = Router::new()
        .route(HEALTH_CHECK_ENDPOINT, routing::get(health_check_handler))
        .route("/_debug", routing::get(debug_handler))
        // if none of the above routes match, defer to the static file handler
        .fallback(
            Router::new()
                .route("/*path", static_files_handler)
                .fallback(fallback_handler.into_service()),
        )
        .boxed();

    ServiceBuilder::new()
        .concurrency_limit(10)
        .buffer(100)
        .map_response(|response: http::Response<CompressionBody<_>>| {
            response.map(|b| b.map_err(axum::Error::new).boxed_unsync())
        })
        .layer(CompressionLayer::new())
        .option_layer(cfg.host_redirect.as_ref().map(|host| {
            tracing::info!("adding host redirect layer with host: {}", host);
            RedirectLayer::new(redirect::RequestRedirect::new(host, HEALTH_CHECK_ENDPOINT))
        }))
        .service(router)
        .boxed_clone()
}

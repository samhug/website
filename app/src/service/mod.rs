use axum::{
    body::{Body, HttpBody},
    handler::Handler,
    http::{self, header, uri, StatusCode, Uri},
    routing, BoxError, Router,
};

use tower::{util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;

use tracing_unwrap::ResultExt;

use std::sync::Arc;

use crate::config::Config;

pub type Request = axum::http::Request<RequestBody>;
pub type RequestBody = axum::body::Body;
pub type Response = axum::response::Response;

const HEALTH_CHECK_ENDPOINT: &str = "/_health";
const HTTP_HOST: &str = "sa.m-h.ug"; // TODO:

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

async fn redirect_handler(
    request_uri: Uri,
) -> (StatusCode, [(header::HeaderName, header::HeaderValue); 1]) {
    let target_uri = {
        let mut parts = request_uri.into_parts();
        parts.scheme = Some(uri::Scheme::HTTPS);
        parts.authority = Some(uri::Authority::from_static(HTTP_HOST));
        Uri::from_parts(parts).unwrap()
    };
    (
        StatusCode::MOVED_PERMANENTLY,
        [(
            header::LOCATION,
            header::HeaderValue::from_str(&target_uri.to_string()).unwrap(),
        )],
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

    let redirect_svc = redirect_handler.into_service().boxed();

    let steer_svc = tower::steer::Steer::new(
        vec![router, redirect_svc],
        move |req: &Request, _services: &[_]| {
            let needs_https_redirect = {
                let is_https =
                    // Check x-forwarded-proto header
                    req
                        .headers()
                        .get("x-forwarded-proto")
                        .map(header::HeaderValue::to_str)
                        .and_then(Result::ok)
                        .map(|proto| proto == "https")
                        .unwrap_or(false)
                        ;
                cfg.https_redirect && !is_https
            };

            let needs_host_redirect = cfg
                .host_redirect
                .as_ref()
                .map(|canonical_host| {
                    let host_matches = req
                        .headers()
                        .get(header::HOST)
                        .map(header::HeaderValue::to_str)
                        .and_then(Result::ok)
                        .map(|host| host == canonical_host)
                        .unwrap_or(false);
                    !host_matches
                })
                .unwrap_or(false);

            let is_health_check = req.uri().path() == HEALTH_CHECK_ENDPOINT
                // && req
                //     .headers()
                //     .get(header::USER_AGENT)
                //     .map(header::HeaderValue::to_str)
                //     .and_then(Result::ok)
                //     .map(|user_agent| user_agent == "Consul Health Check")
                //     .unwrap_or(false)
                ;

            let needs_redirect = (needs_https_redirect || needs_host_redirect) && !is_health_check;

            // tracing::debug!("uri: {:?}", req.uri());
            // tracing::debug!("headers: {:?}", req.headers());
            // tracing::debug!("needs_https_redirect: {needs_https_redirect}, needs_host_redirect: {needs_host_redirect}, is_health_check: {is_health_check}, needs_redirect: {needs_redirect}");

            if !needs_redirect {
                0 // Index of `router`
            } else {
                1 // Index of `redirect_svc`
            }
        },
    );

    ServiceBuilder::new()
        .concurrency_limit(10)
        .buffer(100)
        .service(steer_svc)
        .boxed_clone()
}

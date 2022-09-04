use axum::{
    body::{Body, HttpBody},
    handler::Handler,
    http::{self, header, uri, StatusCode, Uri},
    response::Html,
    routing, BoxError, Router,
};

use tower::{util::BoxCloneService, ServiceBuilder, ServiceExt};
use tower_http::services::ServeDir;

use tracing_unwrap::ResultExt;

use crate::config::Config;

pub type Request = axum::http::Request<RequestBody>;
pub type RequestBody = axum::body::Body;
pub type Response = axum::response::Response;

const HTTP_HOST: &str = "sa.m-h.ug";

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

pub fn new_service(cfg: &Config) -> BoxCloneService<Request, Response, BoxError> {
    let static_files_handler = routing::get_service(ServeDir::new(&cfg.static_files_dir))
        .handle_error(|err| async move {
            tracing::error!("failed to serve static file: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        });

    let router = Router::new()
        .route("/_hello", routing::get(hello_handler))
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
        |req: &Request, _services: &[_]| {
            let is_https = req.uri().scheme() == Some(&uri::Scheme::HTTPS);
            let is_correct_uri_authority =
                req.uri().authority() == Some(&uri::Authority::from_static(HTTP_HOST));
            let is_correct_host_header = req.headers().get(header::HOST)
                == Some(&header::HeaderValue::from_static(HTTP_HOST));
            if is_https && (is_correct_uri_authority || is_correct_host_header) {
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

use std::{marker::PhantomData, str::FromStr};

use axum::http::{header, uri, Request, Response, StatusCode, Uri};

use http_redirect::Redirector;

pub struct RequestRedirect<ResBody> {
    host: String,
    healthcheck_uri: String,
    _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> RequestRedirect<ResBody> {
    pub fn new(host: impl ToString, healthcheck_uri: impl ToString) -> Self {
        Self {
            host: host.to_string(),
            healthcheck_uri: healthcheck_uri.to_string(),
            _ty: PhantomData,
        }
    }
}

impl<ResBody> Clone for RequestRedirect<ResBody> {
    fn clone(&self) -> Self {
        Self {
            host: self.host.clone(),
            healthcheck_uri: self.healthcheck_uri.clone(),
            _ty: PhantomData,
        }
    }
}

impl<B, ResBody> Redirector<B> for RequestRedirect<ResBody>
where
    ResBody: http_body::Body + Default,
{
    type ResponseBody = ResBody;

    fn redirect(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        // does the request uri have an https scheme? (only relevant for proxied requests)
        let is_https_uri = request
            .uri()
            .scheme()
            .map(|v| v == &uri::Scheme::HTTPS)
            .unwrap_or(false);

        // does the request include an `x-forwarded-proto: https` header
        let is_https_forwarded = request
            .headers()
            .get("x-forwarded-proto")
            .map(header::HeaderValue::to_str)
            .and_then(Result::ok)
            .map(|v| v == "https")
            .unwrap_or(false);

        let is_health_check = request.uri().path() == self.healthcheck_uri
            || request
                .headers()
                .get(header::USER_AGENT)
                .map(header::HeaderValue::to_str)
                .and_then(Result::ok)
                .map(|v| v == "Consul Health Check")
                .unwrap_or(false);

        tracing::trace!("is_https_uri: {is_https_uri}, is_https_forwarded: {is_https_forwarded}, is_health_check: {is_health_check}");

        if is_https_uri || is_https_forwarded || is_health_check {
            return Ok(());
        }

        let target_uri = {
            let mut parts = request.uri().clone().into_parts();
            parts.scheme = Some(uri::Scheme::HTTPS);
            parts.authority = Some(uri::Authority::from_str(self.host.as_str()).unwrap());
            Uri::from_parts(parts).unwrap()
        };

        let redirect_res = Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::LOCATION, target_uri.to_string())
            .body(ResBody::default())
            .unwrap();
        Err(redirect_res)
    }
}

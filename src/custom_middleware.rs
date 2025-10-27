/// Custom middleware demonstrating request tracing
///
/// This module shows how to:
/// - Inject a request ID into each request
/// - Create spans for HTTP requests
/// - Track request/response metrics
/// - Link logs across the entire request lifecycle
use crate::metrics;
use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::StatusCode,
};
use futures::future::LocalBoxFuture;
use std::rc::Rc;
use tracing::{info, warn};
use uuid::Uuid;

/// Middleware that injects a request ID and creates a span for each request
pub struct RequestIdMiddleware;

fn normalize_endpoint(path: &str) -> String {
    let mut parts = Vec::new();

    for segment in path.split('/') {
        if segment.is_empty() {
            continue;
        }

        if segment.parse::<u64>().is_ok() || Uuid::parse_str(segment).is_ok() {
            parts.push("{id}");
        } else {
            parts.push(segment);
        }
    }

    if parts.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", parts.join("/"))
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddlewareService<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequestIdMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct RequestIdMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let endpoint = normalize_endpoint(&path);

        // Insert request ID into request extensions
        req.extensions_mut().insert(request_id.clone());

        let span = tracing::info_span!(
            "http_request",
            request_id = %request_id,
            method = %method,
            path = %path,
            status = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
        );

        info!("Request started");

        let service = self.service.clone();
        let start_time = std::time::Instant::now();

        let span_clone = span.clone();
        let method_label = method.clone();
        let endpoint_label = endpoint.clone();

        metrics::track_request_start(&method_label, &endpoint_label);

        Box::pin(
            async move {
                let result = service.call(req).await;
                let elapsed = start_time.elapsed();
                let duration_ms = elapsed.as_millis() as u64;

                match result {
                    Ok(res) => {
                        let status = res.status();

                        span_clone.record("status", status.as_u16());
                        span_clone.record("duration_ms", duration_ms);

                        metrics::track_request_result(
                            &method_label,
                            &endpoint_label,
                            status.as_u16(),
                            elapsed,
                        );

                        info!("Request completed with status {}", status);

                        Ok(res)
                    }
                    Err(err) => {
                        span_clone.record("status", StatusCode::INTERNAL_SERVER_ERROR.as_u16());
                        span_clone.record("duration_ms", duration_ms);

                        metrics::track_request_result(
                            &method_label,
                            &endpoint_label,
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            elapsed,
                        );

                        warn!("Request failed: {}", err);

                        Err(err)
                    }
                }
            }
            .instrument(span),
        )
    }
}

// Helper to instrument futures
use tracing::Instrument;

/// Custom middleware demonstrating request tracing
///
/// This module shows how to:
/// - Inject a request ID into each request
/// - Create spans for HTTP requests
/// - Track request/response metrics
/// - Link logs across the entire request lifecycle

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::rc::Rc;
use tracing::info;
use uuid::Uuid;

/// Middleware that injects a request ID and creates a span for each request
pub struct RequestIdMiddleware;

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

        Box::pin(
            async move {
                let res = service.call(req).await?;
                let duration_ms = start_time.elapsed().as_millis();
                let status = res.status();

                span_clone.record("status", status.as_u16());
                span_clone.record("duration_ms", duration_ms);

                info!("Request completed with status {}", status);

                Ok(res)
            }
            .instrument(span),
        )
    }
}

// Helper to instrument futures
use tracing::Instrument;

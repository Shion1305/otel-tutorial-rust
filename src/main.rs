use actix_web::{App, HttpServer, middleware, web};
use opentelemetry::global;
use tracing::info;

mod custom_middleware;
mod handlers;
mod metrics;
mod observability;

use metrics::metrics_handler;
use observability::setup_telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize observability (tracing, OTEL, Loki)
    setup_telemetry().await;

    info!("Starting OpenTelemetry Tutorial Application");

    // Create HTTP server
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(custom_middleware::RequestIdMiddleware)
            .route("/metrics", web::get().to(metrics_handler))
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(handlers::health_check))
                    .route("/users", web::get().to(handlers::list_users))
                    .route("/users", web::post().to(handlers::create_user))
                    .route("/users/{id}", web::get().to(handlers::get_user))
                    .route("/compute", web::post().to(handlers::compute_fibonacci)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run();

    info!("Server running on http://127.0.0.1:8080");

    server.await?;

    // Shutdown telemetry gracefully
    global::shutdown_tracer_provider();
    Ok(())
}

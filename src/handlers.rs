/// Request handlers demonstrating OTEL instrumentation patterns
///
/// This module shows how to:
/// - Create spans for individual operations
/// - Add events and attributes to spans
/// - Handle errors with proper tracing
/// - Track performance metrics

use actix_web::{web, HttpResponse, Result as ActixResult, error::ErrorInternalServerError};
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug, Instrument};
use uuid::Uuid;

/// User data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// Health check endpoint - simplest instrumentation
///
/// Demonstrates:
/// - Basic span creation
/// - Logging structured data
pub async fn health_check() -> ActixResult<HttpResponse> {
    let span = tracing::info_span!("health_check");

    async {
        info!("Health check requested");
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "version": env!("CARGO_PKG_VERSION")
        })))
    }
    .instrument(span)
    .await
}

/// List all users - demonstrates span attributes
///
/// Demonstrates:
/// - Creating spans with attributes
/// - Nested spans
/// - Error handling in spans
#[tracing::instrument]
pub async fn list_users() -> ActixResult<HttpResponse> {
    info!("Listing all users");

    // Simulate database query
    let users = vec![
        User {
            id: Uuid::new_v4().to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: Uuid::new_v4().to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ];

    info!(count = users.len(), "Users retrieved");

    Ok(HttpResponse::Ok().json(users))
}

/// Create a new user - demonstrates request parsing and validation
///
/// Demonstrates:
/// - Extracting and logging request data
/// - Validation with proper error spans
/// - Recording user ID in span
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(skip(req))]
pub async fn create_user(req: web::Json<CreateUserRequest>) -> ActixResult<HttpResponse> {
    info!("Creating new user: {}", req.name);

    // Validate email (simple validation)
    if !req.email.contains('@') {
        warn!("Invalid email format provided");
        return Err(ErrorInternalServerError("Invalid email format"));
    }

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: req.name.clone(),
        email: req.email.clone(),
    };

    info!(user_id = %user.id, "User created successfully");

    Ok(HttpResponse::Created().json(user))
}

/// Get a specific user - demonstrates error handling in spans
///
/// Demonstrates:
/// - Path parameters in spans
/// - Error cases with context
/// - Using nested spans
#[tracing::instrument(skip(path))]
pub async fn get_user(path: web::Path<String>) -> ActixResult<HttpResponse> {
    let user_id = path.into_inner();

    info!(user_id = %user_id, "Fetching user by ID");

    // Simulate database lookup
    if user_id == "404" {
        warn!("User not found");
        return Err(ErrorInternalServerError("User not found"));
    }

    let user = User {
        id: user_id.clone(),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    info!("User found");
    Ok(HttpResponse::Ok().json(user))
}

/// Compute fibonacci - demonstrates CPU-intensive work with nested spans
///
/// Demonstrates:
/// - Long-running operations
/// - Performance tracking
/// - Nested span structure
/// - Recording intermediate results
#[derive(Debug, Deserialize)]
pub struct FibonacciRequest {
    pub n: u32,
}

#[tracing::instrument(skip(req))]
pub async fn compute_fibonacci(req: web::Json<FibonacciRequest>) -> ActixResult<HttpResponse> {
    if req.n > 50 {
        warn!("Fibonacci computation requested with large n: {}", req.n);
        return Err(ErrorInternalServerError("n too large (max 50)"));
    }

    info!("Computing fibonacci number for n={}", req.n);

    let result = compute_fib_recursive(req.n);

    info!(result = result, "Fibonacci computation completed");

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "n": req.n,
        "result": result,
    })))
}

/// Recursive fibonacci with instrumentation
#[tracing::instrument(skip_all)]
fn compute_fib_recursive(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    debug!("Computing fib({})", n);
    let result = compute_fib_recursive(n - 1) + compute_fib_recursive(n - 2);
    result
}

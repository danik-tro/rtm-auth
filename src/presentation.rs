pub mod handlers;

pub async fn application_router() -> axum::Router {
    axum::Router::new()
        // `GET /health` the `health check` endpoint
        .route(
            "/health",
            axum::routing::get(handlers::health_check::healthz),
        )
}

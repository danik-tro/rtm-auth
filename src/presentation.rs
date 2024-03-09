pub mod docs;
pub mod handlers;

#[allow(clippy::unused_async)]
pub async fn application_router() -> axum::Router {
    let router = axum::Router::new()
        // `GET /health` the `health check` endpoint
        .route(
            "/health",
            axum::routing::get(handlers::health_check::healthz),
        );

    docs::setup(router)
}

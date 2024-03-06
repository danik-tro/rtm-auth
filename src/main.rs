#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build our application with a route
    let app = axum::Router::new()
        // `GET /health` the `health check` endpoint
        .route("/health", axum::routing::get(healthz));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(serde::Serialize)]
struct HealthzResponse {
    status: String,
}

async fn healthz() -> (axum::http::StatusCode, axum::Json<HealthzResponse>) {
    let response = HealthzResponse {
        status: "Ok".into(),
    };

    (axum::http::StatusCode::OK, axum::Json(response))
}

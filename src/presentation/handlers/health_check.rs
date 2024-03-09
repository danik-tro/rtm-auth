#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    status: String,
}

impl axum::response::IntoResponse for HealthCheckResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}

pub async fn healthz() -> HealthCheckResponse {
    HealthCheckResponse {
        status: "Ok".into(),
    }
}

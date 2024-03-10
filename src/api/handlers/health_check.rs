use crate::api::constants::HEALTH_CHECK_PATH;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    status: String,
}

impl axum::response::IntoResponse for HealthResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}

#[utoipa::path(
    get,
    path = HEALTH_CHECK_PATH,
    responses(
        (status = 200, description = "Heatlh check", body = HealthResponse)
    )
)]
pub async fn healthz() -> HealthResponse {
    HealthResponse {
        status: "Ok".into(),
    }
}

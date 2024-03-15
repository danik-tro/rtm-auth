use axum::{Extension, Json};

use crate::{api::constants::USERS_PATH, domain::services::ArcRegistrationService};

use super::ApiResult;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SignupResponse {
    status: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct SignupRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

impl axum::response::IntoResponse for SignupResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::CREATED, Json(self)).into_response()
    }
}

#[utoipa::path(
    post,
    path = USERS_PATH,
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Sign up", body = SignupResponse),
        (status = 409, description = "Email is already in use", body = ApiErrorMessage),
    )
)]
pub async fn signup(
    Extension(service): Extension<ArcRegistrationService>,
    Json(create_user): Json<SignupRequest>,
) -> ApiResult<SignupResponse> {
    service.register(create_user.try_into()?).await?;

    Ok(SignupResponse {
        status: "Ok".into(),
    })
}

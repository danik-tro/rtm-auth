use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::domain::{
    models::DomainError,
    services::{AuthenticationServiceError, HashError, RegistrationServiceError},
};

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ApiErrorMessage {
    message: String,
}

impl ApiErrorMessage {
    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Domain(
        #[from]
        #[source]
        DomainError,
    ),
    #[error("{0}")]
    AuthenticationService(
        #[from]
        #[source]
        AuthenticationServiceError,
    ),
    #[error("{0}")]
    RegistrationService(
        #[from]
        #[source]
        RegistrationServiceError,
    ),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Domain(err) => match err {
                DomainError::ParseEmail(v_err) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ApiErrorMessage::err(v_err.to_string())),
                )
                    .into_response(),
            },
            Self::AuthenticationService(s_err) => match s_err {
                AuthenticationServiceError::IncorrectCredentials => (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiErrorMessage::err(s_err.to_string())),
                )
                    .into_response(),
                AuthenticationServiceError::UserRepository(_) => {
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                AuthenticationServiceError::VerifyPassword(vf_err) => {
                    if let HashError::VerifiyFailed = &vf_err {
                        (
                            StatusCode::UNAUTHORIZED,
                            Json(ApiErrorMessage::err(vf_err.to_string())),
                        )
                            .into_response()
                    } else {
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                }
            },
            Self::RegistrationService(s_err) => match s_err {
                RegistrationServiceError::EmailIsNotUnique => {
                    (StatusCode::CONFLICT, Json(s_err.to_string())).into_response()
                }
                RegistrationServiceError::UserRepository(_) => {
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
                RegistrationServiceError::HashPassword(_) | RegistrationServiceError::Tokio(_) => {
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            },
        }
    }
}

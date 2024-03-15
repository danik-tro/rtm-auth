use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use super::{errors::ApiErrorMessage, handlers::health_check};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check::healthz,
    ),
    components(
        schemas(
            health_check::HealthResponse,
            ApiErrorMessage,
        ),
    ),
    tags(
        (name = "health_check", description = "Health check"),
    )
)]
pub struct ApiDoc;

pub fn setup(router: axum::Router) -> axum::Router {
    router
        .merge(SwaggerUi::new("/docs/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
}

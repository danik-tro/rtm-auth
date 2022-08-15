use actix_web::web;
use serde::Serialize;

use crate::AppState;

pub mod users;

#[derive(Serialize)]
pub struct Health {
    status: String,
}

#[derive(Serialize)]
pub struct ApiInfo {
    version: String,
    api_name: String,
}

pub async fn health_check() -> web::Json<Health> {
    web::Json(Health {
        status: String::from("OK"),
    })
}

pub async fn api_info(data: web::Data<AppState>) -> web::Json<ApiInfo> {
    let version = data.api_version.clone();
    let api_name = data.app_name.clone();

    web::Json(ApiInfo { version, api_name })
}

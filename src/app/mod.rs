pub mod views;

use super::app::views as root_views;

use actix_web::{web, HttpResponse};

pub struct AppState {
    pub app_name: String,
    pub api_version: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") })),
    )
    .service(web::resource("/health").route(web::get().to(root_views::health_check)))
    .service(web::scope("/api").configure(api_config));
}

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(root_views::api_info)));
}

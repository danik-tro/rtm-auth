use actix_web::web;
use serde::{Deserialize, Serialize};

use super::AppState;

pub mod users;

#[derive(Serialize, Deserialize)]
pub struct Health {
    status: String,
}

#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::app;
    use crate::app::views::{ApiInfo, Health};

    #[actix_web::test]
    async fn test_health_check() {
        let state = app::AppState {
            app_name: String::from("rtm-auth"),
            api_version: String::from("v0.1.0"),
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .configure(app::config),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let health_info: Health = test::read_body_json(resp).await;

        assert_eq!(health_info.status, "OK");
    }

    #[actix_web::test]
    async fn test_api_info() {
        let app_name = "rtm-auth";
        let version = "v0.1.0";

        let state = app::AppState {
            app_name: String::from(app_name.to_string()),
            api_version: String::from(version.to_string()),
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .configure(app::config),
        )
        .await;

        let req = test::TestRequest::get().uri("/api").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let api_info: ApiInfo = test::read_body_json(resp).await;

        assert_eq!(api_info.api_name, app_name);
        assert_eq!(api_info.version, version);
    }
}

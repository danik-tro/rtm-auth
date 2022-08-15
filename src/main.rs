use actix_web::{web, App, HttpServer};
use std::env;

mod app;

pub struct AppState {
    app_name: String,
    api_version: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    let bind_address = env::var("BIND_ADDRESS").unwrap_or(String::from("127.0.0.1:8080"));

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("rtm-auth"),
                api_version: String::from("v0.1.0"),
            }))
            .configure(app::config)
    })
    .bind(&bind_address)?
    .run()
    .await
}

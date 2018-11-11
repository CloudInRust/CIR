use actix::prelude::*;
use actix_web::{
    App, middleware
};

use db::{DbExecutor};
use error::{error_middleware::ErrorTemplateHandler};
use build_info;

pub mod web_frontend;
mod controllers;

pub struct AppState {
    db: Addr<DbExecutor>
}

pub type WebApp = App<AppState>;

pub fn init_essential_app_features(app: WebApp) -> WebApp {
    app
        .middleware(middleware::Logger::new(r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#))
        .middleware(ErrorTemplateHandler)
        .middleware(middleware::DefaultHeaders::new().header("Server", format!("CloudInRust v{}", build_info::PKG_VERSION).as_str()))
}
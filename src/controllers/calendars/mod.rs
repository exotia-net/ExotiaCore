use actix_web::web::{ServiceConfig, self};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::auth_middleware;

pub mod get;
pub mod update;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CalendarEntity {
    step: i32,
    streak: i32,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(web::resource("/calendars").wrap(from_fn(auth_middleware))
                .route(web::get().to(get::get))
                .route(web::put().to(update::update))
            );
    }
}

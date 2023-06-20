use actix_web::web::{ServiceConfig, self};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use utoipa::ToSchema;
use sea_orm::prelude::DateTime;

use crate::auth_middleware;

pub mod get;
pub mod update;
pub mod rewards;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEntity {
    step: i32,
    streak: i32,
    obtained_rewards: Vec<i32>,
    last_obtained: DateTime,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(web::resource("/calendars").wrap(from_fn(auth_middleware))
                .route(web::get().to(get::get))
                .route(web::put().to(update::update))
            )
            .service(web::resource("/calendars/rewards").wrap(from_fn(auth_middleware)).route(web::get().to(rewards::rewards)))
        ;
    }
}

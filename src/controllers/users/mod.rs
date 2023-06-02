use actix_web::{web::{ServiceConfig, self}, guard};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{get_auth_key, auth_middleware};

pub mod auth;
pub mod create;
pub mod update;

#[derive(Deserialize, ToSchema)]
pub struct UserEntity {
    pub uuid: String,
    pub nick: String,
    pub first_ip: Option<String>,
    pub last_ip: String,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
	|config: &mut ServiceConfig| {
		config
			.service(web::resource("/me").wrap(from_fn(auth_middleware)).route(web::get().to(auth::auth)))
			.service(web::resource("/signUp").wrap(from_fn(auth_middleware)).route(web::post().to(create::create)))
			.service(web::resource("/update").guard(guard::Header("ExotiaKey", get_auth_key())).route(web::put().to(update::update)));
	}
}


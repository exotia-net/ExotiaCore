pub mod get;
pub mod economy;

use actix_web::web::{ServiceConfig, self};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::auth_middleware;

#[derive(Deserialize, Debug)]
pub enum ServerType {
    Survival,
}

#[derive(Deserialize, ToSchema)]
pub struct Economy {
    balance: i32,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
	|config: &mut ServiceConfig| {
		config
			.service(web::resource("/servers/{server}").wrap(from_fn(auth_middleware)).route(web::get().to(get::get)))
			.service(web::resource("/servers/{server}/economy").wrap(from_fn(auth_middleware)).route(web::put().to(economy::economy)));
	}
}

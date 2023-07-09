pub mod get;
pub mod economy;
pub mod economy_top;

use actix_web::{web::{ServiceConfig, self}, guard};
use actix_web_lab::middleware::from_fn;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{auth_middleware, get_auth_key};

#[derive(Deserialize, Debug, ToSchema)]
pub enum ServerType {
    Survival,
}

#[derive(Deserialize, ToSchema)]
pub struct Economy {
    balance: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct TopsQuery {
    limit: u64,
}

#[derive(Serialize, ToSchema)]
pub struct EconomyTop {
    uuid: uuid::Uuid,
    balance: i32
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
	|config: &mut ServiceConfig| {
		config
			.service(web::resource("/servers/{server}").wrap(from_fn(auth_middleware)).route(web::get().to(get::get)))
			.service(web::resource("/servers/{server}/economy").wrap(from_fn(auth_middleware)).route(web::put().to(economy::economy)))
			.service(web::resource("/servers/{server}/economy/tops").guard(guard::Header("ExotiaKey", get_auth_key())).route(web::get().to(economy_top::economy_top)));
	}
}

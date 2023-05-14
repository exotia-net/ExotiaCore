use actix_web::web::{ServiceConfig, self};
use actix_web_lab::middleware::from_fn;

use crate::auth_middleware;

pub mod get;
// pub mod charge;
// pub mod buy;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
	|config: &mut ServiceConfig| {
		config
			.service(web::resource("/wallet").wrap(from_fn(auth_middleware)).route(web::get().to(get::get)));
			// .service(web::resource("/wallet/charge").wrap(from_fn(auth_middleware)).route(web::post().to(charge::charge)))
			// .service(web::resource("/wallet/buy").wrap(from_fn(auth_middleware)).route(web::post().to(buy::buy)));
	}
}

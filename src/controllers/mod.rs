pub mod auth;

use actix_web::web::ServiceConfig;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
	|config: &mut ServiceConfig| {
		config
			.service(auth::auth);
	}
}


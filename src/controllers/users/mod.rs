pub mod auth;
pub mod create;

use actix_web::web::ServiceConfig;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct User {
    uuid: String,
    ip: String,
}

// pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
// 	|config: &mut ServiceConfig| {
// 		config
//             .service(create::create);
// 	}
// }

// pub fn blocked() -> impl FnOnce(&mut ServiceConfig) {
// 	|config: &mut ServiceConfig| {
// 		config
// 			.service(auth::auth);
// 	}
// }

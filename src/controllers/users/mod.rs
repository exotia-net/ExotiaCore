pub mod auth;
pub mod create;


use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct User {
    uuid: String,
    ip: String,
    nick: String,
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

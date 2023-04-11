use actix_web::{web::ServiceConfig, Responder, get};

#[utoipa::path(
	tag = "Hello",
	responses(
		(status = 200, description = "Ok"),
	)
)]
#[get("/hello")]
pub async fn hello() -> impl Responder {
	"Ok"
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
	|config: &mut ServiceConfig| {
		config
			.service(hello);
	}
}


use actix_web::{Responder, web, HttpResponse, http::header::ContentType};
use sea_orm::{Set, EntityTrait};
use serde_json::json;

use crate::{ApiError, entities::{users, prelude::Users}, AppState};

#[utoipa::path(
	post,
	path = "/auth/signUp",
	tag = "Auth",
	responses(
		(status = 201, description = "User is successfuly created"),
		(status = 500, description = "Database error"),
		(status = 404, description = "If value is none")
	)
)]
// #[post("/auth/signUp")]
pub async fn create(
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let exotia_key_guard = data.exotia_key.lock()?;
	let user_data = &exotia_key_guard.as_ref().unwrap();

	let user = users::ActiveModel {
		uuid: Set(user_data.uuid.clone()),
		nick: Set(user_data.nick.clone()),
		first_ip: Set(user_data.ip.clone()),
		last_ip: Set(user_data.ip.clone()),
		..Default::default()
	};

	let user_insert = Users::insert(user).exec(&data.conn).await;

	drop(exotia_key_guard);

	return match user_insert {
		Ok(v) => {
			Ok(HttpResponse::Created().finish())
		},
		Err(_) => Ok(HttpResponse::Conflict()
			.content_type(ContentType::json())
			.json(json!({ "message": "User with that uuid already exists" }))),
	}
}

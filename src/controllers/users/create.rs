use actix_web::{Responder, web, HttpResponse, http::header::ContentType};
use sea_orm::{Set, EntityTrait};
use serde_json::json;

use crate::{ApiError, entities::{users, prelude::Users}, AppState};
use super::User;

#[utoipa::path(
	post,
	path = "/auth/signUp",
	tag = "Auth",
	request_body(content = User, description = "User to add", content_type = "application/json"),
	responses(
		(status = 201, description = "User is successfuly created"),
		(status = 500, description = "Database error"),
		(status = 404, description = "If value is none")
	)
)]
// #[post("/auth/signUp")]
pub async fn create(
	body: web::Json<User>, 
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let user = users::ActiveModel {
		uuid: Set(body.uuid.clone()),
		nick: Set(body.nick.clone()),
		first_ip: Set(body.ip.clone()),
		last_ip: Set(body.ip.clone()),
		..Default::default()
	};
	println!("{:#?}", &user);

	let user_insert = Users::insert(user).exec(&data.conn).await;

	return user_insert.map_or_else(|_| Ok(HttpResponse::Conflict()
		.content_type(ContentType::json())
		.json(json!({ "message": "User with that uuid already exists" }))), |_| {
			Ok(HttpResponse::Created().finish())
		})
}

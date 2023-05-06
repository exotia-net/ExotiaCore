use actix_web::{Responder, web, HttpResponse, http::header::ContentType, post};
use sea_orm::{Set, EntityTrait};
use serde_json::json;

use crate::{ApiError, entities::{users, prelude::Users, survival_economy, wallet}, AppState};

use crate::entities::prelude::{Wallet, SurvivalEconomy};

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
// #[post("/signUp")]
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

	let user_insert = match user_insert {
		Ok(v) => v,
		Err(_) => return Ok(HttpResponse::Conflict()
			.content_type(ContentType::json())
			.json(json!({ "message": "User with that uuid already exists" }))),
	};

	let survival = survival_economy::ActiveModel {
		user_id: Set(user_insert.last_insert_id),
		balance: Set(0),
		..Default::default()
	};

	let survival_economy_insert = match SurvivalEconomy::insert(survival).exec(&data.conn).await {
		Ok(v) => v,
		Err(_) => {
			Users::delete_by_id(user_insert.last_insert_id);

			return Ok(HttpResponse::InternalServerError()
				.content_type(ContentType::json())
				.json(json!({ "message": "Failed to create SurvivalEconomy Table" })));
		}
	};

	let wallet_ = wallet::ActiveModel {
		user_id: Set(user_insert.last_insert_id),
		coins: Set(0.0),
		spent_coins: Set(0.0),
		..Default::default()
	};

	match Wallet::insert(wallet_).exec(&data.conn).await {
		Ok(_) => (),
		Err(_) => {
			Users::delete_by_id(user_insert.last_insert_id);
			SurvivalEconomy::delete_by_id(survival_economy_insert.last_insert_id);
		}
	}

	drop(exotia_key_guard);

	Ok(HttpResponse::Created().finish())
}

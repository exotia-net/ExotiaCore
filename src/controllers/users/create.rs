use actix_web::{Responder, web, HttpResponse, http::header::ContentType};
use sea_orm::{Set, EntityTrait};
use serde_json::json;

use crate::{ApiError, entities::{users, prelude::Users, survival_economy, wallet, calendars}, AppState};

use crate::entities::prelude::*;

/// Creates new user
#[utoipa::path(
	post,
	path = "/auth/signUp",
	tag = "Auth",
	responses(
		(status = 201, description = "User is successfuly created"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
	)
)]
pub async fn create(
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let exotia_key_guard = data.exotia_key.lock().await;
	let user_data = &exotia_key_guard.as_ref().ok_or(ApiError::NoneValue("User data"))?;

	let user = users::ActiveModel {
		uuid: Set(user_data.uuid),
		nick: Set(user_data.nick.clone()),
		first_ip: Set(user_data.ip.clone()),
		last_ip: Set(user_data.ip.clone()),
		..Default::default()
	};

	let user_insert = Users::insert(user).exec(&*data.conn.lock().await).await;

    let Ok(user_insert) = user_insert else { return Ok(HttpResponse::Conflict()
        .content_type(ContentType::json())
        .json(json!({ "message": "User with that uuid already exists" }))) };

	let survival = survival_economy::ActiveModel {
		user_id: Set(user_insert.last_insert_id),
		balance: Set(0),
		..Default::default()
	};

    let Ok(survival_economy_insert) = SurvivalEconomy::insert(survival).exec(&*data.conn.lock().await).await else {
        Users::delete_by_id(user_insert.last_insert_id);

        return Ok(HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(json!({ "message": "Failed to create SurvivalEconomy Table" })));
    };

	let wallet = wallet::ActiveModel {
		user_id: Set(user_insert.last_insert_id),
		coins: Set(0.0),
		spent_coins: Set(0.0),
		..Default::default()
	};

    let Ok(wallet_insert) = Wallet::insert(wallet).exec(&*data.conn.lock().await).await else {
        Users::delete_by_id(user_insert.last_insert_id);
        SurvivalEconomy::delete_by_id(survival_economy_insert.last_insert_id);

        return Ok(HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(json!({ "message": "Failed to create Wallet Table" })));
    };
    
    let calendar = calendars::ActiveModel {
        user_id: Set(user_insert.last_insert_id),
        step: Set(0),
        streak: Set(0),
        last_obtained: Set(None),
        obtained_rewards: Set(String::new()),
        ..Default::default()
    };

    if (Calendars::insert(calendar).exec(&*data.conn.lock().await).await).is_ok() {} else {
        Users::delete_by_id(user_insert.last_insert_id);
        SurvivalEconomy::delete_by_id(survival_economy_insert.last_insert_id);
        Wallet::delete_by_id(wallet_insert.last_insert_id);
    }

	drop(exotia_key_guard);

	Ok(HttpResponse::Created().finish())
}

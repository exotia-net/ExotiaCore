use actix_web::{web, Responder, HttpResponse};
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ModelTrait, Set};

use crate::{AppState, ApiError, entities::{prelude::*, users, survival_economy, wallet, calendars}};

/// Creates relations to specified user
#[utoipa::path(
	post,
	path = "/auth/setup",
	tag = "Auth",
	responses(
		(status = 201, description = "All relations to user are succesfuly fixed"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
	)
)]
pub async fn setup(
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let exotia_key_guard = data.exotia_key.lock().await;
	let user_data = &exotia_key_guard.as_ref().ok_or(ApiError::NoneValue("User data"))?;

    let user = Users::find()
        .filter(users::Column::Uuid.eq(user_data.uuid))
        .one(&*data.conn.lock().await)
        .await?
        .ok_or(ApiError::NoneValue("User"))?;

    if user.find_related(survival_economy::Entity).one(&*data.conn.lock().await).await?.is_none() {
        let survival = survival_economy::ActiveModel {
            user_id: Set(user.id),
            balance: Set(0),
            ..Default::default()
        };

        SurvivalEconomy::insert(survival).exec(&*data.conn.lock().await).await?;
    }

    if user.find_related(wallet::Entity).one(&*data.conn.lock().await).await?.is_none() {
        let wallet = wallet::ActiveModel {
            user_id: Set(user.id),
            coins: Set(0.0),
            spent_coins: Set(0.0),
            ..Default::default()
        };
        Wallet::insert(wallet).exec(&*data.conn.lock().await).await?;
    }
    
    if user.find_related(calendars::Entity).one(&*data.conn.lock().await).await?.is_none() {
        let calendar = calendars::ActiveModel {
            user_id: Set(user.id),
            step: Set(0),
            streak: Set(0),
            last_obtained: Set(NaiveDateTime::default()),
            obtained_rewards: Set(String::new()),
            ..Default::default()
        };
        Calendars::insert(calendar).exec(&*data.conn.lock().await).await?;
    }

    Ok(
        HttpResponse::Created().finish()
    )
}

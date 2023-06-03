use actix_web::{HttpRequest, web::Data};
use migration::{Expr, Alias};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::{ApiError, entities::{users, wallet}, AppState};

/// Removes money from user
#[utoipa::path(
    post,
    path = "/wallet",
    tag = "Wallet (Websocket)",
    request_body(content = String, description = "POST /wallet {uuid} {balance}", content_type = "text/plain"),
    responses(
        (status = 200, description = "Updated wallet", body = lib::entities::wallet::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn buy(
	req: &HttpRequest,
	args: &Vec<String>,
) -> Result<String, ApiError> {
	let data: &Data<AppState> = req.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?;

    let user = users::Entity::find()
        .filter(Expr::col(users::Column::Uuid).cast_as(Alias::new("VARCHAR")).eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
        .one(&data.conn)
        .await?
        .ok_or(ApiError::NoneValue("User with uuid"))?;

	let wallet_db = wallet::Entity::find()
		.filter(wallet::Column::UserId.eq(user.id))
		.one(&data.conn)
		.await?
		.ok_or(ApiError::NoneValue("Wallet"))?;

	let mut wallet_db: wallet::ActiveModel = wallet_db.into();

	wallet_db.coins = Set(args.get(1).ok_or(ApiError::NoneValue("Coins"))?.parse::<f32>()?);

	wallet_db.update(&data.conn).await?;

	Ok("Updated".to_string())
}

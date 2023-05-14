use actix_web::{HttpRequest, web::Data};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::{ApiError, entities::{users, wallet}, AppState};

pub async fn wallet_buy(
	req: &HttpRequest,
	args: &Vec<String>,
) -> Result<String, ApiError> {
	let data: &Data<AppState> = req.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?;

    let user = users::Entity::find()
        .filter(users::Column::Uuid.eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
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

	Ok(format!("Updated"))
}

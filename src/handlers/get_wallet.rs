use actix_web::{HttpRequest, web::Data};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

use crate::{ApiError, AppState, entities::{users, wallet}};

pub async fn get_wallet(req: &HttpRequest, args: &Vec<String>) -> Result<String, ApiError> {
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

	Ok(serde_json::to_string(&wallet_db)?)
}

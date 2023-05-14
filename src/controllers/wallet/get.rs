use actix_web::{Responder, HttpResponse, web};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use serde_json::json;

use crate::{ApiError, AppState, entities::wallet};


pub async fn get(
	data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().unwrap();

	let wallet_db = wallet::Entity::find()
		.filter(wallet::Column::UserId.eq(user.id))
		.one(&data.conn)
		.await?;

	Ok(
		HttpResponse::Ok().json(json!{ wallet_db })
	)
}

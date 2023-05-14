use actix_web::{HttpRequest, Responder, web, HttpResponse};
use serde_json::json;

use crate::{AppState, ApiError};

use super::WalletBuy;

pub async fn buy(
	req: HttpRequest,
	body: web::Json<WalletBuy>,
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().unwrap();

	let res = crate::handlers::wallet_buy::wallet_buy(&req, &vec![user.uuid.clone(), body.cost.to_string()]).await?;

	Ok(
		HttpResponse::Ok().json(json!{ res })
	)
}


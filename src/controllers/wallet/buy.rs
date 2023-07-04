use actix_web::{HttpRequest, Responder, web, HttpResponse};
use serde_json::json;

use crate::{AppState, ApiError};

use super::WalletBuy;

/// Removes money from user
#[utoipa::path(
    post,
    path = "/api/wallet/buy",
    tag = "Wallet",
    request_body(content = WalletBuy, description = "Removes money from user", content_type = "text/plain"),
    responses(
        (status = 200, description = "Updated wallet", body = lib::entities::wallet::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn buy(
	req: HttpRequest,
	body: web::Json<WalletBuy>,
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock().await;
    let user = &user_guard.as_ref().ok_or(ApiError::NoneValue("User"))?;

	let response = crate::websocket_handlers::wallet::buy(&req, &vec![user.uuid.to_string(), body.cost.to_string()]).await?;

	Ok(
		HttpResponse::Ok().json(json!{ response })
	)
}


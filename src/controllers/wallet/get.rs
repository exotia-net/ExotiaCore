use actix_web::{Responder, HttpResponse, web, HttpRequest};
use serde_json::json;

use crate::{ApiError, AppState};

/// Returns someones wallet
#[utoipa::path(
    get,
    path = "/api/wallet",
    tag = "Wallet",
    responses(
        (status = 200, description = "Requested wallet", body = lib::entities::wallet::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn get(
	req: HttpRequest,
	data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().unwrap();

	let res = crate::handlers::wallet::get(&req, &vec![user.uuid.to_string()]).await?;
	let res: serde_json::Value = serde_json::from_str(&res)?;

	Ok(
		HttpResponse::Ok().json(json!{ res })
	)
}

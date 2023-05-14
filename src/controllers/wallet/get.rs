use actix_web::{Responder, HttpResponse, web, HttpRequest};
use serde_json::json;

use crate::{ApiError, AppState};

pub async fn get(
	req: HttpRequest,
	data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().unwrap();

	let res = crate::handlers::get_wallet::get_wallet(&req, &vec![user.uuid.clone()]).await?;
	let res: serde_json::Value = serde_json::from_str(&res)?;

	Ok(
		HttpResponse::Ok().json(json!{ res })
	)
}

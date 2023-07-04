use actix_web::{Responder, HttpResponse, HttpRequest, web};
use serde_json::json;

use crate::{ApiError, AppState};

/// Returns Calendar rewards for some user
#[utoipa::path(
    get,
    path = "/api/calendars/rewards",
    tag = "Calendars",
    responses(
        (status = 200, description = "Calendar Rewards", body = Vec<String>),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn rewards(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock().await;
    let user = &user_guard.as_ref().ok_or(ApiError::NoneValue("User"))?;

    let response = crate::websocket_handlers::calendars::rewards(&req, &vec![user.uuid.to_string()]).await?;
    let response = if response.len() == 0 {
        Vec::new()
    } else {
        response.split("|").map(|v| v.parse::<i32>().unwrap_or(0)).collect::<Vec<_>>()
    };
    
    Ok(
        HttpResponse::Ok().json(json!({ "obtainedRewards": response }))
    )
}

use actix_web::{web, Responder, HttpResponse, HttpRequest};
use serde_json::json;

use crate::{AppState, ApiError};

/// Returns Calendar for some user
#[utoipa::path(
    get,
    path = "/api/calendars",
    tag = "Calendars",
    responses(
        (status = 200, description = "Calendar Entity", body = lib::entities::calendars::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn get(
    req: HttpRequest,
    data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().ok_or(ApiError::NoneValue("User"))?;

    let response = crate::handlers::calendars::get(&req, &vec![user.uuid.to_string()]).await?;
    let response: serde_json::Value = serde_json::from_str(&response)?;
    
    Ok(
        HttpResponse::Ok().json(json!{ response })
    )
}

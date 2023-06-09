use actix_web::{HttpRequest, web, Responder, HttpResponse};

use crate::{AppState, ApiError};

use super::CalendarEntity;

/// Updates calendar for user
#[utoipa::path(
	put,
	path = "/api/calendar",
	tag = "Calendars",
    request_body(content = CalendarEntity, description = "Calendar values", content_type = "application/json"),
	responses(
		(status = 201, description = "Calendar are successfuly updated"),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
	)
)]
pub async fn update(
    req: HttpRequest,
    body: web::Json<CalendarEntity>,
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let body = body.into_inner();
	let user_guard = data.user.lock().await;
    let user = &user_guard.as_ref().ok_or(ApiError::NoneValue("User"))?;

    crate::handlers::calendars::update(&req, &vec![user.uuid.to_string(), body.step.to_string(), body.streak.to_string()]).await?;

    drop(user_guard);
    Ok(
        HttpResponse::Ok().finish()
    )
}

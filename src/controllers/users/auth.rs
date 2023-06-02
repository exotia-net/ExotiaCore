use actix_web::{web, HttpResponse, http::header::ContentType};
use serde_json::json;

use crate::{ApiError, AppState};

/// Returns user from header
#[utoipa::path(
	get,
	path = "/auth/me",
	tag = "Auth",
	responses(
		(status = 200, description = "Current user", body = lib::entities::users::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
	)
)]
#[allow(clippy::unused_async)]
pub async fn auth(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok()
			.content_type(ContentType::json())
			.json(json!{ data.user })
    )
}

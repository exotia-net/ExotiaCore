use actix_web::{Responder, web, HttpResponse, HttpRequest};
use crate::{ApiError, AppState};
use super::{Economy, ServerType};

/// Updates user economy on server
#[utoipa::path(
    put,
    path = "/api/servers/{server}/economy",
    params(
        ("server" = ServerType, Path, description = "Type of the server")
    ),
    tag = "Servers",
    request_body(content = Economy, description = "New balance", content_type = "application/json"),
    responses(
        (status = 200, description = "Succesfully updated economy"),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn economy(
	req: HttpRequest,
	path: web::Path<ServerType>,
	body: web::Json<Economy>,
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().ok_or(ApiError::NoneValue("User"))?;

	crate::handlers::servers::economy(path.into_inner(), &req, &vec![user.uuid.to_string(), body.balance.to_string()]).await?;

	drop(user_guard);
	Ok(
		HttpResponse::Ok().finish()
	)
}

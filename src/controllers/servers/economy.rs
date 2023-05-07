use actix_web::{Responder, web, HttpResponse, HttpRequest};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::entities::survival_economy;
use crate::{ApiError, AppState};
use crate::entities::prelude::SurvivalEconomy;
use super::{Economy, ServerType};

#[utoipa::path(
	put,
	path = "/api/servers/:server/economy"
)]
pub async fn economy(
	req: HttpRequest,
	path: web::Path<ServerType>,
	body: web::Json<Economy>,
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let user_guard = data.user.lock()?;
	let user = &user_guard.as_ref().unwrap();

	crate::handlers::economy::economy(path.into_inner(), &req, &vec![user.uuid.clone(), body.balance.to_string()]).await;

	drop(user_guard);
	Ok(
		HttpResponse::Ok().finish()
	)
}

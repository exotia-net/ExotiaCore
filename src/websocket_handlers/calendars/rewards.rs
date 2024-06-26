use actix_web::{HttpRequest, web::Data};
use migration::{Expr, Alias};
use sea_orm::{EntityTrait, QueryFilter, ModelTrait};

use crate::{ApiError, entities::{calendars, users}, AppState};

/// Returns Calendar rewards for some user
#[utoipa::path(
    get,
    path = "/calendars/rewards",
    tag = "Calendars (Websocket)",
    responses(
        (status = 200, description = "Calendar Rewards", body = Vec<String>),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn rewards(req: &HttpRequest, args: &Vec<String>) -> Result<String, ApiError> {
	let data: &Data<AppState> = req.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?;

    let user = users::Entity::find()
        .filter(Expr::col(users::Column::Uuid).cast_as(Alias::new("VARCHAR")).eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
        .one(&*data.conn.lock().await)
        .await?
        .ok_or(ApiError::NoneValue("User with uuid"))?;

    let calendar = user.find_related(calendars::Entity)
        .one(&*data.conn.lock().await)
        .await?
        .ok_or(ApiError::NoneValue("Calendar User"))?;

    Ok(calendar.obtained_rewards)
}

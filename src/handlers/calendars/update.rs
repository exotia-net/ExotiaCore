use actix_web::{web::Data, HttpRequest};
use chrono::NaiveDateTime;
use migration::{Expr, Alias};
use sea_orm::{EntityTrait, QueryFilter, ModelTrait, Set, ActiveModelTrait};

use crate::{ApiError, AppState, entities::{users, calendars}};

/// Updates calendar for user
#[utoipa::path(
	put,
	path = "/calendars",
	tag = "Calendars (Websocket)",
    request_body(content = String, description = "PUT /calendar {uuid} {step} {streak} {obtained_rewards} {last_obtained}", content_type = "plain/text", example = json!("PUT /calendars 16b01d4c-7895-4843-b4d6-3dc302f2913b 1 2 1|2|3|4 2023-06-15T15:28:38")),
	responses(
		(status = 201, description = "Calendar are successfuly updated"),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
	)
)]
pub async fn update(
    req: &HttpRequest,
    args: &Vec<String>
) -> Result<String, ApiError> {
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

    let mut calendar_db: calendars::ActiveModel = calendar.into();

    calendar_db.step = Set(
        args.get(1)
            .ok_or(ApiError::NoneValue("Calendar step"))?
            .parse::<i32>()?
    );

    calendar_db.streak = Set(
        args.get(2)
            .ok_or(ApiError::NoneValue("Calendar streak"))?
            .parse::<i32>()?
    );

    calendar_db.obtained_rewards = Set(
        args.get(3)
            .ok_or(ApiError::NoneValue("Calendar obtained_rewards"))?.clone()
    );

    calendar_db.last_obtained = Set(NaiveDateTime::parse_from_str(
        args.get(4)
            .ok_or(ApiError::NoneValue("Calendar last_obtained"))?,
        "%Y-%m-%d %H:%M:%S"
    )?);

    calendar_db.update(&*data.conn.lock().await).await?;

    Ok(String::new())
}

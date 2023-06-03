use actix_web::{HttpRequest, web::Data};
use migration::{Expr, Alias};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

use crate::{controllers::servers::ServerType, ApiError, AppState, entities::{users, survival_economy}};

/// Returns User at Server
#[utoipa::path(
    get,
    path = "/servers/{server}/economy",
    tag = "Servers (Websocket)",
    request_body(content = String, description = "GET /servers/{server}/economy {uuid}", content_type = "text/plain"),
    responses(
        (status = 200, description = "Server Entity", body = lib::entities::servers::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn get(server_type: ServerType, req: &HttpRequest, args: &Vec<String>) -> Result<String, ApiError> {
	let data: &Data<AppState> = req.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?;

    let user = users::Entity::find()
        .filter(Expr::col(users::Column::Uuid).cast_as(Alias::new("VARCHAR")).eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
        .one(&data.conn)
        .await?
        .ok_or(ApiError::NoneValue("User with uuid"))?;

    match server_type {
        ServerType::Survival => {
            let server_db = survival_economy::Entity::find()
                .filter(survival_economy::Column::UserId.eq(user.id))
                .one(&data.conn)
                .await?
                .ok_or(ApiError::NoneValue("SurvivalEconomy User"))?;
            Ok(format!("{}", server_db.balance))
        }
    }
	// Ok(String::new())
}

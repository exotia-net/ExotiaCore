use actix_web::{HttpResponse, web, Responder};
use actix_web::http::header::ContentType;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use serde_json::json;

use crate::{ApiError, AppState, entities::{prelude::SurvivalEconomy, survival_economy}};
use super::ServerType;

/// Returns User at Server
#[utoipa::path(
    get,
    path = "/api/servers/{server}",
    params(
        ("server" = ServerType, Path, description = "Type of the server")
    ),
    tag = "Servers",
    responses(
        (status = 200, description = "Server Entity", body = lib::entities::servers::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn get(
    path: web::Path<ServerType>, 
    data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock().await;
    let user = &user_guard.as_ref().ok_or(ApiError::NoneValue("User"))?;

    let server = match path.into_inner() {
        ServerType::Survival => {
            SurvivalEconomy::find()
                .filter(survival_economy::Column::UserId.eq(user.id))
                .one(&data.conn)
                .await?
                .ok_or(ApiError::NoneValue("SurvivalEconomy User"))
        }
    };

    drop(user_guard);

    Ok(
        HttpResponse::Ok().content_type(ContentType::json()).json(json!{ server? })
    )
}

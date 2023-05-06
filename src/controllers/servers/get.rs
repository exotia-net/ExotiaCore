use actix_web::{HttpResponse, web, Responder};
use actix_web::http::header::ContentType;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use serde_json::json;

use crate::{ApiError, AppState, entities::{prelude::SurvivalEconomy, survival_economy}};
use super::ServerType;

#[utoipa::path(
    get,
    path = "/api/servers/{server}",
    tag = "Servers",
    responses(
        (status = 200, description = "")
    )
)]
pub async fn get(
    path: web::Path<ServerType>, 
    data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().unwrap();

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

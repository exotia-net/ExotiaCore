use actix_web::{HttpResponse, web, Responder};
use actix_web::http::header::ContentType;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use serde_json::json;

use crate::{ApiError, AppState, entities::{prelude::SurvivalEconomy, survival_economy}};
use crate::controllers::servers::ServerEntity;
use super::ServerType;

pub async fn get(
    path: web::Path<ServerType>, 
    data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
    let exotia_key_guard = data.exotia_key.lock()?;
    let exotia_key = &exotia_key_guard.as_ref().unwrap();

    let server = match path.into_inner() {
        ServerType::Survival => {
            SurvivalEconomy::find()
                .filter(survival_economy::Column::UserId.like(exotia_key.uuid.as_str()))
                .one(&data.conn)
                .await?
                .ok_or(ApiError::NoneValue("SurvivalEconomy User"))
        }
    };
    let user_guard = data.user.lock()?;
    let user = &user_guard.as_ref().unwrap();

    let server_entity = ServerEntity {
        server: serde_json::Value::String(serde_json::to_string(&server?)?),
        user: user.clone().clone(),
    };

    Ok(
        HttpResponse::Ok().content_type(ContentType::json()).json(json!{ server_entity })
    )
}

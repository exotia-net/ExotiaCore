use actix_web::{web, Responder, HttpResponse};
use sea_orm::{EntityTrait, QuerySelect, QueryOrder};
use serde_json::json;

use crate::{ApiError, AppState, entities::{survival_economy, users}};

use super::{ServerType, TopsQuery, EconomyTop};

/// Lists top balance in server
#[utoipa::path(
    get,
    path = "/api/servers/{server}/economy/tops",
    params(
        ("server" = ServerType, Path, description = "Type of the server"),
        ("query" = TopsQuery, Query, description = "Parameters for query")
    ),
    tag = "Servers",
    request_body(content = Vec<EconomyTop>, description = "Top balance", content_type = "application/json"),
    responses(
        (status = 200, description = "List of users"),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn economy_top(
    path: web::Path<ServerType>,
    query: web::Query<TopsQuery>,
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let tops = match path.into_inner() {
        ServerType::Survival => {
            let server_db = survival_economy::Entity::find()
                .find_also_related(users::Entity)
                .limit(query.limit)
                .order_by(survival_economy::Column::Balance, sea_orm::Order::Desc)
                .all(&*data.conn.lock().await)
                .await?;
            server_db.iter().map(|v| EconomyTop { 
                uuid: v.1.as_ref().expect("User is None").uuid,
                balance: v.0.balance
            }).collect::<Vec<EconomyTop>>()
        }
    };

    Ok(
        HttpResponse::Ok().json(json!(tops))
    )
}

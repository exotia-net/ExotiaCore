use std::sync::Arc;
use actix_web::{HttpRequest, web::Data};
use futures::lock::Mutex;
use migration::{Expr, Alias};
use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ModelTrait};

use crate::{controllers::servers::ServerType, entities::{survival_economy, users}, AppState, ApiError};

/// Updates user economy on server
#[utoipa::path(
    post,
    path = "/servers/{server}/economy/add",
    tag = "Servers (Websocket)",
    request_body(content = String, description = "POST /servers/{server}/economy/add {uuid} {balance}", content_type = "text/plain"),
    responses(
        (status = 200, description = "Succesfully updated economy"),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub async fn economy_add(server_type: ServerType, req: Arc<Mutex<HttpRequest>>, args: &Vec<String>) -> Result<String, ApiError> {
    let req_thread = Arc::clone(&req);
    let data_guard = req_thread.lock().await;
	let data: Data<AppState> = data_guard.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?.clone();

    let user = users::Entity::find()
        .filter(Expr::col(users::Column::Uuid).cast_as(Alias::new("VARCHAR")).eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
        .one(&*data.conn.lock().await)
        .await?
        .ok_or(ApiError::NoneValue("User with uuid"))?;

    let balance = match server_type {
        ServerType::Survival => {
            let server_db = user.find_related(survival_economy::Entity)
                .one(&*data.conn.lock().await)
                .await?
                .ok_or(ApiError::NoneValue("SurvivalEconomy User"))?;

            let mut server_db: survival_economy::ActiveModel = server_db.into();
            let balance = server_db.balance.unwrap() +
                args.get(1)
                    .ok_or(ApiError::NoneValue("User balance"))?
                .parse::<i32>()?;

            server_db.balance = Set(balance);
            server_db.update(&*data.conn.lock().await).await?;
            balance
        }
    };
    drop(data_guard);
	Ok(format!("{}", balance))
}

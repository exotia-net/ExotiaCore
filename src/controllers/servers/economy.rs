use actix_web::{Responder, web, HttpResponse};
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
	path: web::Path<ServerType>,
	body: web::Json<Economy>,
	data: web::Data<AppState>
) -> Result<impl Responder, ApiError> {
	let user_guard = data.user.lock()?;
	let user = &user_guard.as_ref().unwrap();

    match path.into_inner() {
        ServerType::Survival => {
            let mut server: survival_economy::ActiveModel = SurvivalEconomy::find()
                .filter(survival_economy::Column::UserId.eq(user.id))
                .one(&data.conn)
                .await?
                .ok_or(ApiError::NoneValue("SurvivalEconomy User"))?.into();

				server.balance = Set(body.balance);
				server.update(&data.conn).await?;
        }
    };

	drop(user_guard);
	Ok(
		HttpResponse::Ok().finish()
	)
}

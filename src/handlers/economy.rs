use actix_web::{HttpRequest, web::Data};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::{controllers::servers::ServerType, entities::{survival_economy, users}, AppState, ApiError};

pub async fn economy(server_type: ServerType, req: &HttpRequest, args: &Vec<String>) -> Result<String, ApiError> {

	let data: &Data<AppState> = req.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?;

    let user = users::Entity::find()
        .filter(users::Column::Uuid.eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
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

            let mut server_db: survival_economy::ActiveModel = server_db.into();
				server_db.balance = Set(
                    args.get(1)
                        .ok_or(ApiError::NoneValue("User balance"))?
                        .parse::<i32>()?
                );
				server_db.update(&data.conn).await?;
        }
    };
	Ok(String::new())
}

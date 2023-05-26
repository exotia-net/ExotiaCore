use actix_web::{HttpRequest, web::Data};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

use crate::{controllers::servers::ServerType, ApiError, AppState, entities::{users, survival_economy}};

pub async fn get_economy(server_type: ServerType, req: &HttpRequest, args: &Vec<String>) -> Result<String, ApiError> {
	let data: &Data<AppState> = req.app_data::<Data<AppState>>().ok_or(ApiError::NoneValue("AppState"))?;

    let user = users::Entity::find()
        .filter(users::Column::Uuid.eq(args.get(0).ok_or(ApiError::NoneValue("User uuid"))?))
        .one(&data.conn)
        .await?
        .ok_or(ApiError::NoneValue("User with uuid"))?;

    return match server_type {
        ServerType::Survival => {
            let server_db = survival_economy::Entity::find()
                .filter(survival_economy::Column::UserId.eq(user.id))
                .one(&data.conn)
                .await?
                .ok_or(ApiError::NoneValue("SurvivalEconomy User"))?;
            Ok(format!("{}", server_db.balance))
        }
    };
	// Ok(String::new())
}

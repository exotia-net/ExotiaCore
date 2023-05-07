use actix_web::{HttpRequest, web::Data};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};

use crate::{controllers::servers::ServerType, entities::{survival_economy, users}, AppState, ApiError};

pub async fn economy(server: ServerType, req: &HttpRequest, args: &Vec<String>) -> String {

	let data: &Data<AppState> = req.app_data::<Data<AppState>>().unwrap();

    let user = users::Entity::find()
        .filter(users::Column::Uuid.eq(args.get(0).unwrap()))
        .one(&data.conn)
        .await.unwrap()
        .ok_or(ApiError::NoneValue("User with uuid")).unwrap();

    match server {
        ServerType::Survival => {
            let mut server: survival_economy::ActiveModel = survival_economy::Entity::find()
                .filter(survival_economy::Column::UserId.eq(user.id))
                .one(&data.conn)
                .await.unwrap()
                .ok_or(ApiError::NoneValue("SurvivalEconomy User")).unwrap().into();

				server.balance = Set(args.get(1).unwrap().parse::<i32>().unwrap());
				server.update(&data.conn).await.unwrap();
        }
    };

	format!("Hello, World!")
}

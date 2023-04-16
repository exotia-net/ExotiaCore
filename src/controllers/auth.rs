use actix_web::{web, HttpRequest, HttpResponse, get};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;

use crate::{get_config, utils::token::decrypt_token, UserInfoTrait, entities::{users, prelude::*}, ApiError, AppState};

async fn validate_token(token: &str, data: &web::Data<AppState>) -> Option<crate::entities::users::Model> {
    let key = get_config().ok()?.key;
    // let encrypted_token = encrypt_token(token, &key);
    let plain_token = decrypt_token(token, &key).ok()?;
    let user_info = plain_token.extract();
    let user = Users::find()
        .filter(users::Column::Uuid.like(user_info.uuid.as_str()))
        .one(&data.conn)
        .await
        .ok()?;
	println!("{:#?}", user);
    user
}

#[utoipa::path(
	tag = "Auth",
	responses(
		(status = 200, description = "Ok"),
	)
)]
#[get("/auth/me")]
async fn auth(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer ").map(str::to_owned));
    let token_v;
    match token {
        Some(v) => token_v = v,
        None => return Ok(HttpResponse::Unauthorized().finish())
    }
    return match validate_token(&token_v, &data).await {
        Some(v) => Ok(HttpResponse::Ok().json(json!{ v })),
        None => Ok(HttpResponse::Unauthorized().finish())
    }
}

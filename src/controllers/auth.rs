use actix_web::{web, HttpResponse, get, http::header::ContentType};
use serde_json::json;

// use crate::{get_config, utils::serializers, UserInfoTrait, entities::{users, prelude::*}, ApiError, AppState};
use crate::{ApiError, AppState};

#[utoipa::path(
	tag = "Auth",
	responses(
		(status = 200, description = "Current user", body = User),
        (status = 401, description = "You are not authorized to access this resource")
	)
)]
#[get("/auth/me")]
async fn auth(data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    println!("{:#?}", data);
    Ok(
        HttpResponse::Ok().content_type(ContentType::json()).json(json!{ data.user })
    )
}

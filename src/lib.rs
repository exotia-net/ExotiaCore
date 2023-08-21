pub mod server;
pub mod websocket_handlers;
pub mod controllers;
pub mod utils;
pub mod entities;

use std::sync::PoisonError;
use std::{fs::File, io::Read, fmt, sync::Arc};
use actix_web::{HttpResponse, http::{header::ContentType, StatusCode}, body::{self, MessageBody}, web, dev::{ServiceRequest, ServiceResponse}};
use actix_web_lab::middleware::Next;
use futures::lock::Mutex;
use log::warn;
use migration::{Expr, Alias};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

use once_cell::sync::Lazy;
use entities::{users, prelude::Users};
use utils::token::decrypt;

pub static mut MINECRAFT_ADDRESS: Lazy<String> = Lazy::new(|| "127.0.0.1".to_string());
pub static mut MINECRAFT_PORT: u16 = 25565;
pub static mut DEFAULT_AUTH: Lazy<String> = Lazy::new(|| "00000000-0000-0000-0000-000000000000|0.0.0.0|0".to_string());

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: Arc<Mutex<DatabaseConnection>>,
    pub user: Arc<Mutex<Option<users::Model>>>,
    pub exotia_key: Arc<Mutex<Option<ExotiaKey>>>,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("DbError: {0}")]
    DbError(#[from] sea_orm::DbErr),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("UuidError: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("CraftpingError: {0}")]
    CraftpingError(#[from] craftping::Error),
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Poison Error")]
    PoisonError(),
    #[error("ChronoParseError: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("{0} returned None")]
    NoneValue(&'static str),
}

impl ApiError {
    fn code(&self) -> u16 {
        match *self {
            Self::DbError(_)
            | Self::IoError(_)
            | Self::UuidError(_)
            | Self::SerdeError(_)
            | Self::CraftpingError(_)
            | Self::PoisonError()
            | Self::ChronoParseError(_)
            | Self::ParseIntError(_)
            | Self::ParseFloatError(_) => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            Self::NoneValue(_) => StatusCode::NOT_FOUND.as_u16(),
        }
    }
}

impl<T> From<PoisonError<T>> for ApiError {
    fn from(_: PoisonError<T>) -> Self {
        Self::PoisonError()
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({ "message": *self.to_string() }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DbError(_)
            | Self::IoError(_)
            | Self::UuidError(_)
            | Self::SerdeError(_)
            | Self::CraftpingError(_)
            | Self::PoisonError()
            | Self::ParseIntError(_)
            | Self::ChronoParseError(_)
            | Self::ParseFloatError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoneValue(_) => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub addr: String,
    pub port: u16,
    pub threads: usize,
    pub database_table: String,
    pub database_url: String,
    pub key: String,
    pub minecraft_address: String,
    pub minecraft_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: String::from("127.0.0.1"),
            port: 3000,
            threads: 4,
            database_table: String::new(),
            database_url: String::new(),
            key: String::from("basic_key"),
            minecraft_address: String::from("127.0.0.1"),
            minecraft_port: 25565,
        }
    }
}

pub fn get_config() -> Result<Config, std::io::Error> {
    let mut file: File = File::open("config.json")?;
    let mut data: String = String::new();
    file.read_to_string(&mut data)?;
    let json: Config = serde_json::from_str(&data)?;
    Ok(json)
}

#[derive(Debug)]
pub struct ExotiaKey {
    pub uuid: Uuid,
    pub ip: String,
    pub nick: String,
}

pub trait UserInfoTrait {
    fn extract(&self) -> Result<ExotiaKey, ApiError>;
}

impl UserInfoTrait for String {
    fn extract(&self) -> Result<ExotiaKey, ApiError> {
        let val: Vec<Self> = self.split('|').map(std::borrow::ToOwned::to_owned).collect();
        Ok(
            ExotiaKey {
                uuid: Uuid::parse_str(&val[0])?,
                ip: val[1].clone(),
                nick: val[2].clone(),
            }
        )
    }
}

async fn validate_token(token: &str, data: &web::Data<AppState>) -> Option<entities::users::Model> {
    let key = get_config().ok()?.key;
    let plain_token = decrypt(token, &key).ok()?;
    let user_info = plain_token.extract().ok()?;
    let uuid = user_info.uuid;
    let mut exotia_key = data.exotia_key.lock().await;
    *exotia_key = Some(user_info);
    drop(exotia_key);

    Users::find()
        .filter(Expr::col(users::Column::Uuid).cast_as(Alias::new("VARCHAR")).eq(&uuid.to_string()))
        .one(&*data.conn.lock().await)
        .await
        .ok()?
}

pub async fn auth_middleware(
    data: web::Data<AppState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let token = req
        .headers()
        .get("ExotiaKey")
        .and_then(|value| value.to_str().ok()).map(str::to_owned);

    let Some(token_v) = token else {
        return Err(actix_web::error::ErrorUnauthorized(""));
    };
    match validate_token(&token_v, &data).await {
        Some(v) => {
            let mut user = data.user.lock().await;
            *user = Some(v);
            drop(user);
            next.call(req).await
        },
        None => {
            if req.uri() == "/auth/signUp" {
                next.call(req).await
            } else {
                Err(actix_web::error::ErrorUnauthorized(""))
            }
        }
    }
    //After Request
}

#[allow(clippy::unused_async)]
pub fn get_auth_key() -> &'static str { unsafe { DEFAULT_AUTH.as_str() } }

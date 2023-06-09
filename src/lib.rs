pub mod server;
pub mod handlers;
pub mod controllers;
pub mod utils;
pub mod entities;

use std::sync::PoisonError;
use std::{fs::File, io::Read, fmt};
use actix_web::{HttpResponse, http::{header::ContentType, StatusCode}, body::{self, MessageBody}, web, dev::{ServiceRequest, ServiceResponse}};
use actix_web_lab::middleware::Next;
use futures::lock::Mutex;
use log::warn;
use migration::{Expr, Alias};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use once_cell::sync::Lazy;
use entities::{users, prelude::Users};
use utils::token::decrypt;

pub static mut MINECRAFT_ADDRESS: Lazy<String> = Lazy::new(|| "127.0.0.1".to_string());
pub static mut MINECRAFT_PORT: u16 = 25565;
pub static mut DEFAULT_AUTH: Lazy<String> = Lazy::new(|| "00000000-0000-0000-0000-000000000000|0.0.0.0|0".to_string());

#[derive(Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub user: Mutex<Option<users::Model>>,
    pub exotia_key: Mutex<Option<ExotiaKey>>,
}

#[derive(Debug)]
pub enum ApiError {
    DbError(sea_orm::DbErr),
    IoError(std::io::Error),
    UuidError(uuid::Error),
    SerdeError(serde_json::Error),
    CraftpingError(craftping::Error),
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    PoisonError(),
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
            | Self::ParseIntError(_)
            | Self::ParseFloatError(_) => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            Self::NoneValue(_) => StatusCode::NOT_FOUND.as_u16(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DbError(v) => {
                warn!("DbError: {:?}", v);
                write!(f, "DbError")
            }
            Self::IoError(v) => {
                warn!("IoError: {:?}", v);
                write!(f, "IoError")
            }
            Self::UuidError(v) => {
                warn!("UuidError: {:?}", v);
                write!(f, "UuidError")
            }
            Self::SerdeError(v) => {
                warn!("SerdeError: {:?}", v);
                write!(f, "SerdeError")
            }
            Self::CraftpingError(v) => {
                warn!("CraftpingError: {:?}", v);
                write!(f, "CraftpingError")
            }
            Self::ParseIntError(v) => {
                warn!("ParseIntError: {:?}", v);
                write!(f, "ParseIntError")
            }
            Self::ParseFloatError(v) => {
                warn!("ParseFloatError: {:?}", v);
                write!(f, "ParseFloatError")
            }
            Self::PoisonError() => {
                write!(f, "PoisonError")
            }
            Self::NoneValue(v) => {
                warn!("{:?} returned None", v);
                write!(f, "Value {v} is None")
            }
        }
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::DbError(value)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<uuid::Error> for ApiError {
    fn from(value: uuid::Error) -> Self {
        Self::UuidError(value)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<craftping::Error> for ApiError {
    fn from(value: craftping::Error) -> Self {
        Self::CraftpingError(value)
    }
}

impl From<std::num::ParseIntError> for ApiError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<std::num::ParseFloatError> for ApiError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(value)
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
            .json(json!({ "message": self.to_string() }))
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
        .one(&data.conn)
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

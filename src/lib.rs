pub mod server;
pub mod handlers;
pub mod controllers;
pub mod utils;
pub mod entities;

use std::{fs::File, io::Read, fmt, sync::{Mutex, PoisonError}};
use actix_web::{HttpResponse, http::header::ContentType, body};
use log::warn;
use reqwest::StatusCode;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::json;

use once_cell::sync::Lazy;
use entities::users;

pub static mut MINECRAFT_ADDRESS: Lazy<String> = Lazy::new(|| "127.0.0.1".to_string());
pub static mut MINECRAFT_PORT: u16 = 25565;

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
    SerdeError(serde_json::Error),
    PoisonError(),
    NoneValue(&'static str),
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
            Self::SerdeError(v) => {
                warn!("SerdeError: {:?}", v);
                write!(f, "SerdeError")
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

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
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

    fn status_code(&self) -> reqwest::StatusCode {
        match *self {
            Self::DbError(_)
            | Self::IoError(_)
            | Self::SerdeError(_)
            | Self::PoisonError() => StatusCode::INTERNAL_SERVER_ERROR,
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

#[allow(unused)]
#[derive(Debug)]
pub struct ExotiaKey {
    pub uuid: String,
    pub ip: String,
    pub nick: String,
}

pub trait UserInfoTrait {
    fn extract(&self) -> ExotiaKey;
}

impl UserInfoTrait for String {
    fn extract(&self) -> ExotiaKey {
        let val: Vec<Self> = self.split('|').map(std::borrow::ToOwned::to_owned).collect();
        ExotiaKey {
            uuid: val[0].clone(),
            ip: val[1].clone(),
            nick: val[2].clone(),
        }
    }
}

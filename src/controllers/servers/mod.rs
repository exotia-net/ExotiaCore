pub mod get;

use serde::{Serialize, Deserialize};
use crate::entities::users;

#[derive(Deserialize)]
pub enum ServerType {
    Survival,
}

#[derive(Serialize)]
pub struct ServerEntity {
    server: serde_json::Value,
    user: users::Model,
}
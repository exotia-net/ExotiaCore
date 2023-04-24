pub mod get;
pub mod economy;

use serde::{Serialize, Deserialize};
use crate::entities::users;

#[derive(Deserialize, Debug)]
pub enum ServerType {
    Survival,
}

#[derive(Deserialize)]
pub struct Economy {
    balance: i32,
}

#[derive(Serialize)]
pub struct ServerEntity {
    server: serde_json::Value,
    user: users::Model,
}
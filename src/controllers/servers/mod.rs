pub mod get;
pub mod economy;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum ServerType {
    Survival,
}

#[derive(Deserialize)]
pub struct Economy {
    balance: i32,
}

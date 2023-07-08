use std::sync::Arc;
use actix_web::HttpRequest;
use futures::lock::Mutex;
// use wildmatch::WildMatch;
use crate::{controllers::servers::ServerType, ApiError};

pub mod public;
pub mod servers;
pub mod wallet;
pub mod calendars;

#[must_use]
pub async fn handle_command(cmd: (String, String), kwargs: Vec<String>, req: HttpRequest) -> Result<String, ApiError> {
    let cmd: (&str, &str) = (&cmd.0, &cmd.1);
	let args = kwargs.iter().map(std::borrow::ToOwned::to_owned).collect();

	match cmd {
        // Public
		("GET", "/public/online") => public::get_online(),
		("GET", "/public/servers") => todo!(),
		("GET", "/public/cosmetics") => todo!(),
        
        // Calendars
        ("GET", "/calendars") => calendars::get(&req, &args).await,
        ("PUT", "/calendars") => calendars::update(&req, &args).await,
        ("GET", "/calendars/rewards") => calendars::rewards(&req, &args).await,

        // Servers
        // _ if WildMatch::new("/servers/*/economy").matches(cmd.as_str()) => economy::economy(ServerType::Survival, &req, &args).await,
		("POST", "/servers/Survival/economy") => servers::economy(ServerType::Survival, Arc::new(Mutex::new(req)), &args).await,
		("POST", "/servers/Survival/economy/add") => servers::economy(ServerType::Survival, Arc::new(Mutex::new(req)), &args).await,
        ("GET", "/servers/Survival/economy") => servers::get(ServerType::Survival, &req, &args).await,

        // Wallet
		("GET", "/wallet") => wallet::get(&req, &args).await,
		("GET", "/wallet/buy") => wallet::buy(&req, &args).await,

        // Tops
        // ("GET", "/tops/user") => tops::get(&req, &args).await,
        // ("POST", "/tops/user") => tops::get(&req, &args).await,

		(&_, &_) => Ok(String::new())
	}
}

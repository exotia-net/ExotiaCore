use actix_web::HttpRequest;
// use wildmatch::WildMatch;
use crate::{controllers::servers::ServerType, ApiError};

pub mod get_online;
pub mod economy;
pub mod get_economy;
pub mod get_wallet;
pub mod wallet_buy;

#[must_use]
pub async fn handle_command(cmd: (String, String), kwargs: Vec<String>, req: HttpRequest) -> Result<String, ApiError> {
    let cmd: (&str, &str) = (&cmd.0, &cmd.1);
	let args = kwargs.iter().map(|v| v.to_owned()).collect();

	match cmd {
		("GET", "/public/online") => Ok(get_online::get_online()),
		("GET", "/public/servers") => todo!(),
		("GET", "/public/cosmetics") => todo!(),
        // _ if WildMatch::new("/servers/*/economy").matches(cmd.as_str()) => economy::economy(ServerType::Survival, &req, &args).await,
		("POST", "/servers/Survival/economy") => economy::economy(ServerType::Survival, &req, &args).await,
        ("GET", "/servers/Survival/economy") => get_economy::get_economy(ServerType::Survival, &req, &args).await,
		("GET", "/wallet") => get_wallet::get_wallet(&req, &args).await,
		("GET", "/wallet/buy") => wallet_buy::wallet_buy(&req, &args).await,
		(&_, &_) => Ok(String::new())
	}
}

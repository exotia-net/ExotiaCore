use actix_web::HttpRequest;
use crate::{controllers::servers::ServerType, ApiError};

pub mod get_online;
pub mod economy;

#[must_use]
pub async fn handle_command(cmd: String, kwargs: Vec<String>, req: HttpRequest) -> Result<String, ApiError> {
	let args = kwargs.iter().map(|v| v.to_owned()).collect();

	match cmd.as_str() {
		"/public/online" => Ok(get_online::get_online()),
		"/public/servers" => todo!(),
		"/public/cosmetics" => todo!(),
		"/servers/Survival/economy" => economy::economy(ServerType::Survival, &req, &args).await,
		&_ => Ok(String::new())
	}
}

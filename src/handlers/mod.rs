use actix_web::HttpRequest;
use futures::executor::block_on;

use crate::controllers::servers::ServerType;

pub mod get_online;
pub mod economy;

#[must_use]
pub fn handle_command(cmd: &str, kwargs: Vec<&str>, req: &HttpRequest) -> String {
	let args = kwargs.iter().map(|&v| v.to_owned()).collect();

	match cmd {
		"/public/online" => get_online::get_online(),
		"/public/servers" => todo!(),
		"/public/cosmetics" => todo!(),
		"/servers/Survival/economy" => {
			let future = economy::economy(ServerType::Survival, &req, &args);
			let val = block_on(future);
			val.to_owned()
		},
		&_ => String::new()
	}
}

use std::net::TcpStream;
use craftping::sync::ping;

use crate::{MINECRAFT_ADDRESS, MINECRAFT_PORT, ApiError};

/// Returns currently connected users
#[utoipa::path(
    get,
    path = "/public/online",
    tag = "Public (Websocket)",
    request_body(content = String, description = "GET /public/online", content_type = "text/plain"),
    responses(
        (status = 200, description = "Returns", body = lib::entities::wallet::Model),
        (status = 401, description = "You are not authorized to access this resource"),
		(status = 404, description = "If value is none"),
		(status = 500, description = "Database error"),
    )
)]
pub fn get_online() -> Result<String, ApiError> {
    unsafe {
        let addr: &str = &MINECRAFT_ADDRESS;
        let mut stream = TcpStream::connect((addr, MINECRAFT_PORT))?;
        let res = ping(&mut stream, &MINECRAFT_ADDRESS, MINECRAFT_PORT)?;
    	Ok(format!("{}/{}", res.online_players, res.max_players))
    }
}

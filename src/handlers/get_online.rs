use std::net::TcpStream;
use craftping::sync::ping;

use crate::{MINECRAFT_ADDRESS, MINECRAFT_PORT};

pub fn get_online() -> String {
    unsafe {
        let addr: &str = &MINECRAFT_ADDRESS;
        let mut stream = TcpStream::connect((addr, MINECRAFT_PORT)).unwrap();
        let res = ping(&mut stream, &MINECRAFT_ADDRESS, MINECRAFT_PORT).expect("Cannot ping server");
    	format!("{}/{}", res.online_players, res.max_players)
    }
}

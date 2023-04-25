mod get_online;

#[must_use]
pub fn handle_command(cmd: &str, _kwargs: Vec<&str>) -> String {
	// let _args = kwargs.iter().map(|&v| v.to_owned()).collect();
	match cmd {
		"/public/online" => get_online::get_online(),
		"/public/servers" => todo!(),
		"/public/cosmetics" => todo!(),
		&_ => String::new()
	}
}

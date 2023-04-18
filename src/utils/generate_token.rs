use rand::Rng;

pub fn make_token(length: i32, use_special_chars: Option<bool>) -> String {
	let charset: &[u8];
	if use_special_chars.unwrap_or_default() {
		charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()[]{};':\",./<>?`"; 
	} else {
		charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	}

	let mut rng = rand::thread_rng();

	let token: String = (0..length)
		.map(|_| {
			let idx = rng.gen_range(0..charset.len());
			charset[idx] as char
		}).collect();
	token
}

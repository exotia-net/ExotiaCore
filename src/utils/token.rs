use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::Rng;

pub fn decrypt(token: &str, key: &str) -> Result<String, magic_crypt::MagicCryptError> {
    let mc = new_magic_crypt!(key, 256);
    
    mc.decrypt_base64_to_string(token)
}

pub fn encrypt(token: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    
    mc.encrypt_str_to_base64(token)
}

pub fn create(length: i32, use_special_chars: Option<bool>) -> String {
	let charset: &[u8] = if use_special_chars.unwrap_or_default() {
		b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()[]{};':\",./<>?`" 
	} else {
		b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
	};

	let mut rng = rand::thread_rng();

	let token: String = (0..length)
		.map(|_| {
			let idx = rng.gen_range(0..charset.len());
			charset[idx] as char
		}).collect();
	token
}

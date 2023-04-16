use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn decrypt_token(token: &str, key: &str) -> Result<String, magic_crypt::MagicCryptError> {
    let mc = new_magic_crypt!(key, 256);
    let base64 = mc.decrypt_base64_to_string(token);
    base64
}

pub fn encrypt_token(token: &str, key: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    let base64 = mc.encrypt_str_to_base64(token);
    base64
}

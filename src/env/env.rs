#[allow(unused)]
pub fn get_env_or(key: &str, default: &str) -> String {
    return match std::env::var(key) {
        Ok(v) => v.to_string(),
        Err(_) => default.to_string(),
    };
}

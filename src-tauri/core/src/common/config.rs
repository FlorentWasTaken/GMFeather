use dotenvy::dotenv;
use std::env;
use tracing::{info, warn};

pub fn init_config() {
    match dotenv() {
        Ok(path) => info!("Loaded .env from {:?}", path),
        Err(_) => warn!("No .env file found, using system environment variables"),
    }
}

pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_env_var() {
        let key = "TEST_VAR_GMFEATHER";
        let value = "test_value";
        env::set_var(key, value);

        assert_eq!(get_env_var(key), Some(value.to_string()));

        env::remove_var(key);
        assert_eq!(get_env_var(key), None);
    }
}

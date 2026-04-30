pub fn greet_user(name: &str) -> String {
    format!("Hello, {}! You've been greeted from the Core!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(
            greet_user("Test"),
            "Hello, Test! You've been greeted from the Core!"
        );
    }
}

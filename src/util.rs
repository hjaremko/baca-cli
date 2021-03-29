pub fn get_env(var_name: &str) -> Option<String> {
    use std::env;

    match env::var(var_name) {
        Ok(token) => Some(token),
        Err(_) => {
            tracing::warn!("Variable {} is not present in the environment!", var_name);
            None
        }
    }
}

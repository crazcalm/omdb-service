use std::path::Path;

pub async fn load_config(env: String) -> Result<std::path::PathBuf, dotenv::Error> {
    let config_path = Path::new("config").join(env);
    dotenv::from_filename(config_path)
}

#[cfg(test)]
mod tests {
    use crate::config;
    use std::env;

    #[tokio::test]
    async fn test_load_config() {
        config::load_config("test".to_string()).await.unwrap();

        assert_eq!("test_name", env::var("NAME").unwrap());
    }
}

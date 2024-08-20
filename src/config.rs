use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub namespace: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let settings = Config::builder()
            .add_source(File::with_name("src/config"))
            .add_source(Environment::default().separator("__"))
            .build()?;

        settings.try_deserialize()
    }
}

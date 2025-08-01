use std::sync::LazyLock;
use anyhow::Context;
use config::Config;
use serde::Deserialize;

mod server;

pub use server::ServerConfig;

static CONFIG:LazyLock<AppConfig> = LazyLock::new(|| {
    AppConfig::load().expect("Failed to load application config")
});

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
}

impl AppConfig {

    pub fn load() -> anyhow::Result<AppConfig> {
        Config::builder()
            .add_source(
                config::File::with_name("config/application")
                    .format(config::FileFormat::Yaml)
                    .required(true)
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(",")
            )
            .build()
            .with_context(|| "Could not load application config".to_string())?
            .try_deserialize()
            .with_context(|| "Could not deserialize config".to_string())
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
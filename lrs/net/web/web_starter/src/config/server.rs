use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
}

impl ServerConfig {
    const DEFAULT_PORT: u16 = 3000;

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(Self::DEFAULT_PORT)
    }
}
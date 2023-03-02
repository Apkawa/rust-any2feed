use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct HttpServerConfig {
    pub port: Option<u16>,
    pub threads: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct MainConfig {
    pub server: HttpServerConfig,
}

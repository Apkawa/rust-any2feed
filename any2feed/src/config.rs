use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct MainConfig {
    pub port: Option<u16>
}
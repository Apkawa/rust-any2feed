use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mewe: MeweConfig
}

#[derive(Debug, Deserialize)]
pub struct MeweConfig {
    pub cookies_path: String,
    pub limit: Option<usize>,
    pub pages: Option<usize>,
}

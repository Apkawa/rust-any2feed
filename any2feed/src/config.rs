use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

use crate::cli::{Commands, CLI};
use crate::feed_sources::{FeedSourceList, FeedSourceManager};
use crate::logging;

#[derive(Debug, Default, Deserialize)]
pub struct HttpServerConfig {
    pub port: Option<u16>,
    pub threads: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct MainConfig {
    // Make optional
    pub server: HttpServerConfig,
    pub verbose: Option<u8>,
    pub log_file: Option<PathBuf>,
    pub config_text: Option<String>,

    #[serde(flatten)]
    pub feed_sources: HashMap<String, FeedSourceOption>,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct FeedSourceOption {
    pub disable: Option<bool>,
}

impl MainConfig {
    fn load(config_str: &str) -> MainConfig {
        let mut config: MainConfig = toml::from_str(&config_str).unwrap();
        config.config_text = Some(config_str.to_string());
        config
    }

    fn merge_with_cli(mut self, cli: &CLI) -> Self {
        match &cli.command {
            Commands::Run(server_cfg) => {
                self.server.port = server_cfg.port;
                self.server.threads = server_cfg.threads;
            }
        }
        // Флаг выставлен в cmd
        if cli.verbose > 0 {
            self.verbose = Some(cli.verbose);
        }
        self.log_file = cli.log_file.clone().or(self.log_file).clone();
        self
    }

    pub fn get_enabled_feed_sources(&self) -> FeedSourceList {
        let sources = FeedSourceManager::get_sources().into_iter().filter(|s| {
            self.feed_sources
                .get(s.name().as_str())
                .map_or(false, |f_o| !f_o.disable.unwrap_or(false))
        });
        sources.collect()
    }
}

pub fn load_config_from_args<I, T>(args: I) -> MainConfig
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = CLI::parse_from(args);
    // TODO find config in other locations
    let config_str = read_to_string(&cli.config).unwrap();
    let config: MainConfig = MainConfig::load(&config_str).merge_with_cli(&cli);

    logging::logging_init(&config);
    log::debug!("CLI: {:?}", &cli);
    log::debug!("CONFIG: {:?}", &config);

    config
}

/// load config and initialize logging
pub fn load_config() -> MainConfig {
    load_config_from_args(env::args_os())
}

#[cfg(test)]
mod tests {
    use std::fs::write;

    use super::*;

    #[test]
    fn test_load_config() {
        let config_str = r#"
verbose = 1
# log_file = './log.log'

[server]
port = 1234
threads = 5
        "#;
        write("/tmp/config.toml", config_str).unwrap();
        let args = "any2feed --config /tmp/config.toml run --port 123 --threads 10".split(' ');
        let config = load_config_from_args(args);
        assert_eq!(config.config_text, Some(config_str.to_string()));
        assert_eq!(config.verbose, Some(1));
        assert_eq!(config.server.port, Some(123));
        assert_eq!(config.server.threads, Some(10));
    }

    #[test]
    fn test_feed_sources() {
        let config_str = r#"
        [server] # TODO make optional
        [telegram]
        [telegram.foo_bar] # ignored
        [mewe]
        disable = true
        [dhfdhsfjhj] # Non exist source
        "#;
        let c = MainConfig::load(config_str);
        dbg!(&c);
        assert_eq!(c.feed_sources.get("mewe").unwrap().disable, Some(true));
        assert_eq!(c.feed_sources.get("telegram").unwrap().disable, None);
        let fs = c.get_enabled_feed_sources();
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].name(), "telegram".to_string());
    }

    #[test]
    fn test_feed_sources_disabled() {
        let config_str = r#"
        [server] # TODO make optional
        [telegram]
        # [mewe] disabled
        "#;
        let c = MainConfig::load(config_str);
        dbg!(&c);
        assert_eq!(c.feed_sources.get("mewe"), None);
        assert_eq!(c.feed_sources.get("telegram").unwrap().disable, None);
        let fs = c.get_enabled_feed_sources();
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].name(), "telegram".to_string());
    }
}

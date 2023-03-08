use crate::cli::{Commands, CLI};
use crate::logging;
use clap::Parser;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct HttpServerConfig {
    pub port: Option<u16>,
    pub threads: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct MainConfig {
    pub server: HttpServerConfig,
    pub verbose: Option<u8>,
    pub log_file: Option<PathBuf>,

    pub config_text: Option<String>,
}

pub fn load_config() -> MainConfig {
    let cli = CLI::parse();
    // TODO find config in other locations
    let config_str = read_to_string(&cli.config).unwrap();
    let mut config: MainConfig = toml::from_str(&config_str).unwrap();
    config.config_text = Some(config_str);
    match &cli.command {
        Commands::Run(server_cfg) => {
            config.server.port = server_cfg.port;
            config.server.threads = server_cfg.threads;
        }
    }
    // Флаг выставлен в cmd
    if cli.verbose > 0 {
        config.verbose = Some(cli.verbose);
    }
    config.log_file = cli.log_file.clone().or(config.log_file).clone();

    logging::logging_init(&config);
    log::debug!("CLI: {:?}", &cli);
    log::debug!("CONFIG: {:?}", &config);

    config
}

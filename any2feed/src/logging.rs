use std::fs::File;

use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};

use crate::config::MainConfig;

pub fn logging_init(config: &MainConfig) {
    let log_level = match config.verbose.unwrap_or(0) {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    let log_config = ConfigBuilder::default()
        // Mute too verbosity crate
        .add_filter_ignore_str("selectors")
        .add_filter_ignore_str("html5ever")
        .add_filter_ignore_str("reqwest")
        .build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];
    if let Some(path) = config.log_file.as_ref() {
        let parent = path.parent().unwrap();
        if parent.exists() && parent.is_dir() {
            // to file
            loggers.push(WriteLogger::new(
                log_level,
                log_config.clone(),
                File::create(path).unwrap(),
            ));
            // and critical error write to stderr
            loggers.push(TermLogger::new(
                LevelFilter::Error,
                log_config.clone(),
                TerminalMode::Stderr,
                ColorChoice::Auto,
            ));
        } else {
            panic!("Incorrect `log_path`!");
        }
    } else {
        // stdout/stderr
        loggers.push(TermLogger::new(
            log_level,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ));
    }

    CombinedLogger::init(loggers).unwrap();
}

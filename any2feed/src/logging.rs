use crate::cli::CLI;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

pub fn logging_init(cli: &CLI) {
    let log_level = match cli.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    let config = ConfigBuilder::default()
        // Mute too verbosity crate
        .add_filter_ignore_str("selectors")
        .add_filter_ignore_str("html5ever")
        .add_filter_ignore_str("reqwest")
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(log_level, config, TerminalMode::Mixed, ColorChoice::Auto),
        // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
    ])
    .unwrap();
}

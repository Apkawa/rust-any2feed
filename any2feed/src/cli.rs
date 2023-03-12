use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::ErrorKind;
use clap::{Arg, Args, Command, Error, Parser, Subcommand};

use crate::feed_sources::FeedSourceManager;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "any2feed")]
#[command(bin_name = "any2feed")]
pub struct CLI {
    #[arg(short, long)]
    /// Path to config.toml. See any2feed_config_example.toml
    pub config: PathBuf,

    #[arg(short, long, action = clap::ArgAction::Count)]
    /// Verbosity log debug
    pub verbose: u8,

    #[arg(long)]
    /// Write log to file
    pub log_file: Option<PathBuf>,

    #[arg(long, value_delimiter=',', value_parser=FeedSourceValueParser::new())]
    /// Feed sources
    pub feed_source: Option<Vec<String>>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start http server
    Run(RunServer),
}

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct RunServer {
    /// Server listen port
    #[arg(short, long)]
    pub port: Option<u16>,
    /// Server num threads
    #[arg(long)]
    pub threads: Option<u8>,
}

#[derive(Clone)]
struct FeedSourceValueParser(Vec<PossibleValue>);

impl FeedSourceValueParser {
    fn new() -> Self {
        let sources: Vec<PossibleValue> = FeedSourceManager::get_sources()
            .into_iter()
            .map(|s| PossibleValue::from(s.name()))
            .collect();
        FeedSourceValueParser(sources)
    }
}

impl TypedValueParser for FeedSourceValueParser {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, Error> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        cmd: &Command,
        _arg: Option<&Arg>,
        value: OsString,
    ) -> Result<Self::Value, Error> {
        let value = value.into_string().unwrap();
        if self.0.iter().any(|v| v.matches(&value, false)) {
            Ok(value)
        } else {
            let possible_vals = self
                .0
                .iter()
                .filter(|v| !v.is_hide_set())
                .map(|v| v.get_name().to_owned())
                .collect::<Vec<_>>();
            {
                let mut cmd = cmd.to_owned();
                Err(cmd.error(
                    ErrorKind::InvalidValue,
                    format!(r#""{value}" not in {possible_vals:?}!"#),
                ))
            }
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(self.0.iter().cloned()))
    }
}

#[cfg(test)]
mod tests {
    use super::Commands::*;
    use super::*;

    #[test]
    fn test_cli_parse() {
        let args = "any2feed --config /tmp/config.toml run --port 123 --threads 10".split(' ');
        let cli = CLI::try_parse_from(args).unwrap();
        assert_eq!(cli.config, PathBuf::from("/tmp/config.toml"));
        assert_eq!(cli.verbose, 0);
        assert_eq!(cli.log_file, None);

        if let Run(server) = cli.command {
            assert_eq!(server.threads, Some(10));
            assert_eq!(server.port, Some(123));
        }
    }
    #[test]
    fn test_feed_source() {
        let args = "any2feed --config /tmp/config.toml --feed-source mewe,telegram run".split(' ');
        let cli = CLI::try_parse_from(args).unwrap();
        assert_eq!(
            cli.feed_source,
            Some(vec!["mewe".to_string(), "telegram".to_string()])
        )
    }

    #[test]
    fn test_feed_source_invalid_name() {
        let args = "any2feed --config /tmp/config.toml --feed-source mewe,telegram --feed-source foobar run".split(' ');
        let err = CLI::try_parse_from(args).unwrap_err();
        assert_eq!(
            err.render().to_string().lines().take(1).collect::<String>(),
            r#"error: "foobar" not in ["mewe", "telegram"]!"#
        )
    }
}

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)] // requires `derive` feature
#[command(name = "any2feed")]
#[command(bin_name = "any2feed")]
pub struct CLI {
    #[arg(short, long)]
    /// Path to config.toml. See any2feed_config_example.toml
    pub config: PathBuf,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start http server
    Run(RunServer),
}

#[derive(Args)]
#[command(author, version, about, long_about = None)]
pub struct RunServer {
    /// Server listen port
    #[arg(short, long)]
    pub port: Option<u16>,
    /// Server num threads
    #[arg(long)]
    pub threads: Option<u8>,
}

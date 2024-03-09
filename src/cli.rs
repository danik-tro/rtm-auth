use std::net::SocketAddr;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
    /// File with configuration for the server
    #[arg(short, long)]
    pub config: String,
    /// Workers
    #[arg(long = "workers", default_value_t = num_cpus::get())]
    pub workers: usize,
    /// Do not print log messages
    #[arg(long, short, default_value_t = false)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run server with provided configuration
    RunServer {
        /// Bind server to the url
        #[arg(short, long)]
        bind: SocketAddr,
    },
}

/// Parse cli params
#[must_use]
pub fn parse() -> Cli {
    Cli::parse()
}

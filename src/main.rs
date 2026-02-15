#![allow(dead_code)]

mod client;
mod config;
mod daemon;
mod error;
mod ipc;
mod llm;
mod protocol;
mod server;
mod shell;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use crate::config::{Mode, RuntimeConfig};

#[derive(Parser, Debug)]
#[command(
    name = "bb",
    version,
    about = "Natural language to shell command helper"
)]
struct Cli {
    #[arg(long)]
    daemon: bool,

    #[arg(long)]
    ping: bool,

    #[arg(long)]
    socket: Option<PathBuf>,

    #[arg(long = "pid-file")]
    pid_file: Option<PathBuf>,

    #[arg(long)]
    shell: Option<ShellKind>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ShellKind {
    Bash,
    Zsh,
}

impl From<ShellKind> for crate::shell::ShellKind {
    fn from(value: ShellKind) -> Self {
        match value {
            ShellKind::Bash => crate::shell::ShellKind::Bash,
            ShellKind::Zsh => crate::shell::ShellKind::Zsh,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let cli = Cli::parse();
    let mode = if cli.daemon {
        Mode::Daemon
    } else {
        Mode::Client
    };
    let config = RuntimeConfig::new(
        mode,
        cli.ping,
        cli.socket,
        cli.pid_file,
        cli.shell.map(Into::into),
    )?;

    match config.mode {
        Mode::Client => client::run_client(&config).await,
        Mode::Daemon => server::run_daemon(&config).await,
    }
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("bb=info")),
        )
        .try_init();
}

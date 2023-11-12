use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Monitor profile management and automatic switching for Linux desktops.
#[derive(Debug, Parser)]
#[command(author, version)]
pub(crate) struct Arguments {
    /// Path to the monitor profile configuration directory.
    ///
    /// Defaults to $XDG_CONFIG_HOME/monprof.
    /// The directory will be created if it does not exist.
    #[arg(short = 'p', long = "profiles-directory")]
    pub(crate) profile_directory: Option<PathBuf>,

    /// Disable log messages.
    #[arg(short = 'q', long = "quiet", group = "log_verbosity")]
    pub(crate) quiet: bool,

    /// Enable verbose log messages.
    #[arg(short = 'v', long = "verbose", group = "log_verbosity")]
    pub(crate) verbose: bool,

    /// Enable output as JSON. Implies quiet mode.
    #[arg(short = 'j', long = "json")]
    pub(crate) json_output: bool,

    /// Subcommand to be executed.
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// List currently connected monitors.
    Monitors,
}

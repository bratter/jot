use std::path::PathBuf;

use clap::Parser;

/// A simple CLI app to help plain-text note-taking.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// Initial text to capture in the note.
    pub text: Option<String>,

    /// Jot will open $EDITOR unless this flag is set
    #[arg(short, long)]
    pub no_edit: bool,

    /// Use the config file at the specified path instead of the default
    #[arg(long)]
    pub config: Option<PathBuf>,
}

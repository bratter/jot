use std::path::PathBuf;

use clap::{Args as ClapArgs, Parser, Subcommand as ClapSubcommand};

/// A simple CLI app to help plain-text note-taking.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// Initial text to capture in the note.
    pub text: Option<String>,

    /// Put the jot in an alternative subdirectory.
    #[arg(short, long)]
    pub subdir: Option<String>,

    /// Jot will open $EDITOR unless this flag is set.
    #[arg(short = 'e', long = "no-edit")]
    pub no_edit: bool,

    /// Force creation of the file even if text is blank and no-edit is set.
    #[arg(short, long)]
    pub force: bool,

    /// Use the config file at the specified path instead of the default.
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, ClapSubcommand)]
pub enum Subcommand {
    /// Render a note at the given path as HTML.
    ///
    /// Outputs to stdout unless the -o option is passed
    Html(HtmlCmd),

    /// Render a note at the given path as a PDF file using headless chrome.
    ///
    /// Headless chrome must be available on the system for this to work. Requires the output
    /// option to send to a file.
    Pdf(PdfCmd),
}

/// Command to render a note as HTML from the give path.
#[derive(Debug, ClapArgs)]
pub struct HtmlCmd {
    /// Path to the file to convert. The file must have a .md extension.
    pub input: PathBuf,

    /// Output to a file at the given path, stdout otherwise. The folder must exist, but the file
    /// must not for this to succeed.
    pub output: Option<PathBuf>,
}

#[derive(Debug, ClapArgs)]
pub struct PdfCmd {
    /// Path to the file to convert. The file must have a .md extension.
    pub input: PathBuf,

    /// Output the file to the given path. The folder must exist, but the destination file must not
    /// for this to work.
    pub output: PathBuf,
}

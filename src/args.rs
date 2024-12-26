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
    ///
    /// If not provided will take input from stdin.
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output to a file at the given path.
    ///
    /// If not provided, will output to stdout.
    ///
    /// The location of the output file depends on the value provided:
    /// - If the flag is provided without text, the resulting file will be saved with a .pdf extension and the same file
    ///   stem next to the original.
    /// - Other directories are saved in the directory if it exists (none are created) with the same filename. A dot
    ///   will therefore save in the pwd.
    /// - A full filename will save to the file, but will not create any directories or overwrite existing files.
    #[arg(short, long)]
    pub output: Option<Option<PathBuf>>,

    /// Output only the raw markdown without wrapping HTML.
    ///
    /// This is useful for producing snippets.
    #[arg(short, long)]
    pub raw: bool,
}

#[derive(Debug, ClapArgs)]
pub struct PdfCmd {
    /// Path to the file to convert. The file must have a .md extension.
    ///
    /// If not provided will take input from stdin.
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output the file to the given path.
    ///
    /// If not provided, will output to stdout. This should only be used to redirect to a file.
    ///
    ///  The location of the output file depends on the value provided:
    ///  - If the flag is provided without any text then the resulting file is saved with a .pdf extension next to the
    ///    source file.
    ///  - Other directories are saved in the directory if it exists (none are created) with the same filename. A dot
    ///    will therefore save in the pwd.
    ///  - A full filename will save to the file, but will not create any directories or overwrite existing files.
    #[arg(short, long)]
    pub output: Option<Option<PathBuf>>,
}

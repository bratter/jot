//! Jot: A simple aid to help live the plain text life
//!
//! Provides a minimalist CLI interface to managed Markdown formatted notes. Inspired by:
//!
//! - [Noted](https://github.com/scottashipp/noted)
//! - [Note-Taking in Vanilla Vim](https://www.edwinwenink.xyz/posts/42-vim_notetaking/)

mod args;
mod commands;
mod config;
mod html;
mod path;

use anyhow::Result;
use clap::Parser;

use crate::args::{Args, Subcommand};
use crate::config::Config;

fn main() -> Result<()> {
    let args = Args::parse();
    let config = Config::try_new(&args)?;

    // Route based on the command
    match args.subcommand {
        Some(Subcommand::Html(args)) => commands::render_html(&args),
        Some(Subcommand::Pdf(args)) => commands::render_pdf(&args),
        None => commands::create(&args, &config),
    }
}

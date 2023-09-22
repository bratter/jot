mod args;
mod commands;
mod config;
mod error;

use clap::Parser;

use crate::args::{Args, Subcommand};
use crate::config::Config;
use crate::error::Error;

// TODO: Consider supporting custom template strings for front matter that will need replace (regex)
//       or a dynamic template engine (strfmt) to work
// TODO: Get Serde out of the compilation. Is there some feature in chrono/toml that needs to be
//       disabled?
// TODO: Support resolution of env vars in appropriate config items
// TODO: In the config can we make args 'static, so we don't have to allocate strings?
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config = Config::try_new(&args)?;

    // Route based on the command
    match args.subcommand {
        Some(Subcommand::Html(args)) => commands::html(&args),
        None => commands::create(&args, &config),
    }
}

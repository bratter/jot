mod args;
mod config;
mod error;

use chrono::prelude::*;
use clap::Parser;
use std::{fs, process::Command};

use crate::args::Args;
use crate::config::Config;
use crate::error::Error;

// TODO: Move the text processing to its own module and...
//  - Consider supporting custom template strings for front matter that will need replace (regex)
//    or a dynamic template engine (strfmt) to work
// TODO: Get Serde out of the compilation. Is there some feature in chrono/toml that needs to be
// disabled?
// TODO: Support resolution of env vars in appropriate config items
// TODO: In the config can we make args 'static, so we don't have to allocate strings?
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config = Config::try_new(&args)?;

    // Abort early if attempting to create an empty note without editing
    // when force is not also set
    if args.no_edit && !args.force && args.text.is_none() {
        eprintln!("No edit was set and no text was provided. Aborting.");
        return Ok(());
    }

    // Get the date, including the year and month for building the path and for the front matter
    let date = Local::now();

    // Then process the provided text, adding a heading (#) to the front if required, then push
    // onto the text
    let mut text = format!(
        "---\ntimestamp: {}\ngenerated-by: {} {}\n---\n\n",
        date.format("%FT%T%:z"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let body_text = args.text.unwrap_or_default();
    let body_text = body_text.trim();
    let heading_leader = if body_text.starts_with("#") || body_text.len() == 0 {
        ""
    } else {
        "# "
    };
    text.push_str(heading_leader);
    text.push_str(body_text);
    text.push('\n');

    // For the location to save, start with the base notes folder to add to
    let mut note_path = config.base_dir();

    // Ensure all the folders are created
    note_path.push(date.format("%Y/%m").to_string());
    fs::create_dir_all(&note_path).map_err(|err| Error::IO(err))?;

    // Now write out the file
    let filename = format!("{}.md", date.format("%Y%m%d_%H%M%S"));
    note_path.push(&filename);
    fs::write(&note_path, &text).map_err(|err| Error::IO(err))?;

    // Editing behavior:
    // - Require an editor to be identified in config (first) or path
    // - File is created whether editor can be opened or not
    // - Attempt to open the file for editing as long as no-edit is not set
    // - Otherwise print the full path of the created file
    if args.no_edit {
        println!("{}", note_path.to_string_lossy());
    } else {
        let mut cmd = Command::new(config.editor);
        cmd.arg(&note_path);

        if config.jump {
            cmd.arg(format!("+"));
        }

        cmd.spawn()
            .map_err(|err| Error::IO(err))?
            .wait()
            .map_err(|err| Error::IO(err))?;
    }

    Ok(())
}

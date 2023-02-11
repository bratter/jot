mod args;
mod config;
mod error;

use chrono::prelude::*;
use clap::Parser;
use std::{env, fs, process::Command};

use crate::args::Args;
use crate::config::Config;
use crate::error::Error;

// TODO: Get Serde out of the compilation. Is there some feature in chrono/toml that needs to be
// disabled?
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let config = Config::try_new(args.config)?;

    // Abort early if attempting to create an empty note without editing
    if args.no_edit && args.text.is_none() {
        println!("No edit was set and no text was provided. Aborting.");
        return Ok(());
    }

    // Get the date, including the year and month for building the path and for the front matter
    let date = Local::now();

    // TODO: Move to a new module for text processing
    // TODO: If this is to support configured front matter strings (say in a file in the config
    // dir) this should use replace or a dynamic template engine (e.g. strfmt)
    // Start building the text with the front matter
    let mut text = format!("---\ntimestamp: {}\n---\n\n", date.format("%FT%T%:z"));

    // Then process the provided text, adding a heading (#) to the front if required, then push
    // onto the text
    let body_text = args.text.unwrap_or_default();
    let body_text = body_text.trim();
    let heading_leader = if body_text.starts_with("#") { "" } else { "# " };
    text.push_str(heading_leader);
    text.push_str(body_text);

    // For the location to save, start with the base notes folder to add to
    let mut note_path = config.base_dir();

    // Ensure all the folders are created
    note_path.push(date.format("%Y/%m").to_string());
    //fs::create_dir_all(&note_path).map_err(|err| Error::IO(err))?;

    // Now write out the file
    // TODO: Is this the best date scheme for the file?
    let filename = format!("{}.md", date.format("%Y%m%d_%H%M%S"));
    note_path.push(filename);
    //fs::write(&note_path, text).map_err(|err| Error::IO(err))?;

    // TODO: This should consider falling back to the default system opener
    if !args.no_edit {
        match config.editor.or(env::var("EDITOR").ok()) {
            Some(editor) => {
                //Command::new(editor)
                //    .arg(&note_path)
                //    .spawn()
                //    .map_err(|err| Error::IO(err))?
                //    .wait()
                //    .map_err(|err| Error::IO(err))?;
            }
            None => {
                eprintln!("{}", note_path.to_string_lossy());
                eprintln!("File created but editor could not be opened.");
            }
        }
    }

    // Open by itself
    // TODO: This is not working - competing for the shell
    // I think the process finishes first, but should check differently
    //Command::new(editor).spawn();
    // TODO: This seems to be working
    //Command::new(editor).spawn().unwrap().wait().unwrap();

    // Open in a shell
    // TODO: Also competing for the screen
    //Command::new("/bin/bash").arg("-c").arg(editor).spawn();

    //println!("After the spawn");

    Ok(())
}

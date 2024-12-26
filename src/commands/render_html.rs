use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io,
    path::PathBuf,
};

use anyhow::{bail, Result};

use crate::{
    args::HtmlCmd,
    config::Config,
    html::HtmlWriter,
    path::{generate_output_path, read_md_from_stdin},
};

/// Command called to render HTML.
///
/// Converts Markdown from the input argument to HTML and outputs on stdout by default, or to the
/// file provided using the output argument. To avoid doubt, this will only process files with a
/// `.md` extension. The destination directory must exist.
pub fn render_html(args: &HtmlCmd, config: &Config) -> Result<()> {
    // Find the file to render
    let input = match &args.input {
        Some(input) => input.canonicalize()?,
        // This will require that an output file is provided if the -i flag is set or there won't be a valid output
        // filename to use, unless being output to stdout
        None => PathBuf::new(),
    };
    if args.input.is_some() && input.extension() != Some(&OsString::from("md")) {
        bail!("The file selected is not a markdown file");
    }

    // Dynamically dispatch on the type of writer
    // Do this before reading the input string in case there are issues
    let output_writer: Box<dyn io::Write> = match &args.output {
        Some(output) => Box::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(generate_output_path("html", output.clone(), &input)?)?,
        ),
        None => Box::new(io::stdout()),
    };
    let mut output_writer = HtmlWriter::new(output_writer, config.css.clone());

    let md = match args.input {
        Some(_) => fs::read_to_string(input)?,
        None => read_md_from_stdin()?,
    };

    // Choose the output method based on whether we are producing raw results or not
    match args.raw {
        true => output_writer.write_markdown(&md)?,
        false => output_writer.write_html(&md)?,
    }
    Ok(())
}

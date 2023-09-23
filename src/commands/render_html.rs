use std::{
    fs::{self, OpenOptions},
    io,
};

use anyhow::{bail, Result};

use crate::{args::HtmlCmd, html::HtmlWriter};

/// Converts Markdown from the input argument to HTML and outputs on stdout by default, or to the
/// file provided using the output argument. To avoid doubt, this will only process files with a
/// `.md` extension. The destination directory must exist.
pub fn render_html(args: &HtmlCmd) -> Result<()> {
    // Find the file to render
    let input = args.input.canonicalize()?;
    match input.extension() {
        Some(ext) if ext == "md" => {}
        _ => bail!("The file selected is not a markdown file"),
    }

    // Dyanmically dispatch on the type of writer
    let output_writer: Box<dyn io::Write> = match &args.output {
        Some(output) => Box::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(output)?,
        ),
        None => Box::new(io::stdout().lock()),
    };

    let md = fs::read_to_string(input)?;
    Ok(HtmlWriter::new(output_writer).write(&md)?)
}

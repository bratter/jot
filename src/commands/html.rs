use std::{fs, io};

use anyhow::{bail, Result};
use pulldown_cmark::{html, Parser};

use crate::args::HtmlCmd;

/// Converts Markdown from the input argument to HTML and outputs on stdout by default, or to the
/// file provided using the output argument. To avoid doubt, this will only process files with a
/// `.md` extension. The destination directory must exist.
pub fn html(args: &HtmlCmd) -> Result<()> {
    // Find the file to render
    let input = args.input.canonicalize()?;
    match input.extension() {
        Some(ext) if ext == "md" => {}
        _ => bail!("The file selected is not a markdown file"),
    }

    // Dyanmically dispatch on the type of writer
    let output_writer: Box<dyn io::Write> = match &args.output {
        Some(output) => Box::new(
            fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(output)?,
        ),
        None => Box::new(io::stdout().lock()),
    };

    let md = fs::read_to_string(input)?;
    let parser = Parser::new(&md);
    let writer = io::BufWriter::new(output_writer);

    html::write_html(writer, parser)?;

    Ok(())
}

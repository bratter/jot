//! Module providing logic to write HTML output into a [`Write`], taking care of prepending HTML
//! head contents, and wrapping in a body tag, as well as parsing/rendering markdown.

use std::io::Write;

use anyhow::Result;
use pulldown_cmark::{html, Parser};

pub struct HtmlWriter<W> {
    writer: W,
}

impl<W> HtmlWriter<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Write out the entire contents as HTML, wrapping the Markdown with a full HTML document
    /// including head and body tags.
    pub fn write(&mut self, md: &str) -> Result<()> {
        writeln!(self.writer, "<!DOCTYPE html>\n<html>")?;
        self.head()?;
        writeln!(self.writer, "<body>")?;
        self.markdown(md)?;
        Ok(writeln!(self.writer, "</body>\n</html>")?)
    }

    pub fn head(&mut self) -> Result<()> {
        Ok(writeln!(
            self.writer,
            "<head>\n<meta charset=\"utf-8\" />\n<title>Jot Note</title>\n</head>"
        )?)
    }

    // TODO: This is incredibly inefficient writing the md to a string and then into a writer.
    // Need to create own version of pulldown_cmark's html module (which can then also be used to
    // tweak some rendering, then adapt the writer logic to be able to do this properly.
    // TODO: Strip out front matter and render separately
    // TODO: Add css, likely from a default config file, or a path that is passed in the command
    // TODO: Probably wait for this until can make a separate render crate to work across jot and
    // notes-html
    // TODO: Is there some way to pull the title from the initial H1 if its not in the front
    // matter? Is this beneficial to put in the title anyway?
    pub fn markdown(&mut self, md: &str) -> Result<()> {
        let mut html_str = String::new();
        html::push_html(&mut html_str, Parser::new(&md));

        self.writer.write(&html_str.as_bytes())?;
        Ok(())
    }
}

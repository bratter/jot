//! Module providing logic to write HTML output into a [`Write`], taking care of prepending HTML
//! head contents, and wrapping in a body tag, as well as parsing/rendering markdown.

use std::io::Write;

use anyhow::Result;
use markdown::Markdown;

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

    /// Write out the entire HTML document with article content given by the Markdown content in
    /// the provided string.
    ///
    /// Writes out the entire document including outer `<html>`, `<head>`, and `<body>` tags.
    pub fn write_html(&mut self, md: &str) -> Result<()> {
        writeln!(self.writer, "<!DOCTYPE html>\n<html>")?;
        self.write_head()?;
        writeln!(self.writer, "<body>")?;
        self.write_markdown(md)?;
        Ok(writeln!(self.writer, "</body>\n</html>")?)
    }

    /// Write only the head component of the HTML document.
    pub fn write_head(&mut self) -> Result<()> {
        Ok(writeln!(
            self.writer,
            "<head>\n<meta charset=\"utf-8\" />\n<title>Jot Note</title>\n</head>"
        )?)
    }

    /// Write the rendered markdown component of the document.
    ///
    /// Does not wrap in anything, only produces the raw result of rendering the Markdown string.
    pub fn write_markdown(&mut self, md: &str) -> Result<()> {
        let md = Markdown::new(md);
        md.write_frontmatter_html(&mut self.writer)?;
        md.write_html(&mut self.writer)
    }
}

use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Context, Result};
use headless_chrome::{browser::default_executable, Browser, LaunchOptions};
use tempfile::Builder;

use crate::{
    args::PdfCmd,
    config::Config,
    html::HtmlWriter,
    path::{generate_output_path, read_md_from_stdin},
};

/// Command called to render a PDF.
///
/// Converts Markdown from the input argument to a PDF and outputs to the output file provided
/// using the output argument. To avoid doubt, this will only process files with a`.md` extension.
/// The destination directory must exist.
///
/// PERF: Get rid of all the PathBuf cloning
pub fn render_pdf(args: &PdfCmd, config: &Config) -> Result<()> {
    // Find the file to render
    let input = args
        .input
        .clone()
        .map(|input| canonicalize_input_file(&input))
        .transpose()?;

    // If the output option is provided, turn it into a file writer
    // Keep inside an option to replace with stdout otherwise
    // We do this before generating the HTML in case of errors
    let output_path = args
        .output
        .clone()
        // The default here captures the case when no output file is provided and therefore will error in generate
        // output path if a valid input is required
        .map(|output| generate_output_path("pdf", output, &input.clone().unwrap_or_default()))
        .transpose()?;

    let tmp_file = Builder::new()
        .prefix("jot_tmp_")
        .suffix(".html")
        .tempfile()?;

    // Build the HTML first to avoid spinning up the browser if this step fails
    // We always use an intermediate file as there seems to be no easy way to stream a response
    // directly to Chrome
    let md = match &input {
        Some(input) => fs::read_to_string(input)?,
        None => read_md_from_stdin()?,
    };
    HtmlWriter::new(&tmp_file, config.css.clone()).write_html(&md)?;

    // We can immediately return the result as it appears that random temp will delete the file
    match output_path {
        Some(path) => {
            let mut output_file_writer = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&path)?;
            println!("Starting to convert pdf");
            convert_pdf(&mut output_file_writer, &tmp_file.path().to_string_lossy())?;
            println!(
                "PDF conversion complete, output file at {}",
                path.to_string_lossy()
            );
        }
        None => convert_pdf(&mut io::stdout(), &tmp_file.path().to_string_lossy())?,
    }

    Ok(())
}

/// Take the input file name and canonicalize, noting that this will check for existence.
fn canonicalize_input_file(original_input: &PathBuf) -> Result<PathBuf> {
    let input = original_input.canonicalize().with_context(|| {
        format!(
            "Invalid input path, unable to canonicalize {}",
            &original_input.to_string_lossy()
        )
    })?;

    // Also check that the extension is right
    match input.extension() {
        Some(ext) if ext == "md" => {}
        _ => bail!("The file selected is not a markdown file"),
    }

    Ok(input)
}

/// Convert the HTML file at file_name to a pdf and save into the output file handle.
/// TODO: Conditionally compile the call to print_to_pdf out in test mode
fn convert_pdf<T: Write>(output: &mut T, file_name: &str) -> Result<()> {
    let browser = get_browser()?;
    let tab = browser.new_tab()?;

    tab.navigate_to(&format!("file://{}", file_name))?;
    tab.wait_for_element("html")?;

    output.write_all(&tab.print_to_pdf(None)?)?;

    Ok(())
}

/// Custom get browser function.
///
/// [`Browser::default`] unwraps the call to default executable which causes an unecessary panic
/// that we wuold rather handle gracefully, hence reproducing our own version here.
///
/// We also output a more helpful error message to stdout for end users.
fn get_browser() -> Result<Browser> {
    let exe = match default_executable() {
        Ok(exe) => Ok(exe),
        Err(err) => {
            eprintln!(
                r#"
Cannot convert to pdf.

Converting to pdf requires a Chrome-like browser installed and available on the system PATH.
Please install Chrome, Edge, Chromium, or similar in a manner appropriate for your system to use this feature.
"#
            );
            Err(anyhow!(err))
        }
    }?;
    let launch_options = LaunchOptions::default_builder().path(Some(exe)).build()?;

    Browser::new(launch_options)
}

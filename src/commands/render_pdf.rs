use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::{anyhow, bail, Context, Result};
use headless_chrome::{browser::default_executable, Browser, LaunchOptions};

use crate::{args::PdfCmd, html::HtmlWriter};

/// Converts Markdown from the input argument to a PDF and outputs to the output file provided
/// using the output argument. To avoid doubt, this will only process files with a`.md` extension.
/// The destination directory must exist.
pub fn render_pdf(args: &PdfCmd) -> Result<()> {
    let input = canonicalize_input_file(&args.input)?;
    let output = cannonicalize_output_file(&args.output)?;

    // Open the tmp and output files before anything else to avoid uneccessary processing if any of
    // the files are invalid
    let tmp_file_name = "/tmp/test.html";
    let tmp_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(tmp_file_name)?;
    let mut output_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&output)?;

    // Build the HTML first to avoid spinning up the browser if this step fails
    // We always use an intermediate file as there seems to be no easy way to stream a response
    // directly to Chrome
    let md = fs::read_to_string(input)?;
    HtmlWriter::new(tmp_file).write(&md)?;

    // Don't immediately return so that we can remove the tmp file whether the conversion succeeded
    // or not
    println!("Starting to convert pdf");
    let conversion_result = convert_pdf(&mut output_file, tmp_file_name);
    println!("PDF conversion complete, output file at {:?}", output);

    fs::remove_file(tmp_file_name)?;
    conversion_result
}

fn canonicalize_input_file(original_input: &PathBuf) -> Result<PathBuf> {
    let input = original_input.canonicalize().with_context(|| {
        format!(
            "Invalid input path, unable to canonicalize {:?}",
            &original_input
        )
    })?;

    // Also check that the extension is right
    match input.extension() {
        Some(ext) if ext == "md" => {}
        _ => bail!("The file selected is not a markdown file"),
    }

    Ok(input)
}

fn cannonicalize_output_file(original_output: &PathBuf) -> Result<PathBuf> {
    let output = original_output.parent().ok_or_else(|| {
        anyhow!(
            "Invalid output path {:?}, must end in a filename",
            original_output
        )
    })?;

    let output = if output.as_os_str().is_empty() {
        output.join(".").canonicalize()
    } else {
        output.canonicalize()
    }
    .context(format!(
        "Invalid output path, unable to canonicalize directory in {:?}",
        output
    ))?;

    let canonical_output = original_output.file_name().ok_or_else(|| {
        anyhow!(
            "Invalid output path, unable to located file name in {:?}",
            original_output
        )
    })?;

    Ok(output.join(canonical_output))
}

/// Convert the HTML file at file_name to a pdf and save into the output file handle.
fn convert_pdf(output: &mut File, file_name: &str) -> Result<()> {
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

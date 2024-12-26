//! Path helpers.

use std::{
    ffi::OsString,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Context, Result};

/// Take the output path provided, check its validity, and canonicalize.
/// If the original output is none, we put the file next to its .md source, otherwise try to canonicalize the file name.
pub(crate) fn generate_output_path(
    ext: &str,
    original_output: Option<PathBuf>,
    input: &PathBuf,
) -> Result<PathBuf> {
    // Replace the original output option with the input folder
    // This *should* be valid, but we error appropriately anyway because we are dealing with IO
    let original_output = match original_output {
        Some(output) => output,
        None => input
            .parent()
            .context("Valid input folder required when an output folder is not provided")?
            .to_path_buf(),
    };

    if original_output.is_dir() {
        // If we have a valid directory input, then add the input filename
        let mut output = original_output.canonicalize()?;
        let filename = input.file_name().with_context(|| {
            format!(
                "Input file does not have a filename: {}",
                input.to_string_lossy()
            )
        })?;
        output.push(filename);
        output.set_extension(ext);

        return Ok(output);
    }

    // Now if it is something else, we proceed as if it is a file, erroring if something goes wrong
    let output = original_output.parent().ok_or_else(|| {
        anyhow!(
            "Invalid output path {}, must end in a filename",
            original_output.to_string_lossy()
        )
    })?;

    let mut output = if output.as_os_str().is_empty() {
        output.join(".").canonicalize()
    } else {
        output.canonicalize()
    }
    .with_context(|| {
        format!(
            "Invalid output path, unable to canonicalize directory in {}",
            output.to_string_lossy()
        )
    })?;

    let canonical_file = original_output.file_name().ok_or_else(|| {
        anyhow!(
            "Invalid output path, unable to locate file name in {}",
            original_output.to_string_lossy()
        )
    })?;

    output.push(canonical_file);
    let os_ext = output.extension();
    if os_ext != Some(&OsString::from(ext)) {
        bail!(
            "Extension must be .{}, .{} provided",
            ext,
            os_ext.unwrap_or_default().to_string_lossy()
        );
    }

    Ok(output)
}

/// Pulls string content from stdin.
pub(crate) fn read_md_from_stdin() -> Result<String> {
    let mut md = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut md) {
        bail!("Error reading from stdin: {}", e);
    }
    Ok(md)
}

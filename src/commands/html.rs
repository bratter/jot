use crate::{args::HtmlCmd, error::Error};

// TODO: File behavior - which to pick? Probably just cwd is the best, maybe add a flag for
// the notes dir, but the main one or the atoms?
pub fn html(args: &HtmlCmd) -> Result<(), Error> {
    Ok(())
}

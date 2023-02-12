pub enum Error {
    IO(std::io::Error),
    TomlParse,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Build out the error messages
        writeln!(f, "An error occured")
    }
}

// Custom implementation that delegates to Display to give more user-friendly error messages in
// termination.
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

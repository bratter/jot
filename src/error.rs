#[non_exhaustive]
pub enum Error {
    IO(std::io::Error),
    EditorNotFound,
    TomlParse,
}

impl std::error::Error for Error {}

// TODO: Consider more informative error messages that incorporate more context
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => writeln!(f, "An IO error occured: {err}"),
            Self::TomlParse => writeln!(f, "Unable to correctly parse the configuration file."),
            Self::EditorNotFound => writeln!(
                f,
                "Could not located the target editor in config or as the $EDITOR env var"
            ),
        }
    }
}

// Custom implementation that delegates to Display to give more user-friendly error messages in
// termination.
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

use std::{env, fs, io, path::PathBuf};

use resolve_path::PathResolveExt;

use crate::{args::Args, error::Error};

const CONFIG_FILE: &str = "jot/conf.toml";
const DEFAULT_ROOT: &str = "~/notes";
const DEFAULT_SUBDIR: &str = "atoms";

pub struct Config {
    /// The editor to use. Requires an editor string even if the no-edit flag is passed.
    pub editor: String,

    /// Attempt to jump to the last line of the file to edit. On by default but set to false in
    /// config if it causes the editor to error.
    pub jump: bool,

    /// The root path of the notes directory
    root: PathBuf,

    /// The location inside the root to store notes
    subdir: String,
}

impl Config {
    /// Build a new config from an optional path that will fall back to default if None.
    pub fn try_new(args: &Args) -> Result<Self, Error> {
        let path = &args.config;

        match path {
            Some(path) => Self::try_from_path(path, &args.subdir),
            None => Self::try_default(&args.subdir),
        }
    }

    /// Attempt to build a Config from the provided path.
    pub fn try_from_path(path: &PathBuf, subdir: &Option<String>) -> Result<Self, Error> {
        let toml = fs::read_to_string(path.resolve())
            .map_err(|err| Error::IO(err))?
            .parse::<toml::Table>()
            .map_err(|_| Error::TomlParse)?;

        // First, attempt to find the editor in the config
        // If it's not present, grab the EDITOR env var
        // Return an error if not available
        let editor = match toml.get("editor") {
            Some(toml::Value::String(s)) => s.clone(),
            Some(_) => return Err(Error::TomlParse),
            None => env::var("EDITOR").map_err(|_| Error::EditorNotFound)?,
        };

        let jump = match toml.get("jump") {
            Some(toml::Value::Boolean(b)) => *b,
            Some(_) => return Err(Error::TomlParse),
            None => false,
        };

        let root = match toml.get("root") {
            Some(toml::Value::String(s)) => PathBuf::from(s),
            Some(_) => return Err(Error::TomlParse),
            None => PathBuf::from(DEFAULT_ROOT),
        };
        let root = root
            .try_resolve()
            .map_err(|err| Error::IO(err))?
            .to_path_buf();

        Ok(Self {
            editor,
            jump,
            root,
            subdir: subdir.clone().unwrap_or(DEFAULT_SUBDIR.to_string()),
        })
    }

    /// Attempt to build a config using the default config location.
    /// Will choose the system specific location for the config file based on XDG conventions.
    /// If the file errors for not being present, this is OK - we build a default config
    /// Any other error is passed back to the caller.
    pub fn try_default(subdir: &Option<String>) -> Result<Self, Error> {
        let mut config_path = dirs::config_dir().ok_or(Error::IO(io::Error::new(
            io::ErrorKind::NotFound,
            "Config dir could not be resolved",
        )))?;
        config_path.push(CONFIG_FILE);

        match Self::try_from_path(&config_path, subdir) {
            Err(Error::IO(err)) => {
                // A not found error is ok - build a default
                // Otherwise retain the error
                if err.kind() == std::io::ErrorKind::NotFound {
                    Self::default_config()
                } else {
                    Err(Error::IO(err))
                }
            }
            res @ _ => res,
        }
    }

    pub fn base_dir(&self) -> PathBuf {
        let mut path = self.root.clone();
        path.push(&self.subdir);
        path
    }

    /// Default config. Not implementing Default as this should not be called outside this module.
    fn default_config() -> Result<Self, Error> {
        let editor = env::var("EDITOR").map_err(|_| Error::EditorNotFound)?;
        let root = PathBuf::from(DEFAULT_ROOT)
            .try_resolve()
            .map_err(|err| Error::IO(err))?
            .to_path_buf();

        Ok(Self {
            editor,
            jump: true,
            root,
            subdir: DEFAULT_SUBDIR.to_string(),
        })
    }
}

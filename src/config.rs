use std::{fs, io, path::PathBuf};

use resolve_path::PathResolveExt;

use crate::error::Error;

const CONFIG_FILE: &str = "jot/conf.toml";
const DEFAULT_ROOT: &str = "~/notes";
const DEFAULT_SUBDIR: &str = "atoms";

pub struct Config {
    pub editor: Option<String>,

    /// The root path of the notes directory
    root: PathBuf,

    /// The location inside the root to store notes
    subdir: &'static str,
}

impl Config {
    /// Build a new config from an optional path that will fall back to default if None.
    pub fn try_new(path: Option<PathBuf>) -> Result<Self, Error> {
        match path {
            Some(path) => Self::try_from_path(path),
            None => Self::try_default(),
        }
    }

    /// Attempt to build a Config from the provided path.
    pub fn try_from_path(path: PathBuf) -> Result<Self, Error> {
        let toml = fs::read_to_string(path.resolve())
            .map_err(|err| Error::IO(err))?
            .parse::<toml::Table>()
            .map_err(|_| Error::TomlParse)?;

        let editor = match toml.get("editor") {
            Some(toml::Value::String(s)) => Some(s.clone()),
            Some(_) => return Err(Error::TomlParse),
            None => None,
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

        // TODO: subdir comes from the args, not the settings file
        Ok(Self {
            editor,
            root,
            subdir: DEFAULT_SUBDIR,
        })
    }

    /// Attempt to build a config using the default config location.
    /// Will choose the system specific location for the config file based on XDG conventions.
    /// If the file errors for not being present, this is OK - we build a default config
    /// Any other error is passed back to the caller.
    pub fn try_default() -> Result<Self, Error> {
        let mut config_path = dirs::config_dir().ok_or(Error::IO(io::Error::new(
            io::ErrorKind::NotFound,
            "Config dir could not be resolved",
        )))?;
        config_path.push(CONFIG_FILE);

        match Self::try_from_path(config_path) {
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
        path.push(self.subdir);
        path
    }

    /// Default config. Not implementing Default as this should not be called outside this module.
    fn default_config() -> Result<Self, Error> {
        let root = PathBuf::from(DEFAULT_ROOT)
            .try_resolve()
            .map_err(|err| Error::IO(err))?
            .to_path_buf();

        Ok(Self {
            editor: None,
            root,
            subdir: DEFAULT_SUBDIR,
        })
    }
}

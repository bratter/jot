use std::{env, fs, path::PathBuf};

use crate::error::Error;

const XDG_VAR: &str = "XDG_CONFIG";
const DEFAULT_CONFIG_DIR: &str = "~/.config";
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

// TODO: Use the dirs crate to resolve the paths that start with ~, and also replace some of the
// logic below

// TODO: Load logic
// TOML file must be present, but what to do if its not there?
// Maybe force to run --init before starting any notes, and init creates the config file/folder
// Or, running it without a config just uses a default place for the notes
// I think the latter is best, then can build init logic in afterwards

impl Config {
    /// Build a new config from an optional path that will fall back to default if None.
    pub fn try_new(path: Option<PathBuf>) -> Result<Self, Error> {
        match path {
            Some(path) => Self::try_from_path(&path),
            None => Self::try_default(),
        }
    }

    /// Attempt to build a Config from the provided path.
    pub fn try_from_path(path: &PathBuf) -> Result<Self, Error> {
        let toml_str = fs::read_to_string(path).map_err(|err| Error::IO(err))?;

        // TODO: Attempt to read the TOML
        // Just parse by hand, don't introduce SerDe dependency for this

        todo!()
        //Ok(config)
    }

    /// Attempt to build a config using the default config location. This first tests for
    /// XDG_CONFIG, then attempts ~/.config, accessing the jot/conf.toml file in both cases.
    /// If the file errors for not being present, this is OK - we build a default config
    /// Any other error is passed back to the caller
    /// TODO: Check AppData/local for windows
    pub fn try_default() -> Result<Self, Error> {
        let config_base = env::var(XDG_VAR);
        let config_base = config_base
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(DEFAULT_CONFIG_DIR);
        let mut config_path = PathBuf::from(config_base);
        config_path.push(CONFIG_FILE);

        match Self::try_from_path(&config_path) {
            Err(Error::IO(err)) => {
                // A not found error is ok - build a default
                // Otherwise retain the error
                if err.kind() == std::io::ErrorKind::NotFound {
                    Ok(Self::default_config())
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
    fn default_config() -> Self {
        println!("---");
        println!("{:?}", fs::canonicalize(DEFAULT_ROOT));
        Self {
            editor: None,
            root: PathBuf::from(DEFAULT_ROOT),
            subdir: DEFAULT_SUBDIR,
        }
    }
}

use std::{env, fs, io, path::PathBuf};

use anyhow::{bail, Result};
use resolve_path::PathResolveExt;

use crate::args::Args;

const CONFIG_FILE: &str = "jot/conf.toml";
const CSS_FILE: &str = "jot.css";
const DEFAULT_ROOT: &str = "~/notes";
const DEFAULT_SUBDIR: &str = "atoms";
const FALLBACK_EDITOR: &str = "vim";

/// Configuration reader.
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

    /// The path to the css file to inject into the header when rendering HTML
    pub css: Option<PathBuf>,
}

impl Config {
    /// Build a new config from an optional path that will fall back to default if None.
    pub fn try_new(args: &Args) -> Result<Self> {
        let path = &args.config;

        match path {
            Some(path) => Self::try_from_path(path, &args.subdir),
            None => Self::try_default(&args.subdir),
        }
    }

    /// Attempt to build a Config from the provided path.
    pub fn try_from_path(path: &PathBuf, subdir: &Option<String>) -> Result<Self> {
        let config_file = path.try_resolve()?;
        let toml = fs::read_to_string(&config_file)?.parse::<toml::Table>()?;

        // First, attempt to find the editor in the config
        // If it's not present, grab the EDITOR env var
        // If not available, assume we have vim
        let editor = match toml.get("editor") {
            Some(toml::Value::String(s)) => s.clone(),
            Some(_) => bail!("Could not parse TOML"),
            None => Self::fallback_editor(),
        };

        let jump = match toml.get("jump") {
            Some(toml::Value::Boolean(b)) => *b,
            Some(_) => bail!("Could not parse TOML"),
            None => false,
        };

        let root = match toml.get("root") {
            Some(toml::Value::String(s)) => PathBuf::from(s),
            Some(_) => bail!("Could not parse TOML"),
            None => PathBuf::from(DEFAULT_ROOT),
        };
        let root = root.try_resolve()?.to_path_buf();

        // First attempt to find a css file in the root of the notes directory
        // if that doesn't exist, try to find a global config
        let css = root.join(CSS_FILE).canonicalize().ok().or_else(|| {
            config_file
                .parent()
                .and_then(|p| p.join(CSS_FILE).canonicalize().ok())
        });

        Ok(Self {
            editor,
            jump,
            root,
            subdir: subdir.clone().unwrap_or(DEFAULT_SUBDIR.to_string()),
            css,
        })
    }

    /// Attempt to build a config using the default config location.
    /// Will choose the system specific location for the config file based on XDG conventions.
    /// If the file errors for not being present, this is OK - we build a default config
    /// Any other error is passed back to the caller.
    pub fn try_default(subdir: &Option<String>) -> Result<Self> {
        let config_path = dirs::config_dir()
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Config dir could not be resolved",
            ))?
            .join(CONFIG_FILE)
            .canonicalize();

        match config_path {
            Ok(config_path) => Self::try_from_path(&config_path, subdir),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Self::default_config(),
            Err(err) => return Err(err.into()),
        }
    }

    pub fn base_dir(&self) -> PathBuf {
        self.root.join(&self.subdir)
    }

    /// Default config. Not implementing Default as this should not be called outside this module.
    fn default_config() -> Result<Self> {
        let editor = Self::fallback_editor();
        let root = PathBuf::from(DEFAULT_ROOT).try_resolve()?.to_path_buf();

        Ok(Self {
            editor,
            jump: true,
            root,
            subdir: DEFAULT_SUBDIR.to_string(),
            css: None,
        })
    }

    fn fallback_editor() -> String {
        env::var("EDITOR").unwrap_or(String::from(FALLBACK_EDITOR))
    }
}

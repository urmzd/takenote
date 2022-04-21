use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::{env, fmt};

type ConfigChildren = Option<Vec<String>>;
type ConfigName = String;

/// A container holding the parsed data from a Takenote configuration file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    /// The name associated with this journal.
    name: ConfigName,
    /// A collection of journals available to reference.
    children: ConfigChildren,
}

#[derive(Clone, Debug)]
struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Oops... Something went wrong while creating your config file, check that the path provided is valid!")
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn create_project(
        name: ConfigName,
        children: ConfigChildren,
        path: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let project_dir = match path.to_owned().into_os_string().into_string() {
            Ok(string_path) => string_path,
            _ => return Err(ConfigError.into()),
        };
        let config = Config { name, children };

        // Create directory.
        std::fs::create_dir_all(project_dir)?;

        let project_config_path = Path::new(&path).join("config.toml");

        let serialized_config: String = toml::to_string(&config)?;
        fs::write(project_config_path, serialized_config)?;

        return Ok(());
    }
}

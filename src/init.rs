use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

const DEFAULT_HEAD_ENV_VAR: &str = "TAKENOTE_DEFAULT_HEAD";

/// Holds the values of the environment variables required to run `takenote`.
#[derive(Debug, Serialize)]
pub struct Environment {
    /// Points to the directory which contains the entrypoint.
    pub default_dir: PathBuf,
}

impl Environment {
    /// Retrieves the environment variables needed to intialize the project.
    ///
    /// # Panics
    ///
    /// Panics if the required environment variables are not set.
    ///
    /// # Examples
    ///
    /// ```
    /// let environment = Environment::new();
    /// ```
    pub fn new() -> Environment {
        let default_dir_str: String = env::var(DEFAULT_HEAD_ENV_VAR).unwrap();
        let default_dir: PathBuf = Path::new(&default_dir_str).to_owned();

        Environment { default_dir }
    }
}

/// A container holding the parsed data from a Takenote configuration file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    /// The name associated with this journal.
    name: String,
    /// A collection of journals available to reference.
    children: Option<Vec<String>>,
}

impl From<PathBuf> for Config {
    fn from(file_path: PathBuf) -> Self {
        let contents = fs::read_to_string(file_path).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        config
    }
}

impl Config {
    pub fn generate_folder_structure_from_config(
        &self,
        root_dir: Option<&String>,
    ) -> Result<(), Box<dyn Error>> {
        let cwd = env::current_dir()?.as_os_str().to_str().unwrap().to_owned();
        let working_dir: &String = root_dir.unwrap_or(&cwd);

        // Create directory.
        std::fs::create_dir(working_dir)?;

        return Ok(());
    }
}

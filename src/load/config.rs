use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

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

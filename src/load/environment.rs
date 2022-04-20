use super::constants::DEFAULT_HEAD_ENV_VAR;
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
};

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

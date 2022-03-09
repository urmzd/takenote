use std::{env, env::VarError, error::Error, fs};

use serde::{Deserialize, Serialize};

const DEFAULT_HEAD_ENV_VAR: &str = "TAKENOTE_DEFAULT_HEAD";

/// Holds environment variables
pub struct Environment {
    pub default_dir: String,
}

impl Environment {
    /// Retrieves the environment variables needed to intialize the project.
    ///
    /// # Errors
    ///
    /// Returns a [`VarError`] if the a required environment variable is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// let environment = Environment::pull();
    /// ```
    pub fn pull() -> Result<Environment, VarError> {
        let default_dir = env::var(DEFAULT_HEAD_ENV_VAR)?;

        return Ok(Environment { default_dir });
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

impl Config {
    /// Parses the contents from a Takenote configuration file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file_path is invalid or the contents
    /// of the configuration file do not match the specs outlined in
    /// [`Config`].
    ///
    /// # Examples
    ///
    /// ```
    /// let config = read_config_from_file("/home/urmzd/.takenote.config.toml");
    /// ```
    pub fn read_config_from_file(file_path: &String) -> Result<Config, Box<dyn Error>> {
        let contents = fs::read_to_string(file_path)?;
        let config = toml::from_str(&contents)?;

        return Ok(config);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::{error::Error, io::Write};
    use tempfile::NamedTempFile;

    #[test]
    fn given_valid_file_path_when_file_is_read_then_config_is_provided(
    ) -> Result<(), Box<dyn Error>> {
        // Arrange
        let tmp_config = Config {
            name: "Urmzd Mukhammadnaim".to_string(),
            children: None,
        };

        let tmp_config_string = toml::to_string(&tmp_config)?;
        let mut tmp_file = NamedTempFile::new()?;

        tmp_file.write_all(&tmp_config_string.as_bytes())?;

        // Act
        let config_to_match = match tmp_file.path().to_str() {
            Some(value) => Config::read_config_from_file(&value.to_string()),
            None => Err("THIS SHOULD NEVER HAPPEN".into()),
        }?;

        // Assert
        assert_eq!(config_to_match, tmp_config);

        Ok(())
    }

    #[test]
    fn given_env_var_is_set_when_environment_is_read_then_environment_struct_is_provided(
    ) -> Result<(), Box<dyn Error>> {
        // Arrange.
        let current_env_var = env::var(DEFAULT_HEAD_ENV_VAR);
        let test_var_value = "TEST_VAR_VALUE";

        env::set_var(DEFAULT_HEAD_ENV_VAR, &test_var_value);

        // Act.
        let enviroment = Environment::pull()?;

        // Assert.
        assert_eq!(enviroment.default_dir, test_var_value.to_string());

        // Cleanup.
        if current_env_var.is_ok() {
            env::set_var(DEFAULT_HEAD_ENV_VAR, current_env_var.unwrap());
        }

        Ok(())
    }

    #[test]
    #[should_panic]
    fn given_unset_env_var_when_environment_is_read_errr_is_thrown() {
        // Arrange
        let current_env_var = env::var(DEFAULT_HEAD_ENV_VAR);

        env::remove_var(DEFAULT_HEAD_ENV_VAR);

        let enviroment = Environment::pull();

        if current_env_var.is_ok() {
            env::set_var(DEFAULT_HEAD_ENV_VAR, current_env_var.unwrap());
        }

        enviroment.unwrap();
    }
}

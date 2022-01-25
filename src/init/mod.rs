pub mod init {
    use std::env::{self, VarError};

    use serde::{Deserialize, Serialize, de::Error};

    #[allow(dead_code)]
    struct Environment {
        default_dir: String,
    }

    /// TODO - Retrieve and deserialize environment file.
    ///
    ///
    ///
    ///
    ///
    impl Environment {
        #[allow(dead_code)]
        fn pull() -> Result<Environment, VarError> {
            let default_dir = env::var("TAKENOTE_DEFAULT_DIR")?;

            let enviroment = Environment { default_dir };
            return Ok(enviroment);
        }
    }

    #[allow(dead_code)]
    #[derive(Serialize, Deserialize, Debug)]
    struct ServiceProviders {
        linkedin: String,
        medium: String,
    }

    struct Config {
        children: Option<Vec<String>>,
        providers: Option<ServiceProviders>,
    }

    impl Config {
        #[allow(dead_code)]
        fn read_config_from_file(file_content: &String) -> Result<Config, Error> {
            let config: Result<Config, Error> = toml::from_str(file_content)
            return Ok(Config {
                children: None,
                providers: None,
            });
        }

        #[allow(dead_code)]
        fn read_config_from_cli(args: &[String]) -> Result<Config, &str> {
            return Ok(Config {
                children: None,
                providers: None,
            });
        }

        #[allow(dead_code)]
        fn merge_config() {}
    }

    #[allow(dead_code)]
    fn run() {}
}

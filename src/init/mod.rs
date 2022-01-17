pub mod init {
    use std::env::{self, VarError};

    use serde::{Deserialize, Serialize};

    #[allow(dead_code)]
    struct Environment {
        default_dir: String,
    }

    impl Environment {
        fn new(default_dir: String) -> Environment {
            Environment {
                default_dir: default_dir,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Serialize, Deserialize, Debug)]
    struct Config {
        children: Option<Vec<String>>,
    }

    #[allow(dead_core)]
    fn parse_environment_variables() -> Result<Environment, VarError> {
        let default_dir = env::var("TAKENOTE_DEFAULT_DIR")?;

        let environment = Environment::new(default_dir);
        Ok(environment)
    }

    impl Config {
        /*
         *        #[allow(dead_code)]
         *        fn read_config_from_file(file_content: &String) -> Result<Config, Error> {
         *            return Result<Config, _> = toml::from_str(file_content)
         *        }
         *
         *        #[allow(dead_code)]
         *        fn read_config_from_cli(args: &[String]) -> Result<Config, &str> {}
         */

        /*
         *fn merge_config() {}
         */
    }

    #[allow(dead_code)]
    fn run() {}
}

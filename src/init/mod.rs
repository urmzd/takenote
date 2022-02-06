pub mod init {
    use std::{
        env::{self, VarError},
        error::Error,
        fs,
    };

    use serde::{Deserialize, Serialize};

    pub struct Environment {
        pub default_dir: String,
    }

    impl Environment {
        pub fn pull() -> Result<Environment, VarError> {
            let default_dir = env::var("TAKENOTE_DEFAULT_HEAD")?;

            return Ok(Environment { default_dir });
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct ServiceProviders {
        linkedin: String,
        medium: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Config {
        name: String,
        children: Option<Vec<String>>,
        providers: Option<ServiceProviders>,
    }

    impl Config {
        #[allow(dead_code)]
        fn read_config_from_file(file_path: &str) -> Result<Config, Box<dyn Error>> {
            let contents = fs::read_to_string(file_path)?;
            let config = toml::from_str(&contents)?;

            return Ok(config);
        }
    }

    #[allow(dead_code)]
    fn init_folders() {}

    #[allow(dead_code)]
    fn run() {}
}

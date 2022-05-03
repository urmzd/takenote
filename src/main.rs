use clap::StructOpt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs;
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
pub const DEFAULT_HEAD_ENV_VAR: &str = "TAKENOTE_DEFAULT_HEAD";
pub type ConfigName = String;
pub type ConfigChildren = Option<Vec<String>>;

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

impl TryFrom<Subcommands> for Config {
    type Error = &'static str;

    fn try_from(value: Subcommands) -> Result<Self, Self::Error> {
        match value {
            Subcommands::Init { name, children } => Ok(Config { name, children }),
            _ => Err("Called"),
        }
    }
}

impl Config {
    pub fn create_project(
        name: ConfigName,
        children: ConfigChildren,
        path: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let project_dir = match path.to_owned().into_os_string().into_string() {
            Ok(string_path) => string_path,
            _ => return Err(ConfigError.into()),
        };
        let config = Config { name, children };

        // Create directory.
        std::fs::create_dir_all(project_dir)?;

        let project_config_path = path.join("config.toml");

        let serialized_config: String = toml::to_string(&config)?;
        fs::write(project_config_path, serialized_config)?;

        return Ok(());
    }
}
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    pub message: Option<String>,
    #[clap(subcommand)]
    pub commands: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    Init {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        children: Option<Vec<String>>,
        #[clap(short, long)]
        /// Default flag indicates that all commands used without the specification of a journal
        /// name should be applied to this journal.
        default: bool,
    },
    Find {
        #[clap(short, long)]
        query: String,
        #[clap(short, long)]
        name: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    //let random_path = Path::new("./random_proj");
    //let random_path_buf = random_path.to_owned();
    //let random_name = "random".to_owned();

    //let config = load::config::Config::create_project(random_name, None, &random_path_buf);
    //println!("{:?}", config);
    let cli = Cli::parse();
    println!("{:?}", cli);
    Ok(())
}

use clap::StructOpt;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// TODO - remove this after refactoring.
/// Ramblings of a Software Engineer: Read at your own discretion...
/// So this is how I envision the application working.
///
/// When the application is initialized, the system uses an index file
/// `.takenote.index` which holds two things.
///
/// Note: We use [b-tree indexing](https://en.wikipedia.org/wiki/B-tree)
///
/// 1. It must hold the reference to the "default" journal. This is solely for conveinence
/// and is a way of preventing an unneccessary arg when using any subcommand.
///
/// e.g. `takenote "random message to default journal"` instead of `takenote --name
/// "some_journal_name" "some_message"`.
///
/// Of course, we allow people to specify the name if so desired.
///
/// 2. We need a way to link different references during `markdown`
/// compilation.  Without an index file, we have no idea where to find a journal.
///
/// -- thinking more on this, I wonder if we need an index file within every child node (excluding
/// the leaf). Reading more on b-tree indexing, we likely do. Will investigate more, when querying
/// becomes a bottleneck.
///
/// Ok back to the commmands and the actual use case and interactions that need supporting.
/// Lets first start by showing example commands
///
/// `takenote init --name "urmzd's journal" --default`
/// `takenote "some random message."`
/// `takenote find "some text using custom query language?"` -- will need to revisit this.
/// `taknote generate --language "markdown"` -- we store notes in plain text (encrypted likely)
/// text.

pub type ConfigName = String;

/// A container holding the parsed data from a Takenote configuration file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    /// The name associated with this journal.
    name: ConfigName,
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
            Subcommands::Init { name, .. } => Ok(Config { name }),
            _ => Err("Called with the wrong subcommand."),
        }
    }
}

impl Config {
    pub fn create_project(name: ConfigName, path: &Path) -> Result<(), Box<dyn Error>> {
        let project_dir = match path.to_owned().into_os_string().into_string() {
            Ok(string_path) => string_path,
            _ => return Err(ConfigError.into()),
        };
        let config = Config { name };

        // Create directory.
        std::fs::create_dir_all(project_dir)?;

        let project_config_path = path.join("config.toml");

        let serialized_config: String = toml::to_string(&config)?;
        fs::write(project_config_path, serialized_config)?;

        return Ok(());
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(short, long)]
    name: Option<String>,
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
    },
}

struct JournalReference(String, Path);

/// Holds journal references.
struct Index {
    default: String,
    references: HashMap<String, PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    //let random_path_buf = random_path.to_owned();
    //let random_name = "random".to_owned();

    //let config = load::config::Config::create_project(random_name, None, &random_path_buf);
    //println!("{:?}", config);
    let cli = Cli::parse();
    println!("{:?}", cli);
    Ok(())
}

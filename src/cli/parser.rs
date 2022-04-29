use clap::{Parser, Subcommand};

use crate::load::config::{ConfigChildren, ConfigName};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    message: Option<String>,
    #[clap(subcommand)]
    commands: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    Init {
        #[clap(short, long)]
        name: ConfigName,
        #[clap(short, long)]
        children: ConfigChildren,
    },
    Find {
        query: String,
    },
}

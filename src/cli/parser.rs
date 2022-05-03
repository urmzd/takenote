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
    },
    Find {
        #[clap(short, long)]
        query: String,
    },
}

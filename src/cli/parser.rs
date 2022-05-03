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

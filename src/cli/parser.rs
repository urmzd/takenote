use clap::Subcommand;

enum Commands {
    Init,
    Takenote,
    Find,
}

#[derive(Debug, Subcommand)]
struct Init {
    #[clap(short, long, required)]
    name: ConfigName,

    #[clap(short, long)]
    children: ConfigChildren,
}

struct Takenote {}

struct Find {}

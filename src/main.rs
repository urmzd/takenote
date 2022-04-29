use std::error::Error;
use clap::StructOpt;
use takenote::cli::parser::{Cli}

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

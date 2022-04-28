use std::error::Error;
use std::path::Path;
use takenote::cli::parser::generate_cli;
use takenote::load;

fn main() -> Result<(), Box<dyn Error>> {
    //let random_path = Path::new("./random_proj");
    //let random_path_buf = random_path.to_owned();
    //let random_name = "random".to_owned();

    //let config = load::config::Config::create_project(random_name, None, &random_path_buf);
    //println!("{:?}", config);
    let mut cli = generate_cli();
    println!("{}", cli.render_usage());
    Ok(())
}

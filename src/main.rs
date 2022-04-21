use std::error::Error;
use std::path::Path;
use takenote::load;

fn main() -> Result<(), Box<dyn Error>> {
    //let env = load::environment::Environment::new();
    //println!("{:?}", env);
    let random_path = Path::new("~");
    let random_path_buf = random_path.to_owned();
    let random_name = "random".to_owned();

    let config = load::config::Config::create_project(random_name, None, &random_path_buf);
    println!("{:?}", config);
    Ok(())
}

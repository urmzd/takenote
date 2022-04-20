use std::error::Error;
use takenote::load;

fn main() -> Result<(), Box<dyn Error>> {
    let env = load::Environment::new();
    println!("{:?}", env);
    let config = load::Config::from(env.default_dir);
    println!("{:?}", config);
    Ok(())
}

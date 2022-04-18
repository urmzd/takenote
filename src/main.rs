use std::error::Error;

use takenote::init;

fn main() -> Result<(), Box<dyn Error>> {
    let env = init::Environment::new();
    println!("{:?}", env);
    let config = init::Config::from(env.default_dir);
    println!("{:?}", config);
    Ok(())
}

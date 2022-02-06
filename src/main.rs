use std::error::Error;

mod init;

fn main() -> Result<(), Box<dyn Error>> {
    let env = init::Environment::pull()?;
    println!("GOT MY VARIABLE YUH {}", env.default_dir);
    let config = init::Config::read_config_from_file(&env.default_dir)?;
    let ser = toml::to_string(&config)?;
    println!("GOT MY CONFIGS YUH {}", ser);

    Ok(())
}

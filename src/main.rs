use std::env::VarError;

mod init;

fn main() -> Result<(), VarError> {
    let env = init::Environment::pull()?;
    println!("GOT MY VARIABLE YUH {}", env.default_dir);

    Ok(())
}

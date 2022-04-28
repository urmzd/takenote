use clap::{arg, command, Command};

pub fn generate_cli() -> Command<'static> {
    let find_subcommand = command!().name("find").arg(arg!([QUERY]));
    let init_command = command!()
        .name("init")
        .arg(arg!(--name <PROJECT_NAME>))
        .arg(arg!(--children ...));
    let cli = command!()
        .arg(arg!([MESSAGE]))
        .subcommand(find_subcommand)
        .subcommand(init_command);
    return cli;
}

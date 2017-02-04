use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let setup_subcommand = SubCommand::with_name("authenticate")
        .about("Setup Sherpa CLI")
        .arg(Arg::with_name("token")
             .help("Personal Github Access Token")
             .index(1)
             .required(true));

    App::new("sherpa")
        .version("0.1")
        .setting(AppSettings::VersionlessSubcommands)
        .after_help("You can also run `sherpa SUBCOMMAND -h` to get more information about that subcommand.")
        .subcommand(setup_subcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

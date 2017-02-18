use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let setup_subcommand = SubCommand::with_name("auth")
        .about("Setup Sherpa CLI")
        .arg(Arg::with_name("handle")
             .help("Github handle")
             .index(1)
             .required(true))
        .arg(Arg::with_name("token")
             .help("Personal Github Access Token\nThe token needs to have read:org permissions")
             .index(2)
             .required(true));

    let deploy_subcommand = SubCommand::with_name("deploy")
        .about("Deploy to a stage")
        .arg(Arg::with_name("stage")
             .help("Sherpa stage")
             .index(1)
             .required(true));

    App::new("sherpa")
        .version("0.1")
        .setting(AppSettings::VersionlessSubcommands)
        .after_help("You can also run `sherpa SUBCOMMAND -h` to get more information about that subcommand.")
        .subcommand(setup_subcommand)
        .subcommand(deploy_subcommand)
        .arg(Arg::with_name("config")
             .takes_value(true)
             .help("Override config directory")
             .short("c")
             .long("config"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

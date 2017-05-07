use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let start_subcommand = SubCommand::with_name("start")
        .about("Start a team for mobbing")
        .arg(Arg::with_name("members")
            .help("Names for mob")
            .index(1)
            .required(true))
        .arg(Arg::with_name("minutes")
            .help("Amount of time per driver")
            .takes_value(true)
            .long("minutes")
            .short("m"));

    let prompt_subcommand = SubCommand::with_name("prompt")
        .setting(AppSettings::Hidden)
        .arg(Arg::with_name("next_driver")
            .index(1)
            .required(true));

    let server_subcommand = SubCommand::with_name("server")
        .about("Start server for mob");

    App::new("history")
        .version("0.1")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .after_help("You can also run `mob SUBCOMMAND -h` to get more information about that \
                     subcommand.")
        .subcommand(start_subcommand)
        .subcommand(prompt_subcommand)
        .subcommand(server_subcommand)
}

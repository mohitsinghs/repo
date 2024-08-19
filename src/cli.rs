use clap::{crate_description, crate_name, crate_version, Arg, ArgAction, ArgMatches, Command};
use std::env;

pub fn parse() -> ArgMatches {
    let cmd = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(Command::new("init").about("Initiate config"))
        .subcommand(
            Command::new("sh")
                .about("Generate shell completions")
                .args(&[
                    Arg::new("shell")
                        .required(false)
                        .num_args(1)
                        .value_parser(["zsh", "bash", "fish"])
                        .help("Current shell to generate completions for"),
                    Arg::new("bind")
                        .long("bind")
                        .short('b')
                        .num_args(1)
                        .required(false)
                        .default_value("z")
                        .help("Word or character to bind with"),
                ]),
        )
        .subcommand(
            Command::new("cmp")
                .args(&[
                    Arg::new("term").value_delimiter('/'),
                    Arg::new("full")
                        .long("full")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["json", "tree"])
                        .help("Print repo paths separated by newline"),
                    Arg::new("json")
                        .long("json")
                        .conflicts_with("tree")
                        .short('j')
                        .action(ArgAction::SetTrue)
                        .help("Print matches as json list"),
                    Arg::new("tree")
                        .long("tree")
                        .short('t')
                        .action(ArgAction::SetTrue)
                        .help("Print matches as json tree"),
                    Arg::new("config")
                        .long("config")
                        .short('c')
                        .help("Config path or config JSON")
                        .action(ArgAction::Set)
                        .required(false),
                ])
                .about("Generate completions"),
        )
        .subcommand(
            Command::new("cd")
                .arg(Arg::new("target").value_delimiter('/'))
                .arg_required_else_help(true)
                .about("Change to given directory"),
        );
    cmd.get_matches()
}

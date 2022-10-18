use crate::sh::get_current_shell;
use clap::{crate_description, crate_name, crate_version, Arg, ArgMatches, Command};
use std::env;

pub fn parse() -> ArgMatches {
    let shell = get_current_shell().unwrap_or_else(|| "zsh".to_string());
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
                        .takes_value(true)
                        .possible_values(["zsh", "bash", "fish"])
                        .default_value(&shell)
                        .help("Current shell to generate completions for"),
                    Arg::new("bind")
                        .long("bind")
                        .short('b')
                        .takes_value(true)
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
                        .required(false)
                        .conflicts_with_all(&["json", "tree"])
                        .help("Print repo paths separated by newline"),
                    Arg::new("json")
                        .long("json")
                        .conflicts_with("tree")
                        .short('j')
                        .required(false)
                        .help("Print matches as json list"),
                    Arg::new("tree")
                        .long("tree")
                        .short('t')
                        .required(false)
                        .help("Print matches as json tree"),
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

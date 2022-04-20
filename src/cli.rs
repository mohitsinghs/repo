use crate::sh::get_current_shell;
use clap::{crate_description, crate_name, crate_version, Arg, ArgMatches, Command};
use std::env;

pub fn parse() -> ArgMatches {
    let shell = get_current_shell().unwrap_or_else(|| "zsh".to_string());
    let cmd = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(Command::new("init").about("initiate config"))
        .subcommand(
            Command::new("sh")
                .about("generate shell completions")
                .args(&[
                    Arg::new("shell")
                        .required(false)
                        .takes_value(true)
                        .possible_values(["zsh", "bash", "fish"])
                        .default_value(&shell)
                        .help("current shell to generate completions for"),
                    Arg::new("bind")
                        .long("bind")
                        .short('b')
                        .takes_value(true)
                        .required(false)
                        .default_value("z")
                        .help("word or character to bind with"),
                ]),
        )
        .subcommand(
            Command::new("cmp")
                .arg(Arg::new("term"))
                .about("generate completions"),
        )
        .subcommand(
            Command::new("cd")
                .arg(Arg::new("target"))
                .arg_required_else_help(true)
                .about("change to given directory"),
        );
    cmd.get_matches()
}

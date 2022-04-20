use crate::detector::as_path_names;
use config::Conf;
use detector::{find_path, traverse_roots};
use sh::print_completions;
use std::process::exit;

mod cli;
mod config;
mod detector;
mod sh;

fn main() {
    let matches = cli::parse();
    let conf = Conf::read()
        .map_err(|err| {
            println!("failed to read config: {}", err);
        })
        .unwrap_or_default();
    if let Some((name, args)) = matches.subcommand() {
        match name {
            "init" => {
                Conf::write()
                    .map_err(|err| {
                        println!("failed to write config : {:?}", err);
                        exit(1);
                    })
                    .unwrap();
            }
            "sh" => {
                let shell = args.value_of("shell");
                let binding = args.value_of("bind").unwrap_or("x");
                print_completions(shell, binding)
            }
            "cmp" => {
                let term = args.value_of("term");
                let repos = traverse_roots(conf.roots, term);
                println!("{}", as_path_names(repos).join(" "));
            }
            "cd" => {
                if let Some(term) = args.value_of("target") {
                    let repos = traverse_roots(conf.roots, None);
                    if let Some(dir) = find_path(term, repos) {
                        println!("{}", dir.path().to_str().unwrap())
                    }
                }
            }
            _ => {
                panic!("Unknown command")
            }
        }
    }
}

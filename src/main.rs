use config::Conf;
use detector::traverse_roots;
use matcher::find_match;
use sh::print_completions;
use slog::{error, info, Level};
use std::{env, process::exit};
use transformer::{as_json, as_path_names, as_paths, as_tree};

mod cli;
mod config;
mod detector;
mod logger;
mod matcher;
mod path_extra;
mod sh;
mod transformer;

fn main() {
    let matches = cli::parse();
    let log = if env::var_os("INFO").is_some() {
        logger::logger(Level::Info).unwrap()
    } else {
        logger::logger(Level::Warning).unwrap()
    };
    info!(log, "inf logger initiated");
    let conf = Conf::read()
        .map_err(|err| {
            error!(log, "failed to read config"; "error" => #%err);
        })
        .unwrap_or_default();
    if let Some((name, args)) = matches.subcommand() {
        match name {
            "init" => {
                Conf::write()
                    .map_err(|err| {
                        error!(log, "failed to write config"; "error" => #%err);
                        exit(1);
                    })
                    .unwrap();
            }
            "sh" => {
                let shell = args.value_of("shell");
                let binding = args.value_of("bind").unwrap_or("x");
                print_completions(shell, binding, log)
            }
            "cmp" => {
                let repos = traverse_roots(conf.roots, args.value_of("term"), log.clone());
                info!(log, "scanned roots"; "count" => repos.len());
                info!(log, "converting roots");
                if args.is_present("full") {
                    println!("{}", as_paths(repos).join("\n"));
                } else if args.is_present("json") {
                    println!("{}", as_json(repos).expect("failed to parse json"));
                } else if args.is_present("tree") {
                    println!("{}", as_tree(repos).expect("failed to parse json"));
                } else {
                    println!("{}", as_path_names(repos).join(" "));
                }
                info!(log, "finished");
            }
            "cd" => {
                if let Some(terms) = args.values_of("target") {
                    let repos = traverse_roots(conf.roots, None, log.clone());
                    info!(log, "scanned roots"; "count" => repos.len());
                    if let Some(dir) = find_match(terms.collect(), repos) {
                        println!("{}", dir.to_str().unwrap())
                    }
                    info!(log, "matching completed");
                }
            }
            _ => {
                error!(log, "unknown command");
                exit(1);
            }
        }
    }
}

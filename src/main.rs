use config::Conf;
use detector::traverse_roots;
use matcher::find_match;
use sh::print_completions;
use std::process::exit;
use transformer::{as_json, as_path_names, as_paths, as_tree};

mod cli;
mod config;
mod detector;
mod matcher;
mod path_extra;
mod sh;
mod transformer;

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
                let term: Option<Vec<&str>> = args.values_of("term").map(|v| v.collect());
                let full = args.is_present("full");
                let is_json = args.is_present("json");
                let is_tree = args.is_present("tree");
                let repos = traverse_roots(conf.roots, term);
                if full {
                    println!("{}", as_paths(repos).join("\n"));
                } else if is_json {
                    println!("{}", as_json(repos).expect("failed to parse json"));
                } else if is_tree {
                    println!("{}", as_tree(repos).expect("failed to parse json"));
                } else {
                    println!("{}", as_path_names(repos).join(" "));
                }
            }
            "cd" => {
                if let Some(terms) = args.values_of("target") {
                    let repos = traverse_roots(conf.roots, None);
                    if let Some(dir) = find_match(terms.collect(), repos) {
                        println!("{}", dir.to_str().unwrap())
                    }
                }
            }
            _ => {
                panic!("Unknown command")
            }
        }
    }
}

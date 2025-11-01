use config::Conf;
use detector::traverse_roots;
use matcher::find_match;
use sh::{get_current_shell, print_completions};
use std::{fs, path::Path, process::exit};
use transformer::{as_json, as_path_names, as_paths, as_tree};

mod cli;
mod config;
mod detector;
mod git;
mod matcher;
mod path_extra;
mod sh;
mod transformer;

fn main() {
    let matches = cli::parse();
    let mut conf = Conf::read()
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
                let current_shell = get_current_shell().unwrap_or("zsh".to_string());
                let shell = args.get_one::<String>("shell").unwrap_or(&current_shell);
                let binding = args.get_one::<String>("bind").unwrap();
                print_completions(shell, binding)
            }
            "cmp" => {
                let cfg = args.get_one::<String>("config");
                if let Some(input) = cfg {
                    if !input.is_empty() {
                        conf = if Path::new(input).exists() {
                            let value =
                                fs::read_to_string(input).expect("failed to read config file");
                            serde_json::from_str(&value).expect("failed to read json config")
                        } else {
                            serde_json::from_str(&input).expect("failed to parse json config")
                        };
                    }
                }

                let terms = args
                    .get_many::<String>("term")
                    .unwrap_or_default()
                    .map(|v| v.as_str())
                    .collect();
                let repos = traverse_roots(conf.roots, Some(terms));

                if repos.is_empty() {
                    return;
                };

                if args.get_flag("full") {
                    println!("{}", as_paths(repos).join("\n"));
                } else if args.get_flag("json") {
                    println!("{}", as_json(repos).expect("failed to parse json"));
                } else if args.get_flag("tree") {
                    println!("{}", as_tree(repos).expect("failed to parse json"));
                } else {
                    println!("{}", as_path_names(repos).join(" "));
                }
            }
            "cd" => {
                let terms: Vec<&str> = args
                    .get_many::<String>("target")
                    .unwrap()
                    .map(|v| v.as_str())
                    .collect();

                let repos = traverse_roots(conf.roots, None);
                if let Some(dir) = find_match(terms, repos) {
                    println!("{}", dir.to_str().unwrap())
                }
            }
            _ => {
                panic!("Unknown command")
            }
        }
    }
}

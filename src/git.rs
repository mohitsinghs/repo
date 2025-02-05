use std::{collections::HashMap, fs::read_to_string};

use dirs::home_dir;

pub fn extract_ssh_host(url: &str) -> Option<String> {
    url.strip_prefix("git@")?
        .split(":")
        .next()
        .map(String::from)
}

pub fn detect_provider(url: &str) -> String {
    if url.contains("github.com") {
        "github".to_string()
    } else if url.contains("gitlab.com") {
        "gitlab".to_string()
    } else if url.contains("bitbucket.org") {
        "bitbucket".to_string()
    } else if url.contains("ssh.dev.azure.com") {
        "azure".to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn load_aliases() -> HashMap<String, String> {
    let mut aliases = HashMap::new();
    if let Some(home) = home_dir() {
        if let Ok(config) = read_to_string(home.join(".ssh/config")) {
            let mut lines = config.lines();
            let mut current_alias = None;

            while let Some(line) = lines.next() {
                let trimmed = line.trim();
                if let Some(alias) = trimmed
                    .strip_prefix("Host ")
                    .map(str::trim)
                    .map(str::to_string)
                {
                    current_alias = Some(alias)
                } else if let Some(hostname) = trimmed
                    .strip_prefix("Hostname ")
                    .or_else(|| trimmed.strip_prefix("HostName "))
                    .and_then(|s| s.split_whitespace().next())
                {
                    if let Some(alias) = &current_alias {
                        aliases.insert(alias.clone(), hostname.to_string());
                    }
                }
            }
        }
    }
    aliases
}

pub fn resolve_url(url: &str, aliases: &HashMap<String, String>) -> String {
    if let Some(alias) = extract_ssh_host(url) {
        if let Some(resolved) = aliases.get(&alias) {
            return url.replacen(&alias, resolved, 1);
        }
    }
    url.to_string()
}

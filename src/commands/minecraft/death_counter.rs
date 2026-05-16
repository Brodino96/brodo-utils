use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Args;
use regex::Regex;

const KILL_PATTERN: &str = r"'([^']+)' killed '([^']+)'";

#[derive(Args)]
pub struct DeathCounterArgs {
    /// Log file to parse for death events
    pub input: PathBuf,
}

pub fn run(args: DeathCounterArgs) {
    let content = match fs::read_to_string(&args.input) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Error: file not found: {}", args.input.display());
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error reading {}: {}", args.input.display(), e);
            process::exit(1);
        }
    };

    let re = Regex::new(KILL_PATTERN).expect("invalid kill regex");

    let mut order: Vec<String> = Vec::new();
    let mut deaths: HashMap<String, usize> = HashMap::new();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let victim = caps.get(2).unwrap().as_str().to_string();

            let entry = deaths.entry(victim.clone()).or_insert_with(|| {
                order.push(victim);
                0
            });
            *entry += 1;
        }
    }

    let name_width = order.iter().map(|k| k.len()).max().unwrap_or(0);
    let count_width = order
        .iter()
        .map(|k| deaths[k].to_string().len())
        .max()
        .unwrap_or(0);

    for player in &order {
        println!(
            "{:<name_width$} died {:>count_width$}",
            player,
            deaths[player]
        );
    }
}

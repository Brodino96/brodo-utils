use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Args;
use regex::Regex;

const KILL_PATTERN: &str = r"'([^']+)' killed '([^']+)'";

#[derive(Args)]
pub struct KillCounterArgs {
    /// Log file to parse for kill events
    pub input: PathBuf,
}

pub fn run(args: KillCounterArgs) {
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

    // Insertion-order keys + per-killer stats
    let mut order: Vec<String> = Vec::new();
    let mut stats: HashMap<String, (usize, HashSet<String>)> = HashMap::new();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let killer = caps.get(1).unwrap().as_str().to_string();
            let victim = caps.get(2).unwrap().as_str().to_string();

            let entry = stats.entry(killer.clone()).or_insert_with(|| {
                order.push(killer);
                (0, HashSet::new())
            });
            entry.0 += 1;
            entry.1.insert(victim);
        }
    }

    let name_width = order.iter().map(|k| k.len()).max().unwrap_or(0);
    let total_width = order
        .iter()
        .map(|k| stats[k].0.to_string().len())
        .max()
        .unwrap_or(0);

    for killer in &order {
        let (total, victims) = &stats[killer];
        println!(
            "{:<name_width$} killed {:>total_width$} - Unique kills: {}",
            killer,
            total,
            victims.len()
        );
    }
}

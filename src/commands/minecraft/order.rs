use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;

use chrono::NaiveDateTime;
use clap::Args;
use regex::Regex;

const TIMESTAMP_PATTERN: &str = r"^\[(\d{2}/\d{2}/\d{4} - \d{2}:\d{2}:\d{2})\]";
const TIMESTAMP_FORMAT: &str = "%d/%m/%Y - %H:%M:%S";

#[derive(Args)]
pub struct OrderArgs {
    /// Input log file to sort
    pub input: PathBuf,

    /// Output file (defaults to stdout if not specified)
    pub output: Option<PathBuf>,
}

pub fn run(args: OrderArgs) {
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

    let re = Regex::new(TIMESTAMP_PATTERN).expect("invalid timestamp regex");

    let mut parsed: Vec<(NaiveDateTime, &str)> = Vec::new();
    let mut unparsed: Vec<&str> = Vec::new();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let ts_str = caps.get(1).unwrap().as_str();
            match NaiveDateTime::parse_from_str(ts_str, TIMESTAMP_FORMAT) {
                Ok(ts) => parsed.push((ts, line)),
                Err(_) => unparsed.push(line),
            }
        } else {
            unparsed.push(line);
        }
    }

    parsed.sort_by_key(|(ts, _)| *ts);

    let mut result = String::new();
    for (_, line) in &parsed {
        result.push_str(line);
        result.push('\n');
    }

    if !unparsed.is_empty() {
        result.push('\n');
        result.push_str("# Lines without a recognized timestamp:\n");
        for line in &unparsed {
            result.push_str(line);
            result.push('\n');
        }
    }

    match &args.output {
        Some(out_path) => {
            match fs::File::create(out_path) {
                Ok(mut f) => {
                    if let Err(e) = f.write_all(result.as_bytes()) {
                        eprintln!("Error writing to {}: {}", out_path.display(), e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error creating {}: {}", out_path.display(), e);
                    process::exit(1);
                }
            }
            println!("Sorted {} lines -> {}", parsed.len(), out_path.display());
            if !unparsed.is_empty() {
                println!(
                    "  {} line(s) had no timestamp and were appended at the end.",
                    unparsed.len()
                );
            }
        }
        None => print!("{}", result),
    }
}

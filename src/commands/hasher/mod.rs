use clap::Args;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

mod algorithms;

use algorithms::{HashAlgorithm, AVAILABLE_ALGORITHMS};

#[derive(Args, Debug)]
pub struct HasherArgs {
    path: Option<String>,

    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    #[arg(short = 'f', long = "function")]
    hash_function: Option<String>,

    #[arg(short = 'l', long = "list")]
    list: bool,

    #[arg(short = 'o', long = "output")]
    output: Option<String>,
}

pub fn run(args: HasherArgs) {
    if args.list {
        list_available_algorithms();
        return;
    }

    let path = match &args.path {
        Some(p) => p.clone(),
        None => {
            print_missing_args_error(true, args.hash_function.is_none());
            std::process::exit(1);
        }
    };

    let hash_function = match &args.hash_function {
        Some(f) => f.clone(),
        None => {
            print_missing_args_error(args.path.is_none(), true);
            std::process::exit(1);
        }
    };

    let algorithm = match HashAlgorithm::from_name(&hash_function) {
        Some(algo) => algo,
        None => {
            eprintln!("Error: Unknown hash function '{}'", hash_function);
            eprintln!("Use --list to see available hash functions.");
            std::process::exit(1);
        }
    };

    let path_ref = Path::new(&path);
    if !path_ref.exists() {
        eprintln!("Error: Path '{}' does not exist.", path);
        std::process::exit(1);
    }

    let results = if path_ref.is_file() {
        match hash_file(path_ref, &algorithm) {
            Ok(hash) => {
                let filename = path_ref.file_name().unwrap_or_default().to_string_lossy();
                vec![(hash, filename.to_string())]
            }
            Err(e) => {
                eprintln!("Error hashing file '{}': {}", path, e);
                std::process::exit(1);
            }
        }
    } else if path_ref.is_dir() {
        process_directory(path_ref, &algorithm, args.recursive)
    } else {
        eprintln!("Error: '{}' is neither a file nor a directory.", path);
        std::process::exit(1);
    };

    if let Some(output_path) = args.output {
        if let Err(e) = write_results_to_file(&output_path, &results) {
            eprintln!("Error writing to '{}': {}", output_path, e);
            std::process::exit(1);
        }
        println!("Hashes written to '{}'", output_path);
    } else {
        for (hash, filename) in &results {
            println!("{} - {}", hash, filename);
        }
    }
}

fn list_available_algorithms() {
    println!("Available hash functions:");
    println!();
    for (name, description) in AVAILABLE_ALGORITHMS {
        println!("  {:<12} {}", name, description);
    }
}

fn print_missing_args_error(missing_path: bool, missing_function: bool) {
    eprintln!("Error: Missing required arguments.");
    if missing_path {
        eprintln!("  - No path specified.");
    }
    if missing_function {
        eprintln!("  - No hash function specified.");
    }
    eprintln!();
    eprintln!("Try running:");
    eprintln!("  brodo-utils hasher . -f SHA-256");
    eprintln!();
    eprintln!("Use --help for more information.");
    eprintln!("Use --list to see available hash functions.");
}

fn process_directory(
    dir: &Path,
    algorithm: &HashAlgorithm,
    recursive: bool,
) -> Vec<(String, String)> {
    let walker = if recursive {
        WalkDir::new(dir)
    } else {
        WalkDir::new(dir).max_depth(1)
    };

    let mut results = Vec::new();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let relative_path = match path.strip_prefix(dir) {
            Ok(rel) => rel.to_string_lossy().to_string(),
            Err(_) => path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        };

        let relative_path = relative_path.replace('\\', "/");

        match hash_file(path, algorithm) {
            Ok(hash) => {
                results.push((hash, relative_path));
            }
            Err(e) => {
                eprintln!("Error hashing '{}': {}", relative_path, e);
            }
        }
    }

    results
}

fn write_results_to_file(path: &str, results: &[(String, String)]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for (hash, filename) in results {
        writeln!(writer, "{} - {}", hash, filename)?;
    }

    writer.flush()?;
    Ok(())
}

fn hash_file(path: &Path, algorithm: &HashAlgorithm) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];

    let mut hasher = algorithm.create_hasher();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize_hex())
}

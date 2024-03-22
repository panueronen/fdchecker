use clap::Parser;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{PathBuf, Path};
use walkdir::WalkDir;

/// Simple duplicate file checker
#[derive(Parser, Debug)]
#[command(version, about="a simple duplicate file checker")]
struct Args {
    ///directory to check for duplicates
    #[arg(short, long)]
    directory:String,
    
    ///print output to a file
    #[arg(short,long)]
    textfile:bool,
}
fn main() {
    
    let args = Args::parse();

    let path = args.directory;

    let output_mode = if args.textfile {
        OutputMode::ToFile
    } else {
        // Default to print if --to-file is not specified
        OutputMode::Print
    };

    match find_duplicates(&path) {
        Ok(duplicates) => {
            match output_mode {
                OutputMode::Print => print_duplicates(&duplicates),
                OutputMode::ToFile => write_duplicates_to_file(&duplicates, "duplicates.txt").unwrap(),
            }
        }
        Err(e) => println!("Error: {}", e),
    }

}
fn find_duplicates(path: &str) -> io::Result<HashMap<String, Vec<PathBuf>>> {
    let mut file_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            match calculate_file_hash(path) {
                Ok(hash) => {
                    file_map.entry(hash).or_insert_with(Vec::new).push(path.to_path_buf());
                },
                Err(e) => println!("Error hashing file {}: {}", path.display(), e),
            }
        }
    }

    Ok(file_map.into_iter().filter(|(_, v)| v.len() > 1).collect())
}
fn calculate_file_hash(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

enum OutputMode {
    Print,
    ToFile,
}


fn print_duplicates(duplicates: &HashMap<String, Vec<PathBuf>>) {
    if duplicates.is_empty() {
        println!("No duplicates found.");
    } else {
        for (hash, files) in duplicates {
            println!("Duplicate hash: {}", hash);
            for file in files {
                println!("  - {}", file.display());
            }
        }
    }
}

fn write_duplicates_to_file(duplicates: &HashMap<String, Vec<PathBuf>>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    if duplicates.is_empty() {
        writeln!(file, "No duplicates found.")?;
    } else {
        for (hash, files) in duplicates {
            writeln!(file, "Duplicate hash: {}", hash)?;
            for file_path in files {
                writeln!(file, "  - {}", file_path.display())?;
            }
        }
    }
    Ok(())
}


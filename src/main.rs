use std::io;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;
use walkdir::WalkDir;
use sha2::{Digest, Sha256};

fn main() {
    println!("Enter the directory path to search for duplicate files:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line");
    let path = path.trim();

    println!("Would you like to (1) print the duplicates to the console or (2) write them to a file? Enter 1 or 2:");
    let mut output_choice = String::new();
    io::stdin().read_line(&mut output_choice).expect("Failed to read line");
    let output_mode = output_choice.trim();

    match find_duplicates(path) {
        Ok(duplicates) => {
            match output_mode {
                "1" => print_duplicates(&duplicates),
                "2" => {
                    let filename = "duplicates.txt";
                    if let Err(e) = write_duplicates_to_file(&duplicates, filename) {
                        println!("Failed to write to file: {}", e);
                    } else {
                        println!("Duplicates written to {}", filename);
                    }
                },
                _ => println!("Invalid option selected. Please enter 1 or 2 next time."),
            }
        },
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

fn print_duplicates(duplicates: &HashMap<String, Vec<PathBuf>>) {
    if duplicates.is_empty() {
        println!("No duplicates found.");
    } else {
        for files in duplicates.values() {
            for file in files {
                println!("{}", file.display());
            }
            println!("---"); // Separates groups of duplicates
        }
    }
}

fn write_duplicates_to_file(duplicates: &HashMap<String, Vec<PathBuf>>, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    if duplicates.is_empty() {
        writeln!(file, "No duplicates found.")?;
    } else {
        for files in duplicates.values() {
            for file_path in files {
                writeln!(file, "{}", file_path.display())?;
            }
            writeln!(file, "---")?; // Separates groups of duplicates
        }
    }
    Ok(())
}

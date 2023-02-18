use std::io;
use std::path::PathBuf;

mod filesystem_walker;
mod rust_file_concatenator;

use filesystem_walker::{FileSystemEntryError, FileSystemEntryResult, FileSystemWalker};
use rust_file_concatenator::RustFileConcatenator;

fn get_total_num_files() -> usize {
    let mut input = String::new();
    loop {
        println!("Enter the total number of files:");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(num) = input.trim().parse::<usize>() {
                    return num;
                } else {
                    println!("Invalid input, please enter a number.");
                    input.clear();
                }
            }
            Err(_) => {
                println!("Error reading input.");
                input.clear();
            }
        }
    }
}

pub fn run() -> FileSystemEntryResult<()> {
    // Get the input directory and output file paths from command-line arguments
    println!("Enter a path:");

    // Read the input path from the user
    let mut input_path = String::new();
    io::stdin()
        .read_line(&mut input_path)
        .map_err(|e| FileSystemEntryError::IoError(e.into()))?;

    // Remove any leading or trailing whitespace from the input path
    let input_path = input_path.trim();

    // Create a PathBuf from the input path
    let input_dir_path = PathBuf::from(input_path);

    if !input_dir_path.exists() {
        return Err(FileSystemEntryError::NotFound);
    }

    if !input_dir_path.is_dir() {
        return Err(FileSystemEntryError::NotADirectory);
    }

    // Create a PathBuf for the output file
    let output_file_path = std::env::current_dir()
        .map_err(|e| FileSystemEntryError::IoError(e.into()))?
        .join("output.rs");

    let total_num_files = get_total_num_files();
    // Create the output file
    let mut file_concatenator = RustFileConcatenator::new(&output_file_path, total_num_files);
    file_concatenator.open_output_file()?;

    // Walk the directory tree and concatenate the Rust files
    let mut file_system_walker = FileSystemWalker::new(&mut file_concatenator);
    file_system_walker.walk_directory(&input_dir_path)?;

    // Close the output file
    file_concatenator.close_output_file()?;

    Ok(())
}

use std::io;
use std::path::PathBuf;

mod filesystem_walker;
mod rust_file_concatenator;

use filesystem_walker::{FileSystemEntryError, FileSystemEntryResult, FileSystemWalker};
use rust_file_concatenator::RustFileConcatenator;

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

    // Create the output file
    let mut file_concatenator = RustFileConcatenator::new(&output_file_path);
    file_concatenator.open_output_file()?;

    // Walk the directory tree and concatenate the Rust files
    let mut file_system_walker = FileSystemWalker::new(&mut file_concatenator);
    file_system_walker.walk_directory(&input_dir_path)?;

    // Close the output file
    file_concatenator.close_output_file()?;

    Ok(())
}

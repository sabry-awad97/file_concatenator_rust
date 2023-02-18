mod error;
mod filesystem_walker;
mod rust_file_concatenator;
mod user_input;
use filesystem_walker::{FileSystemEntryResult, FileSystemWalker};

use rust_file_concatenator::RustFileConcatenator;
use user_input::UserInput;

use crate::error::ProgramError;

pub fn run() -> FileSystemEntryResult<()> {
    let user_input = UserInput::new()?;

    // Create a PathBuf for the output file
    let output_file_path = std::env::current_dir()
        .map_err(|e| ProgramError::IoError(e.into()))?
        .join("output.rs");

    // Create the output file
    let mut file_concatenator =
        RustFileConcatenator::new(&output_file_path, user_input.total_files);
    file_concatenator.open_output_file()?;

    // Walk the directory tree and concatenate the Rust files
    let mut file_system_walker = FileSystemWalker::new(&mut file_concatenator);
    file_system_walker.walk_directory(&user_input.path)?;

    // Close the output file
    file_concatenator.close_output_file()?;

    Ok(())
}

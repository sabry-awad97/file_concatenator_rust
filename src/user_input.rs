use std::{
    io::{self, Write},
    path::PathBuf,
};

use crate::error::ProgramError;

// Define a struct to handle user input
pub struct UserInput {
    pub path: PathBuf,
    pub total_files: usize,
}

impl UserInput {
    // Create a new UserInput object by reading user input from the console
    pub fn new() -> Result<Self, ProgramError> {
        let path = UserInput::get_input_path()?;
        let total_files = UserInput::get_total_num_files()?;
        Ok(UserInput { path, total_files })
    }

    // Prompt the user to enter the path to the input directory
    pub fn get_input_path() -> Result<PathBuf, ProgramError> {
        // Get the input directory and output file paths from command-line arguments
        print!("Enter the path to the input directory: ");

        let mut input_path = String::new();

        // Read the input path from the user
        io::stdin()
            .read_line(&mut input_path)
            .map_err(|e| ProgramError::IoError(e.into()))?;
        // Remove any leading or trailing whitespace from the input path
        let input_path = input_path.trim();

        // Create a PathBuf from the input path
        let input_dir_path = PathBuf::from(input_path);

        if !input_dir_path.exists() {
            return Err(ProgramError::NotFound.into());
        }

        if !input_dir_path.is_dir() {
            return Err(ProgramError::NotADirectory.into());
        }

        Ok(input_dir_path)
    }

    // Prompt the user to enter the total number of files to concatenate
    pub fn get_total_num_files() -> Result<usize, ProgramError> {
        let mut input = String::new();
        loop {
            print!("Enter the total number of files to concatenate: ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut input)
                .map_err(|e| ProgramError::IoError(e).into())?;

            match input.trim().parse() {
                Ok(num) if num > 0 => return Ok(num),
                _ => {
                    println!("Invalid input. Please enter a positive integer.");
                    input.clear();
                }
            }
        }
    }
}

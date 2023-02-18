use std::io;

// The possible errors that can occur when processing file system entries
#[derive(Debug)]
pub enum ProgramError {
    NotFound,
    NotADirectory,
    AlreadyExists,
    InvalidInput,
    IoError(io::Error),
}

// Implement the std::error::Error trait for our custom error type
impl std::error::Error for ProgramError {}

// Implement the Display trait for our custom error type
impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramError::NotFound => write!(f, "File or directory not found"),
            ProgramError::NotADirectory => write!(f, "Path is not a directory"),
            ProgramError::AlreadyExists => write!(f, "File or directory already exists"),
            ProgramError::InvalidInput => write!(f, "Invalid input"),
            ProgramError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

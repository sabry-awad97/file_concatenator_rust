use std::{
    fs, io,
    path::{Path, PathBuf},
};

// The possible types of file system entries
#[derive(PartialEq)]
pub enum FileSystemEntryType {
    File,
    Directory,
}

// A file system entry
pub struct FileSystemEntry {
    pub path: PathBuf,
    pub entry_type: FileSystemEntryType,
}

// The possible errors that can occur when processing file system entries
#[derive(Debug)]
pub enum FileSystemEntryError {
    NotFound,
    NotADirectory,
    AlreadyExists,
    InvalidInput,
    IoError(io::Error),
}

// Implement the std::error::Error trait for our custom error type
impl std::error::Error for FileSystemEntryError {}

// Implement the Display trait for our custom error type
impl std::fmt::Display for FileSystemEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystemEntryError::NotFound => write!(f, "File or directory not found"),
            FileSystemEntryError::NotADirectory => write!(f, "Path is not a directory"),
            FileSystemEntryError::AlreadyExists => write!(f, "File or directory already exists"),
            FileSystemEntryError::InvalidInput => write!(f, "Invalid input"),
            FileSystemEntryError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

// A wrapper around `io::Result` that maps `io::Error` to `FileSystemEntryError`
pub type FileSystemEntryResult<T> = Result<T, FileSystemEntryError>;

// A trait for processing file system entries
pub trait FileSystemEntryProcessor {
    fn process_entry(&mut self, entry: &FileSystemEntry) -> FileSystemEntryResult<()>;
}

// A recursive file system walker
pub struct FileSystemWalker<'a> {
    processor: &'a mut dyn FileSystemEntryProcessor,
}

impl<'a> FileSystemWalker<'a> {
    pub fn new(processor: &'a mut dyn FileSystemEntryProcessor) -> Self {
        Self { processor }
    }

    pub fn walk_directory(&mut self, dir_path: &Path) -> FileSystemEntryResult<()> {
        for entry in fs::read_dir(dir_path).map_err(FileSystemEntryError::IoError)? {
            let entry = entry.map_err(FileSystemEntryError::IoError)?;
            let entry_type = if entry.file_type().map_or(false, |ft| ft.is_file()) {
                FileSystemEntryType::File
            } else if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                FileSystemEntryType::Directory
            } else {
                // Skip entries that are neither files nor directories
                continue;
            };
            let fs_entry = FileSystemEntry {
                path: entry.path(),
                entry_type,
            };
            self.processor.process_entry(&fs_entry)?;
            if fs_entry.entry_type == FileSystemEntryType::Directory {
                self.walk_directory(&fs_entry.path)?;
            }
        }
        Ok(())
    }
}

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::error::ProgramError;

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

// A wrapper around `io::Result` that maps `io::Error` to `FileSystemEntryError`
pub type FileSystemEntryResult<T> = Result<T, ProgramError>;

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
        for entry in fs::read_dir(dir_path).map_err(ProgramError::IoError)? {
            let entry = entry.map_err(ProgramError::IoError)?;
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

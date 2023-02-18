use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

// The possible types of file system entries
#[derive(PartialEq)]
enum FileSystemEntryType {
    File,
    Directory,
}

// A file system entry
struct FileSystemEntry {
    path: PathBuf,
    entry_type: FileSystemEntryType,
}

// The possible errors that can occur when processing file system entries
#[derive(Debug)]
enum FileSystemEntryError {
    NotFound,
    NotADirectory,
    AlreadyExists,
    IoError(io::Error),
}

// Implement the std::error::Error trait for our custom error type
// impl std::error::Error for FileSystemEntryError {}

// Implement the Display trait for our custom error type
impl std::fmt::Display for FileSystemEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystemEntryError::NotFound => write!(f, "File or directory not found"),
            FileSystemEntryError::NotADirectory => write!(f, "Path is not a directory"),
            FileSystemEntryError::AlreadyExists => write!(f, "File or directory already exists"),
            FileSystemEntryError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

// A wrapper around `io::Result` that maps `io::Error` to `FileSystemEntryError`
type FileSystemEntryResult<T> = Result<T, FileSystemEntryError>;

// A trait for processing file system entries
trait FileSystemEntryProcessor {
    fn process_entry(&mut self, entry: &FileSystemEntry) -> FileSystemEntryResult<()>;
}

// A recursive file system walker
struct FileSystemWalker<'a> {
    processor: &'a mut dyn FileSystemEntryProcessor,
}

impl<'a> FileSystemWalker<'a> {
    fn new(processor: &'a mut dyn FileSystemEntryProcessor) -> Self {
        Self { processor }
    }

    fn walk_directory(&mut self, dir_path: &Path) -> FileSystemEntryResult<()> {
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

// A processor that concatenates Rust files into a single output file
struct RustFileConcatenator<'a> {
    output_file_path: &'a Path,
    output_file: Option<File>,
}

impl<'a> RustFileConcatenator<'a> {
    fn new(output_file_path: &'a Path) -> Self {
        Self {
            output_file_path,
            output_file: None,
        }
    }

    fn open_output_file(&mut self) -> FileSystemEntryResult<()> {
        if let Some(_) = self.output_file {
            return Err(FileSystemEntryError::AlreadyExists);
        }

        self.output_file =
            Some(File::create(self.output_file_path).map_err(FileSystemEntryError::IoError)?);
        Ok(())
    }

    fn close_output_file(&mut self) -> FileSystemEntryResult<()> {
        if let Some(mut output_file) = self.output_file.take() {
            output_file.flush().map_err(FileSystemEntryError::IoError)?;
        }
        Ok(())
    }
}

impl<'a> FileSystemEntryProcessor for RustFileConcatenator<'a> {
    fn process_entry(&mut self, entry: &FileSystemEntry) -> FileSystemEntryResult<()> {
        match entry.entry_type {
            FileSystemEntryType::File => {
                if let Some(output_file) = &mut self.output_file {
                    if entry.path.extension().map_or(false, |ext| ext == "rs") {
                        let mut input_file =
                            File::open(&entry.path).map_err(FileSystemEntryError::IoError)?;
                        let mut contents = String::new();
                        input_file
                            .read_to_string(&mut contents)
                            .map_err(FileSystemEntryError::IoError)?;

                        writeln!(output_file, "// Start of file: {}", entry.path.display())
                            .map_err(FileSystemEntryError::IoError)?;
                        output_file
                            .write_all(contents.as_bytes())
                            .map_err(FileSystemEntryError::IoError)?;
                        writeln!(output_file, "// End of file: {}", entry.path.display())
                            .map_err(FileSystemEntryError::IoError)?;
                    }
                }
            }
            FileSystemEntryType::Directory => {}
        }
        Ok(())
    }
}

fn main() -> FileSystemEntryResult<()> {
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

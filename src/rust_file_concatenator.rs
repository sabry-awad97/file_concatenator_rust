use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::{
    filesystem_walker::{FileSystemEntry, FileSystemEntryProcessor, FileSystemEntryType},
    FileSystemEntryError, FileSystemEntryResult,
};

pub struct RustFileConcatenator<'a> {
    output_file_path: &'a Path,
    output_file: Option<File>,
}

impl<'a> RustFileConcatenator<'a> {
    pub fn new(output_file_path: &'a Path) -> Self {
        Self {
            output_file_path,
            output_file: None,
        }
    }

    pub fn open_output_file(&mut self) -> FileSystemEntryResult<()> {
        if let Some(_) = self.output_file {
            return Err(FileSystemEntryError::AlreadyExists);
        }

        self.output_file =
            Some(File::create(self.output_file_path).map_err(FileSystemEntryError::IoError)?);
        Ok(())
    }

    pub fn close_output_file(&mut self) -> FileSystemEntryResult<()> {
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

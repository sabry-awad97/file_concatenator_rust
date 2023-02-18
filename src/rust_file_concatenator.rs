use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    time::{Duration, Instant},
};

use crate::{
    filesystem_walker::{FileSystemEntry, FileSystemEntryProcessor, FileSystemEntryType},
    FileSystemEntryError, FileSystemEntryResult,
};

pub struct RustFileConcatenator<'a> {
    output_file_path: &'a Path,
    output_file: Option<File>,
    num_files_processed: usize,
    total_num_files: usize,
    start_time: Option<Instant>,
}

impl<'a> RustFileConcatenator<'a> {
    pub fn new(output_file_path: &'a Path, total_num_files: usize) -> Self {
        Self {
            output_file_path,
            output_file: None,
            num_files_processed: 0,
            total_num_files,
            start_time: None,
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

    fn update_progress(&mut self) {
        self.num_files_processed += 1;
        if let None = self.start_time {
            self.start_time = Some(Instant::now());
        }
    }

    fn elapsed_time(&self) -> Option<Duration> {
        self.start_time.map(|start_time| start_time.elapsed())
    }

    fn progress_message(&self) -> String {
        let elapsed_time = self.elapsed_time().unwrap_or_default();
        format!(
            "Processed {} of {} files in {}",
            self.num_files_processed,
            self.total_num_files,
            humantime::format_duration(elapsed_time),
        )
    }

    
}

impl<'a> FileSystemEntryProcessor for RustFileConcatenator<'a> {
    fn process_entry(&mut self, entry: &FileSystemEntry) -> FileSystemEntryResult<()> {
        self.update_progress();

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

        let progress = self.progress_message();
        println!("{}", progress);
        Ok(())
    }
}

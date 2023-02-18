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
    pub fn new(output_file_path: &'a Path) -> Self {
        Self {
            output_file_path,
            output_file: None,
            num_files_processed: 0,
            total_num_files: 0,
            start_time: None,
        }
    }

    pub fn open_output_file(&mut self) -> FileSystemEntryResult<()> {
        if let Some(_) = self.output_file {
            return Err(FileSystemEntryError::AlreadyExists);
        }

        self.output_file =
            Some(File::create(self.output_file_path).map_err(FileSystemEntryError::IoError)?);

        self.get_total_num_files();
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

    fn get_total_num_files(&mut self) {
        let mut input = String::new();
        loop {
            println!("Enter the total number of files:");
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().parse() {
                    Ok(num) => {
                        self.total_num_files = num;
                        break;
                    }
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        input.clear();
                    }
                },
                Err(_) => {
                    println!("Error reading input.");
                    input.clear();
                }
            }
        }
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

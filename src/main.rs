use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Prompt the user to enter a path
    println!("Enter a path:");

    // Read the input path from the user
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;

    // Remove any leading or trailing whitespace from the input path
    let input_path = input_path.trim();

    // Create a PathBuf from the input path
    let input_dir = PathBuf::from(input_path);

    // Ensure that the input directory exists and is readable
    if !input_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input directory not found or is not a directory",
        ));
    }

    // Create a PathBuf for the output file
    let mut output_file = std::env::current_dir()?.join("output.rs");
    output_file.set_extension("rs");

    // Ensure that the output file is writable
    if output_file.exists() && !output_file.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Output path already exists and is not a file",
        ));
    }

    // Create the output file
    let mut output = File::create(&output_file)?;

    // Recursively iterate through each entry in the input directory
    iterate_directory(&input_dir, &input_dir, &mut output)?;

    Ok(())
}

fn iterate_directory(dir_path: &Path, root_path: &Path, output: &mut File) -> io::Result<()> {
    // Loop through each entry in the directory
    for entry in dir_path.read_dir()? {
        let entry = entry?;

        // Get the path of the entry
        let path = entry.path();

        if path.is_dir() {
            // If the entry is a directory, call this function recursively
            iterate_directory(&path, root_path, output)?;
        } else if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            // If the entry is a file with the .rs extension, read its contents and write them to the output file
            let mut input = File::open(&path)?;
            let mut contents = String::new();
            input.read_to_string(&mut contents)?;
            let relative_path = path.strip_prefix(root_path).unwrap_or(&path);
            writeln!(output, "/* {} */", relative_path.display())?;
            output.write_all(contents.as_bytes())?;
        }
    }

    Ok(())
}

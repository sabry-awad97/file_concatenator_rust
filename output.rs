// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\cli\cli.rs
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub inputs: Vec<PathBuf>,
}
// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\cli\cli.rs
// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\cli\mod.rs
pub mod cli;
// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\cli\mod.rs
// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\main.rs
mod cli;
mod markdown;

use cli::cli::Cli;
use markdown::{
    html_file::HtmlFile,
    markdown::{Markdown, MarkdownError},
    preview::Preview,
};
use std::{fs, io, path::Path, thread, time::Duration};
use structopt::StructOpt;

#[derive(Debug)]
enum AppError {
    MarkdownError(MarkdownError),
    IOError(io::Error),
}

impl From<MarkdownError> for AppError {
    fn from(error: MarkdownError) -> Self {
        AppError::MarkdownError(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IOError(error)
    }
}

struct MarkdownPreview {
    markdowns: Vec<Markdown>,
    html_file: HtmlFile,
    preview: Preview,
}

impl MarkdownPreview {
    fn new(inputs: Vec<String>) -> Result<Self, AppError> {
        let markdowns = inputs.into_iter().map(Markdown::new).collect();
        let path = Path::new("preview.html");
        let html_file = HtmlFile::new(path.to_path_buf());
        let preview = Preview::new(path.to_path_buf());
        Ok(MarkdownPreview {
            markdowns,
            html_file,
            preview,
        })
    }

    fn run(&self) -> Result<(), AppError> {
        let html_output = self
            .markdowns
            .iter()
            .map(|m| m.to_html())
            .collect::<String>();
        self.html_file.write(&html_output).unwrap();
        self.preview.open().unwrap();
        thread::sleep(Duration::from_secs(1));
        self.html_file.remove().unwrap();
        Ok(())
    }
}

fn main() {
    let args = Cli::from_args();
    let inputs = args
        .inputs
        .into_iter()
        .map(|p| fs::read_to_string(&p).unwrap())
        .collect();
    let preview = MarkdownPreview::new(inputs).expect("Failed to initialize Markdown preview");
    if let Err(e) = preview.run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\main.rs
// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\html_file.rs
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum HtmlFileError {
    IOError(io::Error),
}

impl From<io::Error> for HtmlFileError {
    fn from(error: io::Error) -> Self {
        HtmlFileError::IOError(error)
    }
}

pub struct HtmlFile {
    path: PathBuf,
}

impl HtmlFile {
    pub fn new(path: PathBuf) -> Self {
        HtmlFile { path }
    }

    pub fn write(&self, html: &str) -> Result<(), HtmlFileError> {
        let mut file = File::create(&self.path)?;
        write!(file, "{}", html)?;
        Ok(())
    }

    pub fn remove(&self) -> Result<(), HtmlFileError> {
        fs::remove_file(&self.path)?;
        Ok(())
    }
}
// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\html_file.rs
// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\markdown.rs
use std::io;
use pulldown_cmark::{html, Options, Parser};

#[derive(Debug)]
pub enum MarkdownError {
    IOError(io::Error),
}

impl From<io::Error> for MarkdownError {
    fn from(error: io::Error) -> Self {
        MarkdownError::IOError(error)
    }
}

pub struct Markdown {
    input: String,
}

impl Markdown {
    pub fn new(input: String) -> Self {
        Markdown { input }
    }

    pub fn to_html(&self) -> String {
        let parser = Parser::new_ext(&self.input, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\markdown.rs
// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\mod.rs
pub mod markdown;
pub mod html_file;
pub mod preview;

// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\mod.rs
// Start of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\preview.rs
use std::{
    io,
    process::{self, Stdio},
    path::PathBuf,
};

#[derive(Debug)]
pub enum PreviewError {
    IOError(io::Error),
    ProcessError(io::Error),
    UnsupportedPlatform,
}

impl From<io::Error> for PreviewError {
    fn from(error: io::Error) -> Self {
        PreviewError::IOError(error)
    }
}

pub struct Preview {
    path: PathBuf,
}

impl Preview {
    pub fn new(path: PathBuf) -> Self {
        Preview { path }
    }

    pub fn open(&self) -> Result<(), PreviewError> {
        let command = if cfg!(windows) {
            "cmd"
        } else if cfg!(unix) || cfg!(macos) {
            "open"
        } else {
            return Err(PreviewError::UnsupportedPlatform);
        };

        process::Command::new(command)
            .args(&["/C", self.path.to_str().unwrap()])
            .stdout(Stdio::null())
            .spawn()
            .map_err(|e| PreviewError::ProcessError(e))?;

        Ok(())
    }
}
// End of file: D:\programming\Rust\rust-projects\markdown_preview\src\markdown\preview.rs

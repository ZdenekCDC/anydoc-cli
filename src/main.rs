use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

/// Convert Office/PDF/EPUB documents to Markdown.
///
/// Wraps the `anydoc` crate (https://github.com/firecrawl/anydoc, MIT
/// licensed) as a standalone, dependency-free executable.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Input document (doc, docx, odt, pdf, ppt, pptx, rtf, epub, xls, xlsx, ods, odp, csv)
    input: PathBuf,

    /// Write Markdown to this file instead of stdout
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let markdown = match anydoc::to_markdown(&args.input) {
        Ok(markdown) => markdown,
        Err(err) => {
            eprintln!("anydoc-cli: failed to convert {}: {err}", args.input.display());
            return ExitCode::FAILURE;
        }
    };

    match args.output {
        Some(path) => {
            if let Err(err) = fs::write(&path, markdown) {
                eprintln!("anydoc-cli: failed to write {}: {err}", path.display());
                return ExitCode::FAILURE;
            }
        }
        None => print!("{markdown}"),
    }

    ExitCode::SUCCESS
}

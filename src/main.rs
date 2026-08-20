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
    /// Input document(s) (doc, docx, odt, pdf, ppt, pptx, rtf, epub, xls, xlsx, ods, odp, csv).
    /// Pass several paths, or let the shell expand a glob like *.docx, to convert them in one run.
    #[arg(required = true, num_args = 1..)]
    input: Vec<PathBuf>,

    /// Write Markdown to this file instead of a sibling .md file. Only valid with a single input.
    #[arg(short, long, conflicts_with = "stdout")]
    output: Option<PathBuf>,

    /// Print Markdown to stdout instead of writing .md file(s)
    #[arg(short, long)]
    stdout: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if args.output.is_some() && args.input.len() > 1 {
        eprintln!("anydoc-cli: --output can only be used with a single input file");
        return ExitCode::FAILURE;
    }

    let mut had_error = false;

    for input in &args.input {
        let markdown = match anydoc::to_markdown(input) {
            Ok(markdown) => markdown,
            Err(err) => {
                eprintln!("anydoc-cli: failed to convert {}: {err}", input.display());
                had_error = true;
                continue;
            }
        };

        if args.stdout {
            print!("{markdown}");
            continue;
        }

        let output_path = args
            .output
            .clone()
            .unwrap_or_else(|| input.with_extension("md"));

        if let Err(err) = fs::write(&output_path, markdown) {
            eprintln!("anydoc-cli: failed to write {}: {err}", output_path.display());
            had_error = true;
        }
    }

    if had_error { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

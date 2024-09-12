use clap::Parser;
use std::path::PathBuf;

use locatarr_json_to_readme::{generate_md_table, models::Applications};

/// Cli derives a clap parser to allow us to handle command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional file to read in.
    /// If no file is provided, or the file path is `-`, Standard Input will be used instead
    json_file_path: Option<PathBuf>,
}

/// You Already Know What It Is!
/// The entry point function.
fn main() {
    let cli = Cli::parse();

    // The following match checks for whether to create an input stream from a user provided
    // file path or from the standard input stream.
    let input_stream: Box<dyn std::io::Read + 'static> = match (
        cli.json_file_path.clone(),
        cli.json_file_path
            .unwrap_or("".into())
            .to_str()
            .unwrap_or(""),
    ) {
        // Use stdin if we weren't provided a file path, or the file path was '-'
        (Some(_), "-") | (None, _) => Box::new(std::io::stdin()),

        // Otherwise, use the provided file path
        (Some(file_path), _) => {
            if let Ok(file) = std::fs::File::open(file_path) {
                Box::new(file)
            } else {
                panic!("Could not read file");
            }
        }
    };

    let apps_json: Applications = serde_json::from_reader(input_stream)
        .expect("Could not parse JSON. Improper format detected");

    let markdown_table = generate_md_table(&apps_json);

    println!("{}", markdown_table);
}

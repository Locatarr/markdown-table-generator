use clap::Parser;
use std::path::PathBuf;

use locatarr_json_to_readme::{generate_md_table, models::Applications};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional file to read in.
    /// If no file is provided, or the file path is `-`, Standard Input will be used instead
    json_file_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let input_stream: Box<dyn std::io::Read + 'static> = match (
        cli.json_file_path.clone(),
        cli.json_file_path
            .unwrap_or("".into())
            .to_str()
            .unwrap_or(""),
    ) {
        (Some(_), "-") | (None, _) => Box::new(std::io::stdin()),
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

use std::{env, path::PathBuf};

use gha_main::anyhow;
use gha_main::{gha_main, gha_output, GitHubActionResult};
use uuid::Uuid;

use markdown_table_generator::generate_md_table;
use markdown_table_generator::models::Applications;

#[gha_main]
fn main() -> GitHubActionResult {
    let runner_temp_dir: PathBuf = env::var("RUNNER_TEMP")?.into();
    let unique_id = Uuid::new_v4().to_string();

    // Get a PathBuf with the input file path
    let args: Vec<String> = env::args().collect();
    let input_file = args
        .get(1)
        .ok_or(anyhow::anyhow!("Input file not provided"))?;
    let input_filepath: PathBuf = input_file.into();

    // Open the file and get a stream
    let input_stream = std::fs::File::open(&input_filepath)?;

    // Deserialize input file into correct struct and generate markdown table
    let apps_json: Applications = serde_json::from_reader(input_stream)?;
    let md_table = generate_md_table(&apps_json);

    // Create output filepath in form "markdown-table-<original filename>.md"
    // in the runner temporary directory (gets reset after each job, but persistent across steps).
    // If there's an issue reading the filepath name, use a uuid v4 instead.
    let in_file_stem = input_filepath
        .file_stem()
        .map(|a| a.to_str().unwrap_or(&unique_id))
        .unwrap_or(&unique_id);
    let out_file_path = runner_temp_dir.join("markdown-table-".to_owned() + in_file_stem + ".md");

    // Write markdown table to output file
    std::fs::write(&out_file_path, md_table+"\n")?;

    // Export output file path as a GitHub Actions output for other steps to consume
    let outputfile = out_file_path.to_str().unwrap();
    gha_output!(outputfile);

    Ok(())
}

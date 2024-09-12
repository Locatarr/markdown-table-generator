//! Create markdown tables for [Locatarr](https://github.com/Locatarr/Locatarr)

use itertools::Itertools;

use models::{Application, Applications};

pub mod models;

/// Generate a fully valid markdown table from an [Applications] struct.
///
/// The resulting table is the minimum viable table and is not formatted in any way.
pub fn generate_md_table(apps: &Applications) -> String {
    "| **Application** | **Description** | **Github** | **Reddit** |\n|-|-|-|-|\n".to_owned() +
    &apps.applications
        .iter()
        .sorted()
        .map(generate_md_row)
        .join("\n")
}

/// Create one markdown table row from a single [Application]
///
/// Copies the [Application::name] and [Application::description] fields in, and transforms [Application::github_slug] and
/// [Application::subreddit] into proper markdown links.
fn generate_md_row(app: &Application) -> String {
    let github_link = match &app.github_slug {
        Some(slug) => format!("[{}](https://github.com/{})", slug, slug),
        None => String::new(),
    };

    let subreddit_link = match &app.subreddit {
        Some(sub) => format!("[{}](https://reddit.com/{})", sub, sub),
        None => String::new(),
    };

    // Output in table format of:
    // | Application | Description | GitHub | Reddit |
    format!(
        "| {} | {} | {} | {} |",
        app.name, app.description, github_link, subreddit_link
    )
}

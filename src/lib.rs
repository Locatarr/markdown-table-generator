use itertools::Itertools;

use models::{Application, Applications};

pub mod models;

pub fn generate_md_table(apps: &Applications) -> String {
    "| **Service** | **Usage** | **Github** | **Reddit** |\n|-------------|-----------|------------|------------|\n".to_owned() +
    &apps.applications
        .iter()
        .sorted()
        .map(generate_md_row)
        .join("\n")
}

fn generate_md_row(app: &Application) -> String {
    let github_link = match &app.github_slug {
        Some(slug) => format!("[{}](https://github.com/{})", slug, slug),
        None => String::new(),
    };

    let subreddit_link = match &app.subreddit {
        Some(sub) => format!("[{}](http://reddit.com/{})", sub, sub),
        None => String::new(),
    };

    // Output in table format of:
    // | Service | Usage | GitHub | Reddit |
    format!(
        "| {} | {} | {} | {} |",
        app.name, app.description, github_link, subreddit_link
    )
}

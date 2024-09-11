use itertools::Itertools;

use models::{Application, Applications};

mod models;

fn generate_md_table(apps: &Applications) -> String {
    apps.applications
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

fn main() {
    // TODO: Add clap for command line application usage

    let json_string = "{\"applications\":[{\"name\":\"Sonarr\",\"description\":\"Automates & manages **TV series**\",\"github_slug\":\"Sonarr/Sonarr\",\"subreddit\":\"r/Sonarr\"}, {\"name\":\"Radarr\",\"description\":\"Automates & manages **Movies**\",\"github_slug\":\"Radarr/Radarr\",\"subreddit\":\"r/Radarr\"}]}";

    let apps = serde_json::from_str::<Applications>(&json_string).unwrap();

    println!("{}", generate_md_table(&apps));
}

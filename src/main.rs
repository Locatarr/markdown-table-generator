use locatarr_json_to_readme::{generate_md_table, models::Applications};

fn main() {
    // TODO: Add clap for command line application usage

    let json_string = "{\"applications\":[{\"name\":\"Sonarr\",\"description\":\"Automates & manages **TV series**\",\"github_slug\":\"Sonarr/Sonarr\",\"subreddit\":\"r/Sonarr\"}, {\"name\":\"Radarr\",\"description\":\"Automates & manages **Movies**\",\"github_slug\":\"Radarr/Radarr\",\"subreddit\":\"r/Radarr\"}]}";

    let apps = serde_json::from_str::<Applications>(json_string).unwrap();

    println!("{}", generate_md_table(&apps));
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Applications {
    pub applications: Vec<Application>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Application {
    pub name: String,
    pub description: String,
    pub github_slug: Option<String>,
    pub subreddit: Option<String>,
}

impl Application {
    #[allow(dead_code)] // Because I like this function here, but we aren't currently using it anywhere
    pub fn new_from_strs(
        name: &str,
        description: &str,
        github_slug: Option<&str>,
        subreddit: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            github_slug: github_slug.map(|s| s.to_owned()),
            subreddit: subreddit.map(|s| s.to_owned()),
        }
    }
}

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
    pub fn new_from_strs(
        name: &str,
        description: &str,
        github_slug: Option<&str>,
        subreddit: Option<&str>,
    ) -> Self {
        let github_slug: Option<String> = match github_slug {
            Some(s) => Some(s.to_owned()),
            None => None,
        };
        let subreddit: Option<String> = match subreddit {
            Some(s) => Some(s.to_owned()),
            None => None,
        };

        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            github_slug,
            subreddit,
        }
    }
}

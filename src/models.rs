//! The models sub-crate holds all of the general purpose structs used throughout the library.

use serde::{Deserialize, Serialize};

/// Struct for the collection of a list of applications so that [serde] can deserialize a JSON
/// file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Applications {
    pub applications: Vec<Application>,
}

/// Holding struct for each application to place in a table
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Application {
    /// The name of the application/service
    pub name: String,
    /// The description of the application/service
    pub description: String,
    /// Optional GitHub slug (`<owner>/<repo name>`) for the application/service
    pub github_slug: Option<String>,
    /// Optional subreddit slug (`r/<subreddit>`) for the application/service
    pub subreddit: Option<String>,
}

impl Application {
    /// Create a new [Application] from various string references instead of owned strings
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

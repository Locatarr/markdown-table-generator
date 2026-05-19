//! Create markdown tables for [Locatarr](https://github.com/Locatarr/Locatarr)

#![warn(clippy::pedantic)]

use itertools::Itertools;

use models::{Application, Applications};

pub mod models;

/// Generate a fully valid markdown table from an [Applications] struct.
///
/// The resulting table is the minimum viable table and is not formatted in any way.
pub fn generate_md_table(apps: &Applications) -> String {
    "| **Application** | **Description** | **Github** | **Reddit** |\n|-|-|-|-|\n".to_owned()
        + &apps
            .applications
            .iter()
            .sorted()
            .map(generate_md_row)
            .join("\n")
}

/// Create one markdown table row from a single [Application]
///
/// Copies the [`Application::name`] and [`Application::description`] fields in, and transforms [`Application::github_slug`] and
/// [`Application::subreddit`] into proper markdown links.
fn generate_md_row(app: &Application) -> String {
    let github_link = match &app.github_slug {
        Some(slug) => {
            let slug = slug.replace('|', "\\|");
            format!("[{slug}](https://github.com/{slug})")
        }
        None => String::new(),
    };

    let subreddit_link = match &app.subreddit {
        Some(sub) => {
            let sub = sub.replace('|', "\\|");
            format!("[{sub}](https://reddit.com/{sub})")
        }
        None => String::new(),
    };

    // Output in table format of:
    // | Application | Description | GitHub | Reddit |
    format!(
        "| {} | {} | {} | {} |",
        app.name.replace('|', "\\|"),
        app.description.replace('|', "\\|"),
        github_link,
        subreddit_link
    )
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn test_empty_table() {
        let apps = Applications {
            applications: vec![],
        };

        assert_eq!(
            generate_md_table(&apps),
            "| **Application** | **Description** | **Github** | **Reddit** |\n|-|-|-|-|\n"
        );
    }

    #[test]
    fn test_table() {
        let apps = Applications {
            applications: vec![
                Application::new_from_strs(
                    "Project 1",
                    "Epic description.",
                    Some("example/project1"),
                    None,
                ),
                Application::new_from_strs(
                    "Project 2",
                    "Second epic description.",
                    None,
                    Some("r/project2"),
                ),
                Application::new_from_strs(
                    "Project 3",
                    "Third epic description.",
                    Some("example/project3"),
                    Some("r/project3"),
                ),
            ],
        };

        assert_eq!(
            generate_md_table(&apps),
            r"| **Application** | **Description** | **Github** | **Reddit** |
|-|-|-|-|
| Project 1 | Epic description. | [example/project1](https://github.com/example/project1) |  |
| Project 2 | Second epic description. |  | [r/project2](https://reddit.com/r/project2) |
| Project 3 | Third epic description. | [example/project3](https://github.com/example/project3) | [r/project3](https://reddit.com/r/project3) |"
        );
    }

    #[test]
    fn test_basic_row() {
        let app = Application::new_from_strs("name", "description", None, None);

        assert_eq!(generate_md_row(&app), "| name | description |  |  |");
    }

    #[test]
    fn test_full_row() {
        let app = Application::new_from_strs(
            "Full Apps",
            "Full Description",
            Some("example/test1"),
            Some("r/test1"),
        );

        assert_eq!(
            generate_md_row(&app),
            "| Full Apps | Full Description | [example/test1](https://github.com/example/test1) | [r/test1](https://reddit.com/r/test1) |"
        );
    }

    #[test]
    fn test_pipe_char_in_row() {
        let app = Application::new_from_strs(
            "Full|Apps",
            "Full|Description",
            Some("exa|ple/test1"),
            Some("r/tes|t1"),
        );

        assert_eq!(
            generate_md_row(&app),
            "| Full\\|Apps | Full\\|Description | [exa\\|ple/test1](https://github.com/exa\\|ple/test1) | [r/tes\\|t1](https://reddit.com/r/tes\\|t1) |"
        );
    }
}

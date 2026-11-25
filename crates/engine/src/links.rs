//! Project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The app's slug on GitHub.
pub const APP: &str = "designcraft";
/// DesignCraft's source repository.
pub const GITHUB: &str = "https://github.com/storytold/designcraft";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/storytold/designcraft/issues";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(GITHUB.ends_with(&format!("/storytold/{APP}")));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
        // The community and website links of the upstream build are gone from this fork.
        for k in ["discord", "website", "appPage"] {
            assert!(get(k).is_none() && all().get(k).is_none(), "{k}");
        }
    }
}

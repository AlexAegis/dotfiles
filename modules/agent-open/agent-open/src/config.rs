use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, ensure};
use serde::Deserialize;

/// The owner's choices of what the agent may open, and with which programs.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Pages of `https` URLs.
    pub url_pages: Vec<UrlPages>,
    /// Allows `http` and `https` URLs of `localhost`, `127.0.0.1` and `[::1]`.
    #[serde(default)]
    pub allow_loopback: bool,
    /// Folders whose files and subfolders the editor may open.
    pub editor_roots: Vec<PathBuf>,
    #[serde(default)]
    pub path_views: Vec<PathView>,
    /// Program and leading arguments that open a URL.
    pub browser: Vec<String>,
    /// Program and leading arguments that open a loopback URL, in place of
    /// `browser`.
    #[serde(default)]
    pub loopback_browser: Option<Vec<String>>,
    /// Program and leading arguments that open a path.
    pub editor: Vec<String>,
}

/// The pages of one host. Tokens of a path pattern, each one path segment:
/// - `*`: any segment that is not empty
/// - `{n}`: decimal digits
/// - `{hex}`: hexadecimal digits
/// - `**`, only last: the rest of the path, also none
/// - other: the same text
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlPages {
    pub host: String,
    pub paths: Vec<String>,
}

/// A folder of the agent that shows a folder of the owner, such as a read-only
/// bind mount. Paths under `agent` open as the same paths under `owner`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathView {
    pub agent: PathBuf,
    pub owner: PathBuf,
}

impl Config {
    /// Reads the file. A leading `~` in a path means `home`.
    pub fn load(path: &Path, home: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("Config {} unreadable", path.display()))?;
        Self::parse(&text, home).with_context(|| format!("Config {} invalid", path.display()))
    }

    pub fn parse(text: &str, home: &Path) -> Result<Self> {
        let mut config: Self = toml::from_str(text)?;
        ensure!(
            !config.browser.is_empty()
                && !config.editor.is_empty()
                && config
                    .loopback_browser
                    .as_ref()
                    .is_none_or(|b| !b.is_empty()),
            "browser, loopback_browser and editor need a program"
        );
        for pages in &mut config.url_pages {
            ensure!(
                !pages.host.is_empty()
                    && !pages.host.starts_with('.')
                    && !pages.host.ends_with('.'),
                "url_pages host {:?} is not a host name",
                pages.host
            );
            pages.host.make_ascii_lowercase();
            for pattern in &pages.paths {
                check_path_pattern(pattern)?;
            }
        }
        for root in &mut config.editor_roots {
            *root = expand_home(root, home);
        }
        for view in &mut config.path_views {
            view.agent = expand_home(&view.agent, home);
            view.owner = expand_home(&view.owner, home);
        }
        Ok(config)
    }
}

fn check_path_pattern(pattern: &str) -> Result<()> {
    let segments = pattern
        .strip_prefix('/')
        .with_context(|| format!("Path pattern {pattern:?} does not start with /"))?
        .split('/')
        .collect::<Vec<_>>();
    let inner = &segments[..segments.len() - 1];
    ensure!(
        !inner.contains(&"**"),
        "Path pattern {pattern:?} has ** before its last segment"
    );
    let unknown = segments
        .iter()
        .find(|segment| segment.contains(['{', '}']) && !matches!(**segment, "{n}" | "{hex}"));
    ensure!(
        unknown.is_none(),
        "Path pattern {pattern:?} has an unknown token {:?}",
        unknown.unwrap_or(&"")
    );
    Ok(())
}

fn expand_home(path: &Path, home: &Path) -> PathBuf {
    match path.strip_prefix("~") {
        Ok(rest) => home.join(rest),
        Err(_) => path.to_path_buf(),
    }
}

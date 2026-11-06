use std::{
    fs,
    net::{Ipv4Addr, Ipv6Addr},
    path::{Component, Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail, ensure};
use url::{Host, Url};

use crate::config::Config;

/// Longest request line, the newline included.
pub const MAX_REQUEST_BYTES: usize = 4096;
const MAX_URL_BYTES: usize = 2048;
/// The reply to every refused path, so that a reply shows nothing about the
/// files of the owner. The journal gets the reason.
pub const PATH_REFUSAL: &str = "Path not allowed";

/// A request that passed the checks of the config.
#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    /// Opens the URL in the browser.
    Url(Url),
    /// Opens the URL of `localhost`, `127.0.0.1` or `[::1]` in the loopback
    /// browser.
    LoopbackUrl(Url),
    /// Opens the canonical path in the editor.
    Edit(PathBuf),
}

/// Parses one request line without its newline: `url <URL>` or `edit <absolute path>`.
pub fn check(line: &str, config: &Config) -> Result<Request> {
    ensure!(
        !line.chars().any(char::is_control),
        "Request contains a control character"
    );
    let (kind, argument) = line
        .split_once(' ')
        .context("Request needs a kind and an argument")?;
    match kind {
        "url" => check_url(argument, config),
        "edit" => resolve_editor_path(argument, config)
            .map(Request::Edit)
            .map_err(|reason| reason.context(PATH_REFUSAL)),
        _ => bail!("Unknown request kind {kind:?}"),
    }
}

pub fn check_url(text: &str, config: &Config) -> Result<Request> {
    ensure!(
        text.len() <= MAX_URL_BYTES,
        "URL longer than {MAX_URL_BYTES} bytes"
    );
    let url = Url::parse(text).context("URL invalid")?;
    ensure!(
        url.username().is_empty() && url.password().is_none(),
        "URL contains a user name or a password"
    );
    let host = url.host().context("URL has no host")?;
    let scheme = url.scheme();
    ensure!(
        matches!(scheme, "http" | "https"),
        "URL scheme {scheme:?} not allowed"
    );
    if config.allow_loopback && is_loopback(&host) {
        return Ok(Request::LoopbackUrl(url));
    }
    ensure!(
        scheme == "https" && url.port().is_none() && is_allowed_page(&url, config),
        "URL page not allowed"
    );
    Ok(Request::Url(url))
}

fn is_loopback(host: &Host<&str>) -> bool {
    match host {
        Host::Domain(domain) => *domain == "localhost",
        Host::Ipv4(address) => *address == Ipv4Addr::LOCALHOST,
        Host::Ipv6(address) => *address == Ipv6Addr::LOCALHOST,
    }
}

fn is_allowed_page(url: &Url, config: &Config) -> bool {
    let Some(Host::Domain(domain)) = url.host() else {
        return false;
    };
    let mut segments: Vec<&str> = url.path().trim_start_matches('/').split('/').collect();
    if segments.len() > 1 && segments.last() == Some(&"") {
        segments.pop();
    }
    config
        .url_pages
        .iter()
        .filter(|pages| pages.host == domain)
        .flat_map(|pages| &pages.paths)
        .any(|pattern| {
            let pattern: Vec<&str> = pattern.trim_start_matches('/').split('/').collect();
            matches_segments(&pattern, &segments)
        })
}

fn matches_segments(pattern: &[&str], segments: &[&str]) -> bool {
    match (pattern.split_first(), segments.split_first()) {
        (None, None) | (Some((&"**", _)), _) => true,
        (Some((token, pattern)), Some((segment, segments))) => {
            matches_segment(token, segment) && matches_segments(pattern, segments)
        }
        _ => false,
    }
}

fn matches_segment(token: &str, segment: &str) -> bool {
    let is_made_of =
        |digit: fn(&u8) -> bool| !segment.is_empty() && segment.bytes().all(|b| digit(&b));
    match token {
        "*" => !segment.is_empty(),
        "{n}" => is_made_of(u8::is_ascii_digit),
        "{hex}" => is_made_of(u8::is_ascii_hexdigit),
        literal => literal == segment,
    }
}

/// Maps a path of a path view to the owner's path. Checks it against the
/// editor roots before it touches the file system, then again after it
/// resolves symlinks. The result is an existing file or folder, other than a
/// VS Code workspace file, whose folders can be anywhere.
pub fn resolve_editor_path(text: &str, config: &Config) -> Result<PathBuf> {
    let requested = Path::new(text);
    ensure!(requested.is_absolute(), "Path {text:?} not absolute");
    ensure!(
        !requested.components().any(|c| c == Component::ParentDir),
        "Path {text:?} contains .."
    );
    let path = owner_path(requested, config);
    ensure!(
        config
            .editor_roots
            .iter()
            .any(|root| path.starts_with(root)),
        "Path {} outside the editor roots",
        path.display()
    );
    let canonical =
        fs::canonicalize(&path).with_context(|| format!("Path {} unresolvable", path.display()))?;
    let in_root = config
        .editor_roots
        .iter()
        .filter_map(|root| fs::canonicalize(root).ok())
        .any(|root| canonical.starts_with(root));
    ensure!(
        in_root,
        "Path {} resolves outside the editor roots, to {}",
        path.display(),
        canonical.display()
    );
    let is_workspace = canonical
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("code-workspace"));
    ensure!(
        !is_workspace,
        "Path {} is a workspace file",
        canonical.display()
    );
    let metadata = fs::metadata(&canonical)
        .map_err(|error| anyhow!("Path {} unreadable: {error}", canonical.display()))?;
    ensure!(
        metadata.is_dir() || metadata.is_file(),
        "Path {} neither a file nor a folder",
        canonical.display()
    );
    Ok(canonical)
}

fn owner_path(path: &Path, config: &Config) -> PathBuf {
    config
        .path_views
        .iter()
        .find_map(|view| {
            path.strip_prefix(&view.agent)
                .ok()
                .map(|rest| view.owner.join(rest))
        })
        .unwrap_or_else(|| path.to_path_buf())
}

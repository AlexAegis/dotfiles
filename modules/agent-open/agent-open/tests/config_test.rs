use std::path::{Path, PathBuf};

use agent_open::{config::Config, request};

const GITHUB_PAGES: &str = r#"[{ host = "github.com", paths = ["/*/*/pull/{n}/**"] }]"#;

fn parse(url_pages: &str, browser: &str) -> anyhow::Result<Config> {
    let text = format!(
        r#"
url_pages = {url_pages}
editor_roots = ["~/git", "/srv/git", "~other/git"]
path_views = [{{ agent = "/srv/alex/git", owner = "~/git" }}]
browser = {browser}
editor = ["code"]
"#
    );
    Config::parse(&text, Path::new("/home/alex"))
}

#[test]
fn should_expand_a_leading_tilde_to_the_home_folder() {
    let config = parse(GITHUB_PAGES, r#"["xdg-open"]"#).unwrap();
    assert_eq!(
        config.editor_roots,
        [
            PathBuf::from("/home/alex/git"),
            PathBuf::from("/srv/git"),
            PathBuf::from("~other/git"),
        ]
    );
    assert_eq!(config.path_views[0].owner, Path::new("/home/alex/git"));
    assert!(!config.allow_loopback);
}

#[test]
fn should_refuse_url_pages_with_a_bad_host_or_path_pattern() {
    for url_pages in [
        r#"[{ host = "", paths = [] }]"#,
        r#"[{ host = ".github.com", paths = [] }]"#,
        r#"[{ host = "github.com.", paths = [] }]"#,
        r#"[{ host = "github.com", paths = ["*/pull"] }]"#,
        r#"[{ host = "github.com", paths = ["/**/pull"] }]"#,
        r#"[{ host = "github.com", paths = ["/*/*/pull/{id}"] }]"#,
    ] {
        assert!(
            parse(url_pages, r#"["xdg-open"]"#).is_err(),
            "{url_pages} was not refused"
        );
    }
}

#[test]
fn should_refuse_an_empty_program_or_unknown_keys() {
    assert!(parse(GITHUB_PAGES, "[]").is_err());
    let empty_loopback_browser = r#"
url_pages = []
editor_roots = []
browser = ["xdg-open"]
loopback_browser = []
editor = ["code"]
"#;
    assert!(Config::parse(empty_loopback_browser, Path::new("/home/alex")).is_err());
    assert!(Config::parse("url_pages = []\nunknown = 1", Path::new("/home/alex")).is_err());
}

#[test]
fn should_parse_the_config_of_the_module_with_the_owners_pages() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../XDG_CONFIG_HOME.agent-open/agent-open/config.toml");
    let config = Config::load(&path, Path::new("/home/alex")).unwrap();
    for (url, allowed) in [
        ("https://github.com/AlexAegis/pont/pull/20", true),
        ("https://github.com/AlexAegis/pont/pull/20/files", true),
        ("https://github.com/AlexAegis/pont/pulls", true),
        ("https://github.com/AlexAegis/pont/issues", true),
        ("https://github.com/AlexAegis/pont/issues/", true),
        ("https://github.com/AlexAegis/pont/issues/3", true),
        (
            "https://github.com/AlexAegis/pont/actions/runs/42/job/7",
            true,
        ),
        ("https://gist.github.com/AlexAegis/0123abcd", true),
        ("https://gist.github.com/AlexAegis/0123abcd/", true),
        ("https://gist.github.com/0123abcd", true),
        ("http://localhost:5173/", true),
        ("https://github.com/settings/tokens", false),
        ("https://github.com/login/device", false),
        ("https://github.com/AlexAegis/pont/settings", false),
        ("https://github.com/AlexAegis/pont/invitations", false),
        (
            "https://github.com/AlexAegis/pont/pull/new/main...evil:main",
            false,
        ),
        (
            "https://github.com/AlexAegis/pont/issues/new?title=x&body=y",
            false,
        ),
        ("https://github.com:8443/AlexAegis/pont/pull/20", false),
        ("https://github.com/AlexAegis/pont/pull//", false),
        ("https://gist.github.com/auth/github", false),
        ("https://gist.github.com/mine", false),
        ("https://claude.ai/code/artifact/x", false),
    ] {
        assert_eq!(request::check_url(url, &config).is_ok(), allowed, "{url}");
    }
    assert_eq!(
        config.editor_roots,
        [Path::new("/home/alex/git"), Path::new("/home/agent/git")]
    );
    assert!(config.loopback_browser.is_some());
}

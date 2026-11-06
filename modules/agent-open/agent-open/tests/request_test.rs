use std::{fs, os::unix::fs::symlink, path::Path, process::Command};

use agent_open::{
    config::Config,
    launch,
    request::{self, PATH_REFUSAL, Request},
};
use tempfile::TempDir;
use url::Url;

struct Fixture {
    folder: TempDir,
    config: Config,
}

impl Fixture {
    fn new(allow_loopback: bool) -> Self {
        let folder = TempDir::new().unwrap();
        let base = folder.path();
        for path in ["home/git/repo", "view", "outside"] {
            fs::create_dir_all(base.join(path)).unwrap();
        }
        fs::write(base.join("home/git/repo/main.rs"), "").unwrap();
        fs::write(base.join("outside/secret"), "").unwrap();
        let text = format!(
            r#"
url_pages = [
    {{ host = "GitHub.com", paths = ["/*/*/pull/{{n}}/**", "/*/*/issues/**"] }},
    {{ host = "gist.github.com", paths = ["/*/*"] }},
]
allow_loopback = {allow_loopback}
editor_roots = ["~/git"]
path_views = [{{ agent = "{view}", owner = "~/git" }}]
browser = ["xdg-open"]
loopback_browser = ["brave", "--user-data-dir=/tmp/profile"]
editor = ["code", "--reuse-window"]
"#,
            view = base.join("view").display(),
        );
        let config = Config::parse(&text, &base.join("home")).unwrap();
        Self { folder, config }
    }

    fn path(&self, relative: &str) -> String {
        self.folder.path().join(relative).display().to_string()
    }

    fn check(&self, line: &str) -> anyhow::Result<Request> {
        request::check(line, &self.config)
    }
}

fn assert_url_refused(fixture: &Fixture, url: &str) {
    assert!(
        fixture.check(&format!("url {url}")).is_err(),
        "{url} was not refused"
    );
}

#[test]
fn should_allow_https_urls_that_match_a_page_pattern() {
    let fixture = Fixture::new(false);
    for url in [
        "https://github.com/AlexAegis/dotfiles/pull/1",
        "https://github.com/AlexAegis/dotfiles/pull/1/",
        "https://github.com/AlexAegis/dotfiles/pull/1/files?w=1#diff",
        "https://GITHUB.COM/AlexAegis/dotfiles/issues",
        "https://github.com/AlexAegis/dotfiles/issues/7",
        "https://gist.github.com/AlexAegis/0123abcd",
    ] {
        let request = fixture.check(&format!("url {url}")).unwrap();
        assert_eq!(request, Request::Url(Url::parse(url).unwrap()));
    }
}

#[test]
fn should_refuse_urls_outside_the_page_patterns() {
    let fixture = Fixture::new(false);
    for url in [
        "https://github.com/",
        "https://github.com/settings/tokens",
        "https://github.com/login/oauth/authorize?client_id=x",
        "https://github.com/AlexAegis/dotfiles",
        "https://github.com/AlexAegis/dotfiles/pull",
        "https://github.com/AlexAegis/dotfiles/pulls",
        "https://github.com/AlexAegis//pull/1",
        "https://github.com/AlexAegis/dotfiles/pull/1/../../settings",
        "https://github.com/AlexAegis/dotfiles/pull/1/%2e%2e/%2e%2e/settings",
        "https://docs.github.com/AlexAegis/dotfiles/pull/1",
        "https://gist.github.com/AlexAegis/0123abcd/edit",
        "http://github.com/AlexAegis/dotfiles/pull/1",
        "https://evilgithub.com/AlexAegis/dotfiles/pull/1",
        "https://github.com.evil.com/AlexAegis/dotfiles/pull/1",
        "https://github.com@evil.com/AlexAegis/dotfiles/pull/1",
        "https://user:password@github.com/AlexAegis/dotfiles/pull/1",
        "https://140.82.121.4/AlexAegis/dotfiles/pull/1",
        "file:///home/alex/.ssh/id_ed25519",
        "javascript:alert(1)",
        "vscode://file/etc/passwd",
        "https://localhost:3000/",
        "github.com/AlexAegis/dotfiles/pull/1",
    ] {
        assert_url_refused(&fixture, url);
    }
}

#[test]
fn should_allow_loopback_urls_only_when_allowed() {
    let allowing = Fixture::new(true);
    for url in [
        "http://localhost:5173/",
        "https://localhost/",
        "http://127.0.0.1:8080/x",
        "http://127.1/",
        "http://[::1]:3000/",
    ] {
        let request = allowing.check(&format!("url {url}")).unwrap();
        assert_eq!(request, Request::LoopbackUrl(Url::parse(url).unwrap()));
        assert_url_refused(&Fixture::new(false), url);
    }
    for url in [
        "ftp://localhost/",
        "http://127.0.0.2/",
        "http://localhost./",
        "http://[::ffff:127.0.0.1]/",
        "http://0.0.0.0/",
    ] {
        assert_url_refused(&allowing, url);
    }
}

#[test]
fn should_refuse_a_url_longer_than_2048_bytes() {
    let fixture = Fixture::new(false);
    let url = format!(
        "https://github.com/AlexAegis/dotfiles/pull/1/{}",
        "a".repeat(2048)
    );
    assert_url_refused(&fixture, &url);
}

#[test]
fn should_resolve_files_and_folders_under_an_editor_root() {
    let fixture = Fixture::new(false);
    for relative in ["home/git/repo", "home/git/repo/main.rs", "home/git/./repo"] {
        let request = fixture
            .check(&format!("edit {}", fixture.path(relative)))
            .unwrap();
        let expected = fs::canonicalize(fixture.path(relative)).unwrap();
        assert_eq!(request, Request::Edit(expected));
    }
}

#[test]
fn should_map_a_path_view_to_the_owner_path() {
    let fixture = Fixture::new(false);
    let request = fixture
        .check(&format!("edit {}", fixture.path("view/repo")))
        .unwrap();
    let expected = fs::canonicalize(fixture.path("home/git/repo")).unwrap();
    assert_eq!(request, Request::Edit(expected));
}

#[test]
fn should_refuse_paths_outside_the_editor_roots_with_one_reply() {
    let fixture = Fixture::new(false);
    symlink(fixture.path("outside"), fixture.path("home/git/link")).unwrap();
    fs::write(fixture.path("home/git/repo/w.code-workspace"), "{}").unwrap();
    fs::write(fixture.path("home/git/repo/W.CODE-WORKSPACE"), "{}").unwrap();
    symlink(
        fixture.path("home/git/repo/w.code-workspace"),
        fixture.path("home/git/repo/plain"),
    )
    .unwrap();
    for path in [
        fixture.path("outside/secret"),
        fixture.path("outside/missing"),
        fixture.path("home/git/repo/../repo"),
        fixture.path("home/git/../../outside/secret"),
        fixture.path("home/git/link/secret"),
        fixture.path("home/git/link/missing"),
        fixture.path("view/../outside/secret"),
        fixture.path("home/git/missing"),
        fixture.path("home/git/repo/w.code-workspace"),
        fixture.path("home/git/repo/W.CODE-WORKSPACE"),
        fixture.path("home/git/repo/plain"),
        "home/git/repo".to_owned(),
    ] {
        let error = fixture.check(&format!("edit {path}")).unwrap_err();
        assert_eq!(error.to_string(), PATH_REFUSAL, "{path}");
    }
}

#[test]
fn should_refuse_a_path_outside_the_roots_before_it_reads_the_file_system() {
    let fixture = Fixture::new(false);
    let error = fixture
        .check(&format!("edit {}", fixture.path("outside/missing")))
        .unwrap_err();
    let reason = format!("{error:#}");
    assert!(reason.contains("outside the editor roots"), "{reason}");
    assert!(!reason.contains("unresolvable"), "{reason}");
}

#[test]
fn should_refuse_a_path_that_is_neither_a_file_nor_a_folder() {
    let fixture = Fixture::new(false);
    let fifo = fixture.path("home/git/fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    assert!(fixture.check(&format!("edit {fifo}")).is_err());
}

#[test]
fn should_refuse_malformed_request_lines() {
    let fixture = Fixture::new(false);
    for line in [
        "",
        "url",
        "open https://github.com/a/b/pull/1",
        "URL https://github.com/a/b/pull/1",
        "url https://github.com/a/b/pull/1\r",
        "url https://github.com/a/b/pull/1\tx",
        "edit /tmp\u{1b}[2J",
    ] {
        assert!(fixture.check(line).is_err(), "{line:?} was not refused");
    }
}

#[test]
fn should_open_a_url_with_systemd_run_and_the_browser() {
    let fixture = Fixture::new(false);
    let request = fixture
        .check("url https://github.com/a/b/pull/1?q=c d")
        .unwrap();
    let command_line = launch::command_line(&request, &fixture.config);
    assert_eq!(
        command_line,
        [
            "systemd-run",
            "--user",
            "--quiet",
            "--collect",
            "--property=ExitType=cgroup",
            "--description=agent-open: URL",
            "--",
            "xdg-open",
            "https://github.com/a/b/pull/1?q=c%20d",
        ]
    );
}

#[test]
fn should_open_a_loopback_url_with_the_loopback_browser() {
    let fixture = Fixture::new(true);
    let request = fixture.check("url http://localhost:5173/").unwrap();
    let command_line = launch::command_line(&request, &fixture.config);
    assert_eq!(
        command_line[5..],
        [
            "--description=agent-open: loopback URL",
            "--",
            "brave",
            "--user-data-dir=/tmp/profile",
            "http://localhost:5173/",
        ]
    );
}

#[test]
fn should_open_a_loopback_url_with_the_browser_without_a_loopback_browser() {
    let mut fixture = Fixture::new(true);
    fixture.config.loopback_browser = None;
    let request = fixture.check("url http://localhost:5173/").unwrap();
    let command_line = launch::command_line(&request, &fixture.config);
    assert_eq!(command_line[7..], ["xdg-open", "http://localhost:5173/"]);
}

#[test]
fn should_open_a_path_with_systemd_run_and_the_editor() {
    let fixture = Fixture::new(false);
    let path = fs::canonicalize(fixture.path("home/git/repo")).unwrap();
    let command_line = launch::command_line(&Request::Edit(path.clone()), &fixture.config);
    let tail: Vec<&Path> = command_line[6..].iter().map(Path::new).collect();
    assert_eq!(command_line[5], "--description=agent-open: path");
    assert_eq!(
        tail,
        [
            Path::new("--"),
            Path::new("code"),
            Path::new("--reuse-window"),
            &path
        ]
    );
}

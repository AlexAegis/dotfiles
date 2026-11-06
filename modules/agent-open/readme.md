# Agent Open

Lets the `agent` user open a URL in the owner's browser or a path in the
owner's editor, within the limits of
`XDG_CONFIG_HOME.agent-open/agent-open/config.toml`.

```text
agent client -> /run/agent-open/open.sock -> agent-open@.service -> systemd-run --user -> browser | loopback_browser | editor
                (group agent)                (owner, sandboxed)     (owner's user manager)
```

- systemd starts one `agent-open` process for each connection to the socket, at
  most 4 at a time. It reads one request line, checks it against the config and
  replies with one line.
- The process sees all home folders read-only, no network, and of `/run` only
  the private socket of the user manager. Its memory, task and time limits are
  in `agent-open@.service`.
- For an allowed request, it asks the user manager to start the program as a
  transient service with `systemd-run --user`. The program gets the environment
  of the desktop session, not the sandbox of the process.
  `--property=ExitType=cgroup` keeps the program alive after a launcher, such as
  `xdg-open`, exits.
- The config is read at each request, so a change applies to the next request.

## Requests

One line each, at most 4096 bytes with the newline:

- `url <URL>`: opens the URL with `browser`. The scheme is `https`, with the
  default port. The host and the path match an entry of `url_pages`. The config
  of the module allows, on GitHub, the pull request and issue lists of a
  repository and its pull requests, issues and Actions runs by number. It also
  allows gists. With `allow_loopback`, the scheme can also be `http`, with the
  host `localhost`, `127.0.0.1` or `[::1]`. Such a URL opens with
  `loopback_browser` when it is set. The config of the module sets it to a
  Brave profile without the owner's logins. The URL has no user name or
  password, and is at most 2048 bytes long.
- `edit <path>`: opens the path with `editor`. The path is absolute, without
  `..`. A path under the `agent` folder of a `path_views` entry opens the same
  path under its `owner` folder. The result is under one of `editor_roots`, also
  after symlinks are resolved. It is an existing file or folder, and not a VS
  Code workspace file, whose folders can be anywhere.

The reply is `ok`, or `error: <reason>`. A refused path gets the same reason,
`Path not allowed`, whatever the cause, so that replies show nothing about the
files of the owner. The journal of `agent-open@.service` has one line with the
whole reason for each request:

```sh
journalctl --user -u 'agent-open@*'
```

The program gets the checked URL or the resolved path as its last argument. The
request adds no other argument.

## Install

`pont -fx agent-open` builds `agent-open` with the locked dependencies and
installs it to `$XDG_DATA_HOME/agent-open`. Its root step creates
`/run/agent-open` through `/etc/tmpfiles.d/agent-open.conf`. The module applies
only to writable dotfiles on a systemd machine with KWin, where the `agent`
group exists. It does not install the browser or the editor.

The `agent-open` module of
[agent-dotfiles](https://github.com/AlexAegis/agent-dotfiles) installs the
client of `agent`, and makes it the handler of `http` and `https` links.

## Limits

- A program opens with the access of the owner. A URL of `url_pages` opens in
  the owner's browser, with the owner's logins. A GitHub pull request or Actions
  run page can still show an approval, such as "Approve and run workflows" or
  "Review deployments".
- `agent` can listen on a loopback port. A loopback URL can then show a page of
  `agent` that leads anywhere, and that shares the cookies of the other pages
  in its browser profile. Without `loopback_browser`, it opens in the owner's
  browser with the owner's logins.
- In an editor root that `agent` can write, `agent` can change the files
  between the check and the start of the editor, such as by replacing a folder
  with a symlink. The editor can then open another path that the owner can
  read.
- The config of the module allows no folder of `agent`, such as
  `/home/agent/git`. The owner's Git refuses a repository there, because
  `agent` owns it. VS Code then offers to add it to `safe.directory`. After
  that, the owner's Git runs the commands that `agent` writes into the
  configuration of that repository, such as `core.fsmonitor`. If you add such
  a folder, keep its repositories out of `safe.directory`. Also keep the folder
  untrusted in the editor's workspace trust.
- The socket accepts 4 requests at a time, with no limit per minute.
- A request longer than 4096 bytes gets an error reply, then the connection
  resets.

## Check

```sh
systemctl --user status agent-open.socket
printf 'url https://github.com/AlexAegis/dotfiles/pulls\n' | socat - UNIX-CONNECT:/run/agent-open/open.sock
```

The second command opens the pull requests of this repository and prints `ok`,
for the owner and for `agent`. `cargo test` in `agent-open` tests the checks of
the requests and whole connections with a fake launcher.

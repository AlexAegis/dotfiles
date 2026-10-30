# Agent Wayland

Lets programs of the `agent` user open windows on the owner's KDE Plasma
desktop, without the clipboard, the primary selection, or the other globals
that the proxy hides.

```text
agent client -> /run/agent-wayland/wayland-0 -> agent-wayland-proxy -> KWin security context
                (group agent)                    (owner, sandboxed)     (agent-wayland-context)
```

- `agent-wayland-context` connects to KWin and creates a security context. KWin
  hides its privileged globals, such as clipboard managers, screencast, and fake
  input, from every client of that context. It still offers them the clipboard
  and layer shell.
- `agent-wayland-proxy` serves one client of `agent`. systemd starts one for
  each connection to the socket, at most 32 at a time. It sees no home folder,
  no network, and of `/run` only the socket of the security context.
- Each proxy has its own memory, task, and open file limits, and no swap.
  `app-agent_wayland.slice` limits the memory of all of them. A client that
  makes its proxy buffer too much ends only its own connection.
- The proxy forwards only the globals for windows, buffers, input, and frame
  timing, listed in `agent-wayland-proxy/src/allowed_globals.rs`. The clipboard,
  the primary selection, drag and drop, layer shell, input methods, idle state,
  and output control stay hidden.
- It gives forwarded globals new names, so a client cannot address a hidden
  one. A client that binds a name it was not offered loses its connection.
- It prefixes window titles with `[agent]` and app ids with `agent.`. The KWin
  rule of `4.user.kwriteconfig6.sh` matches those app ids. It forces
  server-side decorations, the emerald `AgentWindow` title bar, and the extreme
  level of focus stealing prevention. The title bar has the color of the `agent`
  status line in Claude Code.

The `wayland` module of
[agent-dotfiles](https://github.com/AlexAegis/agent-dotfiles) sets
`WAYLAND_DISPLAY=/run/agent-wayland/wayland-0` for `agent`.

## Install

`pont -fx agent-wayland` builds both programs with the locked dependencies and
installs them to `$XDG_DATA_HOME/agent-wayland`. Its root step creates
`/run/agent-wayland` through `/etc/tmpfiles.d/agent-wayland.conf`. The module
applies only to writable dotfiles on a systemd machine with KWin, where the
`agent` group exists.

## Limits

- A fullscreen window has no title bar, so only the taskbar marks it.
- GTK 4 programs, such as Ghostty, refuse a display without
  `wl_data_device_manager`. They fall back to X11, which `agent` cannot reach.
- A client keeps keyboard input while it has focus, as on any desktop.
- A sandbox that hides `/run/agent-wayland` or the GPU, such as that of Claude
  Code, stops a program from opening a window. Run such a program outside the
  sandbox.
- Many clients that each stay below the limit of their proxy can together reach
  the limit of the slice. The kernel then stops the largest proxy in the slice.

## Check

```sh
systemctl --user status agent-wayland-proxy.socket
WAYLAND_DISPLAY=/run/agent-wayland/wayland-0 wayland-info | grep interface:
```

The second command lists only the allowed globals, for the owner and for
`agent`. `cargo test` in `agent-wayland-proxy` runs test clients through the
connection code to a fake compositor.

# Approval Queue

One list for the permission prompts of coding agent sessions. A hook of the
harness sends each prompt to the queue. The owner answers it in the list or in
the terminal of its session. The first answer applies. A prompt that its
terminal shows only after the hook ends is different: see `list_only` in
[Requests](#requests).

```text
hook of a session -> /run/approval-queue/request.sock -> approval-queue@.service -> $XDG_RUNTIME_DIR/approval-queue <- approval-queue
                     (group agent)                       (owner, sandboxed)         (entries, answers)                 (the list)
```

- systemd starts one `approval-queue-serve` process for each connection to the
  socket, at most 64 at a time and 48 for one user. The process reads one
  request line, stores it as an entry and waits.
- The list shows the entries. An answer becomes an answer file. The process of
  that entry replies with it and ends.
- When the hook closes its connection, the entry leaves the queue. The prompt
  got its answer in the terminal, or its session ended.
- Of the home folders, the process sees only the folder of its binary. Of
  `/run`, it sees only the folder of the queue. It has no network. Its memory,
  task and time limits are in `approval-queue@.service`.

## List

`approval-queue` opens the list in a terminal. The upper part has one row for
each waiting prompt, the oldest first: how long it waits, user, start of the
session ID, tool, folder, command or file of the call. The lower part shows the
selected prompt in full: user, harness, permission mode, session, folder, the
reason of a guard hook that asked, the tool and its input. Its border counts the
lines above and below it.

| Key              | Effect                                          |
| ---------------- | ----------------------------------------------- |
| `a`              | allows the selected prompt                      |
| `d`              | denies it                                       |
| `m`              | denies it with a message, which the agent reads |
| `j`, `k`, arrows | select                                          |
| PgUp, PgDn       | scroll the selected prompt                      |
| `s`              | mutes the pings for 10 minutes                  |
| `S`              | mutes the pings for 1 hour                      |
| `u`              | ends the mute                                   |
| `q`, Ctrl-C      | quit                                            |

- An answer names its entry by ID. The selection stays on its entry while other
  entries come and go.
- When the selected entry leaves, the oldest entry gets the selection.
- The list takes `a`, `d`, `m`, `s`, `S`, `u` and `q` only after 1 second
  without a key press, pasted text, a new selection by the list or a new focus
  of its window. A key that the list does not use counts as a key press too. A
  key meant for the entry that left then does not answer the next one. Of text
  typed into the list without a pause of 1 second, only the first key can act.
  An `a` or `d` there answers one prompt. An `s` or `S` there mutes the pings.
  An `m` there opens a message with the rest of the text, which Enter sends as
  a denial.
- A ping gives the window of the list the focus. Keys that the owner types for
  another window in the second after that answer no prompt, open no message,
  change no mute and do not quit with `q`. An open message then takes a key or
  pasted text only after 1 second without either. The keys that select and
  scroll, and Ctrl-C, act at once. A terminal that does not report the focus
  gives no such second.
- A key without a function gets a notice with its name, such as
  `Key Alt+a has no function`.
- With Caps Lock on, `s` arrives as `S` and mutes for 1 hour.
- After `m`, each key is text of the message, also when the prompt leaves the
  queue in that time. Esc closes the message.
- A key with Ctrl or Alt is not its plain key: Ctrl-D is no `d`. Ctrl-C quits.
- Pasted text is text of an open message and no key. A terminal without
  bracketed paste sends pasted text as keys.
- `?` before a user name: the service could not verify the name. It verifies
  only the name of the owner.
- Control characters of a request show as escapes, such as `\u{1b}`. So do the
  blank and invisible characters that `display_text.rs` lists, such as the
  direction overrides. Unicode has more such characters than that list.
- Several lists can run at the same time. The queue takes the first answer to a
  prompt and refuses a later one.
- After the last list quits, the prompts stay in the queue, except those with
  `list_only`.

## Notify

`approval-queue-notify.service` runs `approval-queue-notify` in the graphical
session. It checks the entry files 2 times each second and brings the list to
view at a cadence, so that many prompts give few pings. Each prompt counts
for at most one ping. Until then it waits for its ping:

- A prompt pings at once when no prompt waits for a ping and the last ping is
  at least 30 seconds old.
- After a ping, no ping comes for 30 seconds.
- After that time, a ping comes when 4 prompts wait for a ping, or, with
  fewer, when the oldest of them has waited 2 minutes.
- A prompt that leaves the queue before its ping counts for none.

The cadence comes from the
[`ping_throttle`](https://github.com/AlexAegis/ping_throttle) crate. Its values
are the `CADENCE` in `ping_watch.rs`. A restart of the service starts the
cadence again: each waiting prompt then counts as new.

A ping does this:

1. It loads `focus_list.js` into KWin through D-Bus (`busctl`). The script
   moves the window of the list to the current desktop and gives it the focus.
   It finds the window by its caption, which must be `approval-queue`: the
   list sets the title of its terminal to that, and clears it when it quits.
   The script stays loaded for 10 seconds, so it also focuses one list window
   that opens in that time. A window on all desktops stays on all desktops.
2. When no list is open, it starts `ghostty` with the list, as a unit of its
   own (`systemd-run --user`).

The script finds no list whose window shows a caption other than that title:

- a list in a tab that is not the current tab of its window
- a list in a terminal multiplexer, or in a terminal with a fixed title format

A ping then shows nothing while that list is open. The script does not move a
window between activities. A ping also opens a list while nobody watches the
screen.

The cadence counts the time that the machine runs, without the time of a
suspend.

`s` and `S` in the list mute the pings. The upper border of the list shows the
minutes left. A later `s` or `S` sets a new time. While the mute lasts, no ping
comes and no list opens. The prompts keep their wait: when the mute ends, one
ping comes for those that still wait for theirs. A mute ends at a clock time:
the time of a suspend counts. It also lasts after the list quits.

On a desktop without KWin, step 1 fails and the journal has the reason. Step 2
still runs:

```sh
journalctl --user -u approval-queue-notify.service
```

## Requests

One JSON line, at most 65536 bytes with the newline. Each field is required. A
text with a limit is not empty:

| Field       | Content                                                           | Limit      |
| ----------- | ----------------------------------------------------------------- | ---------- |
| `version`   | `1`                                                               |            |
| `host`      | the harness, such as `claude-code`                                | 32 bytes   |
| `user`      | the user that the hook runs as                                    | 64 bytes   |
| `session`   | the ID of the session                                             | 128 bytes  |
| `folder`    | the working folder of the session                                 | 4096 bytes |
| `mode`      | the permission mode of the session                                | 32 bytes   |
| `tool`      | the name of the tool                                              | 256 bytes  |
| `input`     | the input of the tool call, an object                             |            |
| `reason`    | why a guard hook asked, or `null`                                 |            |
| `list_only` | `true` or `false`: only the list can answer the prompt, see below |            |

The hook keeps the connection open and sends nothing after its line. It ends its
wait after 1 day. The reply is one JSON line:

- `{"behavior":"allow"}`
- `{"behavior":"deny"}`, with a `message` when the owner typed one
- `{"error":"<reason>"}` for a refused request and for a `list_only` prompt
  while no list is open. The journal of `approval-queue@.service` has the whole
  reason:

  ```sh
  journalctl --user -u 'approval-queue@*'
  ```

The service sends no reply when the hook closes first or when nobody answers in
25 hours.

`list_only` is `true` for a prompt that its session shows in the terminal only
after the hook ends. While such a prompt waits in the queue, only the list can
answer it. The service keeps it only while a list is open. After 3 seconds
without an open list, the service replies `{"error":"No list is open"}`, so the
hook ends and the terminal shows the prompt.

`user` has the form of an account name: lowercase letters, digits, `_` and `-`.
The service compares it with the sender of the connection. It refuses the name
of the owner from another user. It also refuses another name from the owner. So
a request of another user does not show as a request of the owner. The service
cannot tell two other users apart: for them, the list shows the name that the
request states.

## Files

`$XDG_RUNTIME_DIR/approval-queue`, for the owner only:

- `entries/<id>.json`: one waiting prompt. Its service process keeps a lock on
  the file. The list and `approval-queue-notify` remove an entry file that no
  process locks.
- `answers/<id>.json`: the answer for that entry, `{"behavior":"allow"}` or
  `{"behavior":"deny","message":"<text>"}`. The message is one line of at most
  500 characters. The file stays until its entry leaves, so the queue refuses a
  second answer.
- `lists/<id>`: the mark of one open list. Its list keeps a lock on the file. A
  service process and `approval-queue-notify` remove a mark that no process
  locks.
- `focus-list.js`: the KWin script of the last ping.
- `pings-muted-until`: the end of a mute, in seconds since 1970.

## Install

`pont -fx approval-queue` builds the programs with the locked dependencies and
installs them to `$XDG_DATA_HOME/approval-queue`. Its root step creates
`/run/approval-queue` through `/etc/tmpfiles.d/approval-queue.conf`. The module
applies only to writable dotfiles on a systemd machine where the `agent` group
exists.

A session needs the hook of its harness to send its prompts. For Claude Code,
that is `approval-queue.ts` of the `ai-prompts` module.

## Limits

- A process of the owner can write an answer file. The folder of the queue has
  the protection of the owner's other files, not more. A user of the `agent`
  group cannot read or write it.
- An entry shows what its hook sent. A user of the `agent` group can send a
  request for a call that no session waits for. An answer to it has no effect.
- The service cannot see why a hook keeps its connection. An entry whose prompt
  got its answer elsewhere stays until its hook closes the connection.
- The socket accepts 64 connections at a time. A further prompt stays only in
  the terminal of its session. The default poll limit of systemd applies: after
  150 connections in 2 seconds, the socket takes the next ones when that time
  ends.
- A request longer than 65536 bytes gets an error reply. Its prompt stays only
  in the terminal.
- A `list_only` prompt waits while a list is open, also a list that nobody
  watches. Its terminal shows no prompt in that time.

## Check

```sh
systemctl --user status approval-queue.socket
printf '%s\n' '{"version":1,"host":"check","user":"'"$USER"'","session":"check","folder":"/","mode":"default","tool":"Bash","input":{"command":"true"},"reason":null,"list_only":false}' |
	socat -,ignoreeof UNIX-CONNECT:/run/approval-queue/request.sock
```

The second command waits. Its request shows in `approval-queue`. An answer there
prints the reply and ends the command. `cargo test` in `approval-queue` tests
the request form, the files of the queue, whole connections, the keys of the
list and what the list draws. The tests give the list its terminal events
directly. No test runs a real terminal, so none covers its bracketed paste and
focus reporting modes.
The tests of `approval-queue-notify` run it with stand-ins for `busctl` and
`systemd-run`. No test runs KWin, `ghostty` or `focus_list.js`.

To see a ping, close each list, send the request above and wait: a terminal
with the list opens and gets the focus.

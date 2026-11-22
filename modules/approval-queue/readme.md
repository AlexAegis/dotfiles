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

`approval-queue` opens the list in a terminal. In a terminal of at least 100
columns, the list has two columns. In a narrower terminal, the left column is
above the right one.

- Left: one entry for each waiting prompt, the oldest first.
  - Line 1: the kind, `PERMISSION` or `QUESTION`. At the right: how long the
    prompt waits, or its answer, `approved` or `declined`.
  - Line 2: the title of the request. For a request without one: the tool with
    the command or the file of the call, else with the whole input, at most
    144 characters.
  - Line 3: user, tool, name of the folder, start of the session ID.
  - The border counts the entries above and below the shown ones. Its lower
    edge shows how long the pings are muted.
- Below the entries, while a session reports its state: one line for each
  session, at most 8, with a count of the others in the border. Idle sessions
  come first, then waiting ones, then active ones. In each group, the oldest
  state comes first. A narrow part cuts the end of a line. In a narrow
  terminal, the lines take their rows from the selected prompt.
  - Lamp:
    - green `[##]`: active, a turn runs
    - green `[# ]`: waiting, no turn runs and something that the session
      started still runs, such as a build or a wait for a CI run
    - yellow `[##]`: idle, the turn ended with nothing in flight
  - Then: name of the folder, `active`, `waits` or `idle` with the time since
    then, for `waits` the first thing that the session waits for and the count
    of the others, user, start of the session ID.
- Right: the selected prompt in full: user, harness, permission mode, session,
  folder, the reason of a guard hook that asked, the title and the input of the
  tool. A `description` of the input that equals the title is not shown again.
  The last line names the keys that answer. The border counts the lines above
  and below.
  - The command of a `Bash` call has the colors of a shell script.
  - The content of a `Write` call has the colors of the language that the
    extension of its file names, from the default syntaxes of `syntect`. A
    TypeScript file has the colors of JavaScript. The content of a file with
    another extension, or without one, has no colors.
  - The two texts of an `Edit` call show as one diff under `change`: `-` and red
    for a line of only the old text, `+` and green for a line of only the new
    one. A line of both texts shows after a blank. When the product of the line
    counts of both texts without their common first and last lines is above
    4000000, the diff shows those old lines, then those new ones. An input with
    a field `change` shows both texts as they are.
  - The colors are those of the terminal. A comment is dim. A tab shows as
    `\t`. Each other control character shows as its escape. A command or a
    content with a line above 4096 bytes has no colors. After 250 ms, the lines
    that follow have none.

| Key              | Effect                                          |
| ---------------- | ----------------------------------------------- |
| `a`              | approves the selected prompt                    |
| `d`              | declines it                                     |
| `m`              | opens the message for the answer to the prompt  |
| `j`, `k`, arrows | select                                          |
| PgUp, PgDn       | scroll the selected prompt                      |
| `s`              | mutes the pings for 10 minutes                  |
| `S`              | mutes the pings for 1 hour                      |
| `u`              | ends the mute                                   |
| `q`, Ctrl-C      | quit                                            |

- An answer names its entry by ID. The selection stays on its entry while other
  entries come and go.
- When the selected entry leaves, the oldest entry gets the selection.
- The list takes `a`, `d`, `s`, `S`, `u` and `q` only after 1 second
  without a key press, pasted text, a new selection by the list or a new focus
  of its window. A key that the list does not use counts as a key press too. A
  key meant for the entry that left then does not answer the next one. Text
  typed into the list without a pause of 1 second sends no answer, except
  with its first key: an `a` or `d` there answers one prompt. An `s` or `S`
  there mutes the pings. An `m` in such text opens a message with the rest of
  the text, which sends nothing (owner, 2026-11-22). The message field below
  the actions shows that text, and the next answer to that prompt sends it.
- A ping gives the window of the list the focus. Keys that the owner types for
  another window in the second after that answer no prompt, change no mute and
  do not quit with `q`. An open message or own answer then takes a key or pasted
  text only after 1 second without either. The keys that select and scroll, and
  Ctrl-C, act at once. A terminal that does not report the focus gives no such
  second.
- A key without a function gets a notice with its name, such as
  `Key Alt+a has no function`.
- With Caps Lock on, `s` arrives as `S` and mutes for 1 hour.
- `m` opens the message field below the actions. Each key is then text of the
  message, also when the prompt leaves the queue in that time. Enter or Esc
  closes the field and keeps the text of at most 500 characters. The next answer
  to that prompt sends the message with it: an `a`, a `d` or the answers to its
  questions. `m` opens the text again. An empty text takes the message away. The
  field shows the whole message. In a pane too small for it, the field shows the
  end of a message that the owner types and the start of a kept one, and the
  border of the pane counts the lines that do not show. A message whose prompt
  left shows in the footer and is dropped when it closes.
- A key with Ctrl or Alt is not its plain key: Ctrl-D is no `d`. Ctrl-C quits.
- Pasted text is text of an open message or own answer and no key. A terminal
  without bracketed paste sends pasted text as keys.
- `?` before a user name: the service could not verify the name. It verifies
  only the name of the owner.
- Control characters of a request show as escapes, such as `\u{1b}`. So do the
  blank and invisible characters that `display_text.rs` lists, such as the
  direction overrides. Unicode has more such characters than that list.
- A prompt of the tool `AskUserQuestion` with 1 to 4 questions, each with a
  text of its own and at most 9 options, shows one question after the other:
  - `1` to `9` mark an option or take its mark away. A question with one
    answer keeps one mark.
  - `t` opens an own answer as text. Enter keeps it, Esc closes the text
    without a change. After `t`, each key is text of the answer, also when the
    prompt leaves the queue in that time. A question with one answer takes
    the own answer in place of a mark. A question with several answers takes
    it after its marked options.
  - Enter shows the next question when the current one has an answer. Esc
    shows the question before. The marks stay while another prompt is
    selected.
  - Enter at the last question sends the answers. `d` declines the prompt, as
    for a permission prompt. `a` does not answer a question.
  - The Enter that sends and `d` obey the rule of 1 second without a key. The
    keys that mark an option, show another question or open a text act at
    once: they send nothing.
  - Another prompt of that tool, such as one with an option that the queue
    does not take as an answer, shows as its input, with the keys of a
    permission prompt. An `a` there leaves the dialog to the terminal.
- Several lists can run at the same time. The queue takes the first answer to a
  prompt and refuses a later one.
- After the last list quits, the prompts stay in the queue, except those with
  `list_only`.

## Notify

`approval-queue-notify.service` runs `approval-queue-notify` in the graphical
session. It shows a tray icon and checks the entry files 2 times each second.
It pings at a cadence, so that many prompts give few pings. Each prompt counts
for at most one ping. Until then it waits for its ping:

- A prompt pings at once when no prompt waits for a ping and the last ping is
  at least 30 seconds old.
- After a ping, no ping comes for 30 seconds.
- After that time, a ping comes when 4 prompts wait for a ping, or, with
  fewer, when the oldest of them has waited 2 minutes.
- A prompt that leaves the queue before its ping counts for none.

A session that is idle for 1 minute counts as one more prompt for the cadence,
one time for each time that it goes idle (owner, 2026-11-22). The icon counts
it from then on too. A session that gets its next turn sooner pings for nothing
and shows only in the list.

The cadence comes from the
[`ping_throttle`](https://github.com/AlexAegis/ping_throttle) crate. The values
above are those of the preset `Normal`. The menu of the tray icon sets another
preset. At a new preset, the waiting prompts keep their wait. The quiet time
keeps its start. A restart of the service starts the cadence again: each waiting
prompt then counts as new.

The cadence counts the time that the machine runs, without the time of a
suspend.

`s` and `S` in the list mute the pings, as the menu of the icon does. The lower
border of the list and the tooltip of the icon show the minutes left. A later
mute sets a new time. While the mute lasts, no ping comes. The icon still shows
what waits. The prompts keep their wait: when the mute ends, one ping comes for
those that still wait for theirs. A mute ends at a clock time: the time of a
suspend counts. It also lasts after the list quits.

### Tray Icon

The icon is a StatusNotifierItem on the session bus, an inbox:

| Icon                          | Shows                      |
| ----------------------------- | -------------------------- |
| outline in the text color     | nothing waits              |
| white with a yellow border    | a ping, for 400 ms         |
| yellow with an attention mark | a permission prompt waits  |
| blue with a question mark     | only question dialogs wait |
| yellow with a white lamp      | only idle sessions wait    |

A question dialog is a prompt of the `AskUserQuestion` tool.

While a prompt waits or a session is idle for 1 minute, the icon has the status
`NeedsAttention`, so a desktop that hides idle icons shows it. The tooltip has
the counts. A click on the icon
brings the list to view, as steps 1 and 2 below describe. The menu of the icon:

- Open the list
- High alert: a ping also brings the list to view. Off by default.
- Mute the pings: for 10 minutes, for 1 hour, unmute
- Cadence of the pings:

  | Preset | Quiet time | Ping for  | Or when the oldest waited |
  | ------ | ---------- | --------- | ------------------------- |
  | Often  | 10 seconds | 2 prompts | 30 seconds                |
  | Normal | 30 seconds | 4 prompts | 2 minutes                 |
  | Seldom | 2 minutes  | 8 prompts | 10 minutes                |

The high alert and the cadence do not depend on each other. Both stay over a
restart, in `settings.json` of `$XDG_STATE_HOME/approval-queue`. While no
desktop service takes the icon, or the program cannot show it, such as without a
session bus, each ping brings the list to view, as with high alert. The journal
has the reason.

### High Alert

With high alert, a ping also does this:

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

A ping then shows nothing more than the icon while that list is open. The script
does not move a window between activities. A ping also opens a list while nobody
watches the screen.

On a desktop without KWin, step 1 fails and the journal has the reason. Step 2
still runs:

```sh
journalctl --user -u approval-queue-notify.service
```

## Requests

One JSON line, at most 65536 bytes with the newline. Each field but `title` is
required. A text with a limit is not empty:

| Field       | Content                                                           | Limit          |
| ----------- | ----------------------------------------------------------------- | -------------- |
| `version`   | `1`                                                               |                |
| `host`      | the harness, such as `claude-code`                                | 32 bytes       |
| `user`      | the user that the hook runs as                                    | 64 bytes       |
| `session`   | the ID of the session                                             | 128 bytes      |
| `folder`    | the working folder of the session                                 | 4096 bytes     |
| `mode`      | the permission mode of the session                                | 32 bytes       |
| `tool`      | the name of the tool                                              | 256 bytes      |
| `title`     | what the call is for, on one line, written by the sender          | 144 characters |
| `input`     | the input of the tool call, an object                             |                |
| `reason`    | why a guard hook asked, or `null`                                 |                |
| `list_only` | `true` or `false`: only the list can answer the prompt, see below |                |

The hook keeps the connection open. After its request, it can send lines of the
form `{"title":"<text>"}`: each gives the waiting prompt that title in place of
the one before, such as a summary that took time to write. The rules of the
request title apply. These change nothing: a line of another form, a title that
the queue cannot store. A line of more than 2048 bytes, its newline included,
ends the wait without a reply. A service before this version ends the wait at
each later byte.

The hook ends its wait after 1 day. The reply is one JSON line:

- `{"behavior":"allow"}`, with a `message` when the owner typed one
- `{"behavior":"allow","answers":{"<question>":"<answer>"}}` for a prompt with
  questions: one answer for each question, by the text of the question. An
  answer is the label of the marked option, the owner's own text or the
  labels of several marked options and the own text joined with `, `, at most
  500 characters. A `message` comes with it when the owner typed one.
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

### Session Reports

A hook can also report the state of its session, on a connection of its own.
The line names its kind, which no request does:

| Field            | Content                                                         | Limit                         |
| ---------------- | --------------------------------------------------------------- | ----------------------------- |
| `version`        | `1`                                                             |                               |
| `kind`           | `session`                                                       |                               |
| `host`           | the harness, such as `claude-code`                              | 32 bytes                      |
| `user`           | the user that the hook runs as, an account name as in a request | 64 bytes                      |
| `session`        | the session ID: letters, digits, `-` and `_`                    | 128 bytes                     |
| `folder`         | the working folder of the session                               | 4096 bytes                    |
| `pid`            | the process of the session: the sender or a process above it    |                               |
| `state`          | `active`, `waiting`, `idle` or `ended`                          |                               |
| `sent_at`        | milliseconds since 1970 at which the hook wrote the line        |                               |
| `waits_for`      | optional: what a waiting session waits for                      | 16 items, 144 characters each |
| `seconds_ago`    | optional: the seconds for which the session already has `state` | 86400 seconds                 |
| `unless_waiting` | optional, `true` with `idle`: replaces no waiting state         |                               |

The hook keeps the connection open until the service closes it: the service
checks the sender while it runs, and refuses the line of a sender that ended.
The service stores the state in place of the one before and sends no reply. A
state that stays keeps its start. `ended` removes the state. These lines change
nothing and get no reply:

- a line with a `sent_at` before that of the stored state. Each line comes on
  its own connection, so two lines of one session can arrive in the other
  order. A stored `sent_at` more than 5 seconds after the time of the
  service, as after a step back of the clock, does not count.
- a line with `unless_waiting` for a session that waits

A refused report gets `{"error":"<reason>"}`:

- a line of another form
- a `user` that the sender is not, by the rule of a request
- a `pid` that names neither the sender nor a process above it, or that names
  process 1. The state leaves when that process ends, so a sender cannot keep
  a state for longer than a process that it comes from. The service does not
  compare user IDs for this: in its unit, every other user has one ID. It
  only refuses a process of the owner from another sender.
- a `sent_at` more than 5 seconds after the time of the service
- `unless_waiting` with a `state` other than `idle`
- a session whose stored state has another user
- a new session of a user that has 64 sessions with a state

Limit: `pid` and the sender must be process IDs in the PID namespace of the
service. The service refuses the report of a hook in another PID namespace, or
takes it for a wrong process.

## Files

`$XDG_RUNTIME_DIR/approval-queue`, for the owner only:

- `entries/<id>.json`: one waiting prompt. Its service process keeps a lock on
  the file. The list and `approval-queue-notify` remove an entry file that no
  process locks.
- `titles/<id>`: the later title of that entry, when its hook sent one. The
  service process removes it with the entry.
- `answers/<id>.json`: the answer for that entry, in the form of the reply
  line: `{"behavior":"allow"}`, with `answers` for a prompt with questions, or
  `{"behavior":"deny"}`, each with a `message` when the owner typed one. The
  message is one line of at most 500 characters. The file stays until its
  entry leaves, so the queue refuses a second answer.
- `sessions/<session>.json`: the state of one session that reported it, with
  the time since which it has it. The list, `approval-queue-notify` and the
  service remove the state of a session whose process is gone.
- `lists/<id>`: the mark of one open list. Its list keeps a lock on the file. A
  service process and `approval-queue-notify` remove a mark that no process
  locks.
- `focus-list.js`: the KWin script of the last ping.
- `icons/`: the icons of the tray, as an icon theme folder.
- `pings-muted-until`: the end of a mute, in seconds since 1970.

`$XDG_STATE_HOME/approval-queue`:

- `settings.json`: the high alert and the cadence preset of the menu.

## Install

`pont -fx approval-queue` builds the programs with the locked dependencies and
installs them to `$XDG_DATA_HOME/approval-queue`. Its root step creates
`/run/approval-queue` through `/etc/tmpfiles.d/approval-queue.conf`. The module
applies only to writable dotfiles on a systemd machine where the `agent` group
exists.

A session needs the hook of its harness to send its prompts. For Claude Code,
that is `approval-queue.ts` of the `ai-prompts` module.

## Limits

- The program knows whether a desktop service takes tray icons, not whether a
  panel draws them. While the panel is down and that service runs, a ping
  without high alert shows nothing.
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
`systemd-run`. The tests of the tray run it on a session bus of their own
(`dbus-daemon`) and click its menu through D-Bus (`busctl`). They fail where a
sandbox refuses unix sockets. No test runs KWin, `ghostty`, `focus_list.js` or a
desktop that draws the icon.

To see a ping, send the request above: the icon flashes, then turns yellow.
With high alert and no open list, a terminal with the list also opens and gets
the focus.

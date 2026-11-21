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
| `q`, Ctrl-C      | quit                                            |

- An answer names its entry by ID. The selection stays on its entry while other
  entries come and go.
- When the selected entry leaves, the oldest entry gets the selection.
- The list takes `a`, `d`, `m` and `q` only after 1 second without a key press,
  pasted text or a new selection by the list. A key that the list does not use
  counts as a key press too. A key meant for the entry that left then does not
  answer the next one. Of text typed into the list without a pause of 1 second,
  only the first key can act. An `a` or `d` there answers one prompt. An `m`
  there opens a message with the rest of the text, which Enter sends as a
  denial.
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
  the file. The list removes an entry file that no process locks.
- `answers/<id>.json`: the answer for that entry, `{"behavior":"allow"}` or
  `{"behavior":"deny","message":"<text>"}`. The message is one line of at most
  500 characters. The file stays until its entry leaves, so the queue refuses a
  second answer.
- `lists/<id>`: the mark of one open list. Its list keeps a lock on the file. A
  service process removes a mark that no process locks.

## Install

`pont -fx approval-queue` builds both programs with the locked dependencies and
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
directly. No test runs a real terminal, so none covers its bracketed paste mode.

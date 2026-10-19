# Agent user

Settings for a user that applies these dotfiles from a read-only copy and
installs no packages, such as the user that coding agents run as. It applies
only where the dotfiles are not writable, so never to the owner's own setup.

It links a `pontrc` that turns off root scripts, package installs, the base
modules, and permission fixes. It keeps the install hashes in
`$XDG_STATE_HOME/pont/hashes` and sets `headless`.

The first run has no `pontrc` yet. Link it with these settings on the command
line, then apply the `agent` preset:

```sh
env PONT_ROOT_FLAG=0 PONT_FIX_PERMISSIONS=0 \
	PONT_HASH_HOME="$HOME/.local/state/pont/hashes" \
	sh ~/.config/dotfiles/modules/pont/pont.sh -fx agent-user
sh ~/.config/dotfiles/modules/pont/pont.sh -ex +agent
```

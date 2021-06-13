# macOS

macOS specific system setup

## Manual Setup

### Finder

- `View > Sort By > Name (Snap to Grid)`
- `View > Show Path Bar`
- `Finder > Prefrences > General`
  - Untick desktop items

### Keyboard settings

System > Keyboard > Shortcuts > Input Sources

Change `Select the previous input source` from `Control+Space to Option+Space`
and the `Select next source in Input menu` to `Shift+Option+Space`.

> This shortcut prevents the `Control+Space` keyevent from reaching any other
> programs, so in many IDEs and editors where it would open the code
> suggestions/completion menu it wouldn't work.
> The suggested alternative of using the Option key makes this feature in-line
> with Windows where input sources are selectable with `Super+Space` and
> `Shift+Super+Space`

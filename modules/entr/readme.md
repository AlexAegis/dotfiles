# [entr](http://eradman.com/entrproject/)

Run arbitrary commands when files change

## Watchers

This module contains a `~/.config/entr/watchers` folder in which any script will be executed as part of the `entr-watch` service.

## Notes on Systemd

While systemd services can be used to achieve the same result, this entr
solution is easy to expand, and only the watcher itself depends on systemd
which is easier to expand on non-systemd distributions.

But using systemd for this would be more correct on systemd distributions
than using something that's originally a test-runner.

TODO: Decide on entr vs systemd

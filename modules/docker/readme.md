# [Docker](https://www.docker.com/)

> [Arch Wiki](https://wiki.archlinux.org/index.php/docker)

TODO: Download some cool containers, like explainshell for the BASH IDE
docker run --rm --name bash-explainshell -p \
5000:5000 chrismwendt/codeintel-bash-with-explainshell

## Permissions

To interact with docker, you need permissions for the `docker.sock` socket.
Either enable it globally, or add yourself to the group `docker`.

```sh
sudo chmod 666 /var/run/docker.sock
```

or

```sh
sudo usermod -a -G docker "$USER"
```

This module does this by default, but you have to do a login to be in the
group.

You can just invoke a login shell:

```sh
exec su -l "$USER"
```

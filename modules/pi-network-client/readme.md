# Pi Network Client

A home network client configuration. It contains ssh configuration to access
the local server.

## Adding your ssh key to the server

> If you want to specify which key it should use, use the `-i` flag

```sh
ssh-copy-id $USER@pi
```

## WSL

If you are on WSL, you can copy the generated `pi_rsa` keys to your host
machine too.

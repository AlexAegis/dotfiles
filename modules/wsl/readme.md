# [WSL](https://docs.microsoft.com/en-us/windows/wsl/about)

[Installation Guide](https://docs.microsoft.com/en-us/windows/wsl/install-win10)

## Password reset

Changing the password is the same as in any other distro, using `passwd` as the
root user. The fortunate thing is that WSL doesn't ask for a password for the
initial login.

Change your default user from PowerShell or cmd:

```ps
ubuntu config --default-user root
```

> Use the launcher exe of your distribution if you don't use ubuntu.exe

Launch it, change your users password if necessary:

```sh
passwd <username>
```

And change the password for the root user:

```sh
passwd
```

Then change the default user back to yours from PowerShell/cmd:

```ps
ubuntu config --default-user <username>
```

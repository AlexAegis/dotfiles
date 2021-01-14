# Windows

## [Chocolatey](https://chocolatey.org/)

To update everything

```ps1
choco upgrade all -y
```

To update everything on startup

```ps1
choco install choco-upgrade-all-at-startup -y
```

### [Git](https://git-scm.com/download/win)

```ps1
choco install git -y
```

### [VS Code](https://code.visualstudio.com/)

```ps1
choco install vscode -y
```

### [NodeJS](https://nodejs.org/en/)

```ps1
choco install nodejs -y
```

### [Python](https://www.python.org/)

```ps1
choco install python -y
```

### [qBittorrent](https://www.qbittorrent.org/)

```ps1
choco install qbittorrent -y
```

## WSL

[Installation Guide](https://docs.microsoft.com/en-us/windows/wsl/install-win10)

### Windows Terminal

[From Microsoft Store](https://www.microsoft.com/en-us/p/windows-terminal/9n0dx20hk701)

### Ubuntu

[From Microsoft Store](https://www.microsoft.com/en-us/p/ubuntu/9nblggh4msv6)

## Fonts

[Powerline Fonts](https://github.com/powerline/fonts)

### PowerShell Exectuion Policy

Enable PowerShell scripts

```ps1
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope CurrentUser
```

## Drivers

### [Nvidia](https://www.nvidia.com/en-us/geforce/drivers/)

```ps1
choco install nvidia-display-driver
```

## Settings

### Keyboard

Tick in `Let me use a different input method for each app window`

### Passworldless Login

1. Make sure `Require Windows Hello sign-in for Microsoft Accounts` is turned
   off within `Sign-in Options` from `Settings`
2. Run `netplwiz`
3. Uncheck `Users Must Enter a User Name and Password to Use This Computer`
4. Click Ok and enter your Microsoft accounts password

## VoIP

### [Discord](https://discord.com/download)

### [TeamSpeak](https://www.teamspeak.com/en/downloads/)

## Audio

### [VoiceMeeter Banana](https://vb-audio.com/Voicemeeter/banana.htm)

## Video

### [VLC](https://www.videolan.org/vlc/)

```ps1
choco install vlc -y
```

### [HEVC Video Codecs](https://www.microsoft.com/en-us/p/hevc-video-extensions/9nmzlz57r3t7)

### [Plex Media Server](https://www.plex.tv/media-server-downloads/#plex-app)

```ps1
choco install plexmediaserver -y
```

## Gaming

### [Steam](https://store.steampowered.com/)

```ps1
choco install steam -y
```

### [Battle.net](https://www.blizzard.com/en-us/apps/battle.net/desktop)

### [Epic Games](https://www.epicgames.com/store/en-US/download)

### [Ubisoft Connect](https://ubisoftconnect.com/en-US/)

### [Origin](https://www.origin.com/irl/en-us/store/download)

### [Riot Games](https://signup.eune.leagueoflegends.com/en/signup/redownload)

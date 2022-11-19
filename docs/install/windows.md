# Windows

## Timezone settings

To dual boot with Windows it is recommended to configure Windows to use UTC,
rather than Linux to use localtime.

> From elevated command line or powershell

```ps
reg add "HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\TimeZoneInformation" /v RealTimeIsUniversal /d 1 /t REG_DWORD /f
```

Make sure to set both `Set time automatically` and
`Set timezone automatically` to on in the Date & time settings page, then hit
Sync Now.

## [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)

WSL is preinstalled on Windows 11, just run the following command to install `Unbuntu` for example:

```ps1
wsl --install --distribution Ubuntu
```

To get a list of available distributions run

```ps1
wsl --list --online
```

## [Chocolatey](https://chocolatey.org/)

To update everything

```ps1
choco upgrade all -y
```

To update everything on startup

```ps1
choco install choco-upgrade-all-at-startup -y
```

To enable auto confirmation on installs

```ps1
choco feature enable -n allowGlobalConfirmation
```

### [Git](https://git-scm.com/download/win)

```ps1
choco install git -y
```

### [VS Code](https://code.visualstudio.com/)

```ps1
choco install vscode -y
```

### [fnm](https://github.com/Schniz/fnm) 

> To then install [NodeJS](https://nodejs.org/en/)

```ps1
choco install fnm -y
```

Then make sure you have a PowerShell profile file at
`~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`
with the following content:

```ps1
fnm env --use-on-cd | Out-String | Invoke-Expression
```

### [Rust](https://www.rust-lang.org/)

```ps1
choco install rust -y
```

### [Python](https://www.python.org/)

```ps1
choco install python -y
```

### [gnupg](https://www.gnupg.org/)

```ps1
choco install gnupg -y
```

### [mingw](https://www.mingw-w64.org/)

```ps1
choco install mingw -y
```

### [7zip](https://www.7-zip.org/)

```ps1
choco install 7zip -y
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

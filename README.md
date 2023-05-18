# ElvUI Refresh CLI 📦

This tool allows users to download and install the latest version of ElvUI to their Blizzard addons folder.

## Installation

### (Easiest) Are you a Rustacean? 🦀

Run `cargo install elvui-refresh`

### Manual installation

Open `bash.exe` on windows and run the following:

```
$ curl https://github.com/chancehl/elvui-refresh/releases/download/v1.0.3/elvui-refresh-x86_64-pc-windows-msvc.tar.gz --output %userprofile%\Downloads\elvui-refresh.tar.gz -L

$ tar -xf %userprofile%\Downloads\elvui-refresh.tar.gz
```

Replace the version with the one that you are trying to download (latest = 1.0.3). You can now execute the program by running: `%userprofile%\Downloads\elvui-refresh.exe`.

## Help

```
A program to download, extract and install the latest version of ElvUI

Usage: elvui-refresh [OPTIONS]

Options:
  -f, --addons-folder <ADDONS_FOLDER>  The addons folder location
  -h, --help                           Print help
  -V, --version                        Print version
```

# ElvUI Refresh CLI 📦

This tool allows users to download and install the latest version of ElvUI to their Blizzard addons folder.

## Installation

### (Easiest) Are you a Rustacean? 🦀

Run `cargo install elvui-refresh`

### Manual installation

Open `bash.exe` on windows and run the following:

```
$ curl https://github.com/chancehl/elvui-refresh/releases/download/v1.0.2/elvui-refresh-x86_64-pc-windows-msvc.zip --output %userprofile%\Downloads\elvui-refresh.zip -L

$ tar -zvxf %userprofile%\Downloads\elvui-refresh.tar.gz
```

## Help

```
A program to download, extract and install the latest version of ElvUI

Usage: elvui-refresh [OPTIONS]

Options:
  -f, --addons-folder <ADDONS_FOLDER>  The addons folder location
  -h, --help                           Print help
  -V, --version                        Print version
```

# ElvUI Refresh CLI 📦

This tool allows users to download and install the latest version of ElvUI to their Blizzard addons folder.

## Installation

### (Easiest) Are you a Rustacean? 🦀

Run `cargo install elvui-refresh`

### Manual installation 🔨

Open `bash.exe` on windows and run the following:

```
# Downloads the file
$ curl https://github.com/chancehl/elvui-refresh/releases/download/v1.0.3/elvui-refresh-x86_64-pc-windows-msvc.tar.gz --output %userprofile%\Downloads\elvui-refresh.tar.gz -L

# Unzips the file
$ tar -xf %userprofile%\Downloads\elvui-refresh.tar.gz
```

Replace the version with the one that you are trying to download (latest = 1.0.3). You can now execute the program by running: `%userprofile%\Downloads\elvui-refresh.exe`.

### Adding the exe to $PATH on Windows 🪟

1. Search "environment variables" in the start menu on windows
2. Select the "edit environment variables" option
3. Under the "Advanced" tab select "Environment Variables..."
4. Select the entry in the table with key = "Path"
5. Click the "Edit" button
6. Click the "New" button on the window that pops up
7. Add the location where you downloaded & extracted the `.tar.gz` file from above (note: this must end in .exe)
8. Click "OK", close menus
9. Reopen your terminal and you should be able to run as such:

```
$ elvui-refresh --help
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

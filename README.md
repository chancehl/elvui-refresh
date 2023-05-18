# ElvUI Refresh CLI 📦

This tool allows users to download and install the latest version of ElvUI to their Blizzard addons folder.

## Installation

### (Easiest) Are you a Rustacean? 🦀

Run `cargo install elvui-refresh`. Afterwards you should be able to run the program like this:

```
$ elvui-refresh --version
```

### Semi-automated installation 🚙

Open `bash.exe` on windows and run the following:

```
# Downloads the file
$ curl https://github.com/chancehl/elvui-refresh/releases/download/v1.0.3/elvui-refresh-x86_64-pc-windows-msvc.tar.gz --output %userprofile%\Downloads\elvui-refresh.tar.gz -L

# Unzips the file
$ tar -xf %userprofile%\Downloads\elvui-refresh.tar.gz
```

Replace the version with the one that you are trying to download (latest = 1.0.3). You can now execute the program by running: `%userprofile%\Downloads\elvui-refresh.exe`.

### Manual installation 🔨

1. Navigate to the [ElvUI refresh Github repository](https://github.com/chancehl/elvui-refresh)
2. On the right-hand side look for a panel that says "releases"
3. Click the link to the release that says "latest"
4. Download the appropriate .zip for your operating system (Windows users will want the .zip most likely)
5. Extract the zip
6. You should now be able to run the program like this:

```
$ C:\Path\To\Where\You\Extracted\The\Zip\elvui-refresh.exe --version
```

### Adding the exe to $PATH on Windows 🪟

1. Search "environment variables" in the start menu on windows
2. Select the "edit environment variables" option
3. Under the "Advanced" tab select "Environment Variables..."
4. Select the entry in the table with key = "Path"
5. Click the "Edit" button
6. Click the "New" button on the window that pops up
7. Add the location where you downloaded & extracted the `.tar.gz` file from above (note: this must end in .exe)
8. Click "OK", close menus
9. Reopen your terminal and you should be able to run like this:

```
$ elvui-refresh --version
```

## Running the program

With an explicit addon folder location:

```
$ elvui-refresh --addon-folder C:\Path\To\Your\Blizzard\Addons\Folder
```

With a variable instead of the addon folder flag

```
$ EXPORT BLIZZARD_ADDONS_FOLDER="C:\Path\To\Your\Blizzard\Addons\Folder"
$ elvui-refresh
```

The output of both commands should look something like this if it was successful:

```
[info] Executing
[info] If you don't want to provide the -f/--addons-folder flag every time you execute this command, you can set the $BLIZZARD_ADDONS_FOLDER environment variable
[info] Fetching latest ElvUI version from Github...
[info] Latest version: v13.32
[info] Downloading addon source code...
[info] Created temp directory at "/tmp/.tmpwZsQpc"
[info] Extracting addon files...
[info] Copying addon files...
[info] Cleaning up...
[info] Success! Upgraded ElvUI to version v13.32
```

If you see anything that deviates significantly from this example, it's likely the program failed to execute. If that happens, you can open an issue in this Github repository or contact me.

## Help

```
A program to download, extract and install the latest version of ElvUI

Usage: elvui-refresh [OPTIONS]

Options:
  -f, --addons-folder <ADDONS_FOLDER>  The addons folder location
  -h, --help                           Print help
  -V, --version                        Print version
```

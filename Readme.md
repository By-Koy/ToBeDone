# ToBeDone
**A cross-platform TUI note-taking app**

## Quick-start
  **Linux/Macos (anything UNIX-like really)**\
  ``curl https://raw.githubusercontent.com/By-Koy/ToBeDone/refs/heads/main/scripts/install.sh | zsh ``
  ###### (Requires curl, ZSH, a full [Rust](https://rust-lang.org/tools/install/) install and Git)

**Windows**\
The easiest way to use the app on Windows is to download the portable .exe from the latest release, then running it next to the TBD folder.(they must be in the same directory for the portable version to work)

## Features (and to-do list)
- [ ] ~~**Styling support**~~ **(CANCELED)** - A screen to enable styling and formating. this is instead of markdown which I sadly couldn't figure out :sadge. this feature has been cancled, please check out [this section](https://github.com/By-Koy/ToBeDone/tree/main#a-few-technical-considerations) for more info
- [x] **Memory** - places notes in a system-wide location for easy access via title
- [x] **Recent file** - automatically gets the last used file when a title is not supplied
- [x] **Cross-platform** - works on windows, linux and macos.
- [x] **Runs quickly and efficiently** right in your terminal.
- [x] **Written in Rust** ;P

## Installation
### Install script
**Linux/Macos (anything UNIX-like really)**\
  ``curl https://raw.githubusercontent.com/By-Koy/ToBeDone/refs/heads/main/scripts/install.sh | zsh ``
  ###### (Requires curl, ZSH, a full [Rust](https://rust-lang.org/tools/install/) install and Git)

### Testing (currently linux/macos only)\
These commands build the portable version of the app.\
It will be put in your Downloads folder and requires a folder named TBD to be put in the same directory.
```
git clone https://github.com/By-Koy/ToBeDone.git && cd ToBeDone
./build_portable.sh
```
###### (requires a full [Rust](https://rust-lang.org/tools/install/) install and Git)

Alternatively, you may use the pre-built binaries in the latest release

## Uninstallation
**Linux/Macos (anything UNIX-like really)**\
  ``curl https://raw.githubusercontent.com/By-Koy/ToBeDone/refs/heads/main/scripts/uninstall.sh | zsh ``
  ###### (Requires ZSH, a full [Rust](https://rust-lang.org/tools/install/) install and Git)

## A few technical considerations
The project it built with Rust and Ratatui to maximize efficiency of both the program and development, and to help me learn and get comfortable with a new library.\
I decided to use the Crossterm backend for mostly the same reasons, as well as it seaming like the easiest to use and the mot compatible of all the options easily accessible from Ratatui.\
Quite a bit later in development I had realized some parts of the markdown spec aren't possible within a TUI (mostly changing text size) with this and a few other considerations I decided to modify the markdown spec, a full guide of how it works will be added once the feature is out of development.\
After further consideration, the whole formatting idea went way out of scope even after cutting it. saving such a complecated dataset into a file is not an easy feat, at least for my skill level (and I have grown very sick of this project [![
](badge)](https://hackatime.hackclub.com/api/v1/badge/U0ALZQ9FP9P/By-Koy/ToBeDone)) I have come to the conclusion that a much more sensible approach is to cancel the feature and maybe add it in the future. 

## Legal and credits
Thanks to Hackclub and specificaly the Stardance team for giving me the motivation to make this project.
I don't think a cooler non-profit exists.

Thanks to Ren Gill ([RenMakesMusic](https://www.renmakesmusic.com/)) for the quote used in the sample text.

This project is protected by the GNU GPL license, for more information please visit the LICENSE file.

Thanks to StackOverflow user Socowi for the sudo code used in the install/uninstall scripts (for more info visit those files)

Made with love, by Koy.

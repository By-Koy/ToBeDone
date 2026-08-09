# ToBeDone
**A cross-platform TUI note-taking app**

## Quick-start
**Coming soon!!**\
For now, please look at the testing instructions!

## Features (and to-do list)
- [ ] **Styling support** - A screen to enable styling and formating. this is instead of markdown which I sadly couldn't figure out :sadge
- [x] **Memory** - places notes in a system-wide location for easy access via title
- [x] **Recent file** - automatically gets the last used file when a title is not supplied
- [ ] **Cross-platform** - works on windows, linux and macos.
- [x] **Runs quickly and efficiently** right in your terminal.
- [x] **Written in Rust** ;P

## Install
**Install script**
Under construction!

**Testing (currently linux/macos only)**
For faster development, currently the only way to use the program is by hosting notes at ``/var/local/TBD``, when windows support will be added a --path flag will be added to choose where notes are stored so that superuser privileges are not required for testing.
```
git clone https://github.com/By-Koy/ToBeDone.git
sudo mkdir -p /var/local/TBD && sudo chmod 777 /var/local/TBD
cd ToBeDone && cargo run -- --sample
```
(requires a full [rust](https://rust-lang.org/tools/install/) install)

## A few technical considerations
The project it built with Rust and Ratatui to maximize efficiency of both the program and development, and to help me learn and get comfortable with a new library.\
I decided to use the Crossterm backend for mostly the same reasons, as well as it seaming like the easiest to use and the mot compatible of all the options easily accessible from Ratatui.\
Quite a bit later in development I had realized some parts of the markdown spec aren't possible within a TUI (mostly changing text size) with this and a few other considerations I decided to modify the markdown spec, a full guide of how it works will be added once the feature is out of development.

## Legal and credits
Thanks to Hackclub and specificaly the Stardance team for giving me the motivation to make this project.
I don't think a cooler non-profit exists.

Thanks to Ren Gill ([RenMakesMusic](https://www.renmakesmusic.com/)) for the quote used in the sample text.

This project is protected by the GNU GPL license, for more information please visit the LICENSE file.

Made with love, by Koy.
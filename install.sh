#!/bin/zsh

# Check whether XDG_DATA_HOME is set and use fallback
if [[ -z $XDG_DATA_HOME ]]; then
export XDG_DATA_HOME="$HOME/.local/share"
fi

# Clone the repo and build
cd $HOME/Downloads
git clone https://github.com/By-Koy/ToBeDone.git && cd ToBeDone
cargo build --release

# Properly use sudo (requires sudo caching)
# Source - https://stackoverflow.com/a/42876846
# Posted by Socowi, modified by community. See post 'Timeline' for change history
# *Further modified by Koy
# Retrieved 2026-09-13, License - CC BY-SA 4.0
if [[ "$EUID" = 0 ]]; then
else
    sudo -k # make sure to ask for password on next sudo ✱
    if sudo true; then
    else
        echo "Wrong Password (or are you unable to use sudo?)"
        exit 1
    fi
fi

# Install the app
cp target/release/ToBeDone /usr/local/bin/
mkdir $XDG_DATA_HOME/TBD

# Clean up
rm -rf .
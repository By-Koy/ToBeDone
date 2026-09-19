#!/bin/bash

# Check whether XDG_DATA_HOME is set and use fallback
if [[ -z $XDG_DATA_HOME ]]; then
    export TBD_NOTES_HOME="$HOME/.local/share"
else
    export TBD_NOTES_HOME="$XDG_DATA_HOME"
fi

# Clone the repo and build
git clone "https://github.com/By-Koy/ToBeDone.git" "$HOME/Downloads/ToBeDone"
cd "$HOME/Downloads/ToBeDone" || echo "Could not clone!" && return
cargo build --release

# Properly use sudo (requires sudo caching)
# Source - https://stackoverflow.com/a/42876846
# Posted by Socowi, modified by community. See post 'Timeline' for change history
# *Further modified by Koy
# Retrieved 2026-09-13, License - CC BY-SA 4.0
if [[ "$EUID" = 0 ]]; then
    echo "Yay I'm superuser :gold_star:"
else
    sudo -k # make sure to ask for password on next sudo ✱
    if sudo true; then
        true
    else
        echo "Script requires sudo"
        exit 1
    fi
fi

# Install the app
sudo cp target/release/ToBeDone /usr/local/bin/
mkdir "$TBD_NOTES_HOME/TBD"

# Clean up
rm -rf "$HOME/Downloads/ToBeDone"
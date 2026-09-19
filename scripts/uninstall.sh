#!/bin/bash

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

#Remove files
rm -rf "$XDG_DATA_HOME/TBD"
sudo rm -f "/usr/local/bin/ToBeDone"
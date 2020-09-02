#!/usr/bin/env bash
set -euo pipefail


# Installation script for wng, learn more at : https://github.com/Wmanage/wng

binLink="https://github.com/Wmanage/wng/releases/tag/v2.10.18/wng-unix.18"
manLink="https://github.com/Wmanage/wng/releases/tag/v2.10.18/wng.gz"

echo "Downloading files ..."

curl $binLink -o /bin/wng &>/dev/null
curl $manLink -o /usr/local/man/man1/wng.1.gz &>/dev/null

success_bin=false
success_man=false

if [ $(ls /bin/ | grep wng) ]; then
    echo "Binary was installed succesfully"
    success_bin=true
else
    
    echo "Error while installing binary"
    echo "Installation failed"
fi

if [ $(ls /usr/local/man/man1/ | grep wng.1.gz) ]; then
    echo "Manual was installed succesfully"
    success_man=true
else 
    echo "Error while installing manual"
    echo "Installation failed"
fi

if [ $success_man == true ] && [ $success_bin == true ]; then
    echo "wng was installed succesfully"
else
    echo "Installation failed"
fi

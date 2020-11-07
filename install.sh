#!/usr/bin/env bash
set -euo pipefail


# Installation script for wng, learn more at : https://github.com/Wmanage/wng

binLink="https://github.com/Wmanage/wng/releases/download/3.2.0/wng"

echo "Downloading files ..."

curl $binLink -o /bin/wng &>/dev/null

success_bin=false

if [ $(ls /bin/ | grep wng) ]; then
    echo "Binary was installed succesfully"
    success_bin=true
else
    
    echo "Error while installing binary"
    echo "Installation failed"
fi


if [ $success_bin == true ]; then
    echo "wng was installed succesfully"
else
    echo "Installation failed"
fi

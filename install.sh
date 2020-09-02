# RUN ME AS SUDO


binLink="https://github.com/Wmanage/wng/releases/tag/v2.10.18/wng-unix.18"
manLink="https://github.com/Wmanage/wng/releases/tag/v2.10.18/wng.gz"

echo "Downloading files ..."

curl $binLink -o /bin/wng &>/dev/null
curl $manLink -o /usr/local/man/man1/wng.1.gz &>/dev/null

success_bin=false
success_man=false

if [-z "$(ls /bin/ | grep wng)"]; then
    echo "Error while installing binary"
else
    echo "Binary was installed succesfully"
    $success_bin=true
fi

if [-z "$(ls /usr/local/man/man1/ | grep wng.1.gz)"]; then
    echo "Error while installing manual"
else
    echo "Manual was installed succesfully"
    $success_man=true
fi

if [$success_man && $success_bin]; then
    echo "wng was installed succesfully"
else
    echo "Installation failed"
fi

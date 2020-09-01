# RUN ME AS SUDO


binLink="https://github.com/Wmanage/wng/releases/tag/v2.10.18/wng-unix.18"
manLink="https://github.com/Wmanage/wng/releases/tag/v2.10.18/wng.gz"

echo "Downloading files ..."

curl $binLink -o /bin/wng &>/dev/null
curl $manLink -o /usr/local/man/man1/wng.1.gz &>/dev/null

echo "wng was installed succesfully"

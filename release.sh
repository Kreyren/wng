#!/usr/bin/env bash
set -euo pipefail

OK="\033[0;32mok\033[0m\n"

if [[ -d "release" ]]
then
    printf "Deleting ancient release directory...... "
    rm -rf release/
    printf "${OK}"
fi

printf "Creating new release directory.......... "
mkdir release/
printf "${OK}"

# Check before building
printf "Checking if projet builds............... "
cargo check 2> /dev/null
printf "${OK}\n"

# Building project

## For GNU/Linux 64 bits

printf "Building project for gnu/linux x86_64... "
cargo build --release 2> /dev/null
printf "${OK}"

printf "Copying binary for gnu/linux x86_64..... "
cp target/release/wng release/wng-gnu-linux-x86_64
printf "${OK}\n"

## For Windows 64 bits

printf "Building project for windows x86_64..... "
cargo build --target x86_64-pc-windows-gnu --release 2> /dev/null
printf "${OK}"

printf "Copying binary for windows x86_64....... "
cp target/x86_64-pc-windows-gnu/release/wng.exe release/wng-windows-x86_64.exe
printf "${OK}\n"

## For Windows 32 bits

printf "Building project for windows i686....... "
cargo build --target i686-pc-windows-gnu --release 2> /dev/null
printf "${OK}"

printf "Copying binary for windows i686......... "
cp target/i686-pc-windows-gnu/release/wng.exe release/wng-windows-i686.exe
printf "${OK}\n"

printf "Creating build directory................ "
mkdir release/wng-userbuild
printf "${OK}"

printf "Copying files for user build............ "
cp -r wng/src/ release/wng-userbuild/
cp wng/Cargo.toml release/wng-userbuild/
printf "${OK}"

printf "Creating install script................. "
cat >> release/wng-userbuild/install << EOF
#!/usr/bin/env bash
set -euo pipefail
OK="\033[0;32mok\033[0m\n"
if ! command -v cargo &> /dev/null 
then
    echo "Cargo is needed to build WNG."
    exit 1
fi

echo "Building binary..."
cargo build --release
echo "Done"

mkdir -p $HOME/.cargo/bin

printf "Moving binary to ~/.cargo/bin/..... "
mv target/release/wng $HOME/.cargo/bin/
printf "${OK}"

printf "Creating ~/.wng folder........ "
mkdir -p $HOME/.wng
printf "${OK}"

printf "Creating ~/.wng.config file... "
if [[ ! -f $HOME/.wng.config ]]
then
    touch $HOME/.wng.config
fi
printf "${OK}\n"

echo "Installation successful, thanks for chosing WNG !"
EOF
printf "${OK}"

printf "Making install script executable........."
chmod 751 release/wng-userbuild/install
printf "${OK}"

cp -r release/wng-userbuild .

printf "Packing user build directory............ "
tar -czf release/wng-userbuild.tar.gz wng-userbuild/
printf "${OK}"

printf "Cleaning user build directory........... "
rm -rf release/wng-userbuild
rm -rf wng-userbuild
printf "${OK}"
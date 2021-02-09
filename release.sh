#!/usr/bin/env bash
set -euo pipefail

OK="\033[0;32mok\033[0m\n"

if [[ -d "release" ]]
then
    printf "Deleting ancient release directory...... "
    rm -rf release/
    printf $OK
fi

printf "Creating new release directory.......... "
mkdir release/
printf $OK

# Check before building
printf "Checking if projet builds............... "
cargo check 2> /dev/null
printf $OK

# Building project

## For GNU/Linux 64 bits

printf "Building project for gnu/linux x86_64... "
cargo build --release 2> /dev/null
printf $OK

printf "Copying binary for gnu/linux x86_64..... "
cp target/release/wng release/wng-gnu-linux-x86_64
printf $OK

## For Windows 64 bits

printf "Building project for windows x86_64..... "
cargo build --target x86_64-pc-windows-gnu --release 2> /dev/null
printf $OK

printf "Copying binary for windows x86_64....... "
cp target/x86_64-pc-windows-gnu/release/wng.exe release/wng-windows-x86_64.exe
printf $OK

## For Windows 32 bits

printf "Building project for windows i686....... "
cargo build --target i686-pc-windows-gnu --release 2> /dev/null
printf $OK

printf "Copying binary for windows i686......... "
cp target/i686-pc-windows-gnu/release/wng.exe release/wng-windows-i686.exe
printf $OK



#/usr/bin/env bash

# Script to install or update wng

VERSION_REGEX="\"(.*?)\"" # Regex to get version on crates.io
TEMP_VERSION=$(cargo search wng | egrep -o ${VERSION_REGEX}) # Get wng version on crates.io and apply regex on it
CURRENT_VERSION=$(wng -v) # Get the version of wng installed on the computer
CRATES_IO_VERSION="${TEMP_VERSION%\"}" # Removing quotes at the beggining of the version name
CRATES_IO_VERSION="${CRATES_IO_VERSION#\"}" # Removing quotes at the end of the version name

if [[ ${CRATES_IO_VERSION} == ${CURRENT_VERSION} ]];then
	echo "Already up to date"
else
	echo "Updating..."
	WD=$(pwd)
	cd /bin/
	echo "Downloading version ${CRATES_IO_VERSION} from crates.io..."

	OUTPUT=$(cargo install wng)

	if [[ $? == 0 ]];then
		echo "Successfully downloaded latest version"
	else
		echo "Error in process. Please retry later"
	fi
	cd ${WD}
fi

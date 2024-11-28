#!/bin/bash

if [ -z "$1" ]
then
    echo "Please provide a folder name"
    exit 1
fi

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

folder_path="$SCRIPT_DIR/../builds/$1"

echo "$folder_path"

RUSTFLAGS='-C target-cpu=native' cargo build --release

echo "Creating folder $folder_path"
mkdir -p "$folder_path"

echo "Copying files to $folder_path"
cp target/release/castled_engine "$folder_path"

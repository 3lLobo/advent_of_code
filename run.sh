#!/bin/bash

echo "Whish year would you like?/nYou can choose from:"

for file in *; do
    if [ -d $file ]; then
        echo "   "$file
    fi
done

# year="2021"
while [ -z $year ]; do
    read year
done

if [ $year = "2021" ]; then
    echo "Horrible year!"
else
    echo "Great year!"
fi

echo "Now which day of the advent would you like to play?"
echo "Choose from:"

options=()
for file in $year/*; do
    if [ -d $file ]; then
        # exclude the input_data folder
        if [ $file != $year/input_data ]; then
            options+=($file)
        fi
    fi
done
select opt in "${options[@]}"; do
    case $opt in
    *)
        cargo_path=$opt
        break
        ;;
    esac
done

echo "To play from "$cargo_path "press enter!"

read -s $start

cd $cargo_path
cargo run

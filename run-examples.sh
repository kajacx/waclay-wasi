#!/usr/bin/sh
set -e

# Run from this directory

./build.sh

cd examples

for dir in */; do
    echo "Running example $dir"
    cd $dir
    ./run-example.sh
    cd ..
done
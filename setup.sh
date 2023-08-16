#!/bin/bash

git submodule init
git submodule update

cargo install --force --path .
complete -C ~/.cargo/bin/er er


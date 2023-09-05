#!/bin/sh
# This script is used to set up the environment for the build
# It should be sourced, not run
# It should be sourced by any shell (bash, zsh, etc)

# Detect the OS
OS="unknown"
if [ "$(uname)" = "Darwin" ]; then
    OS="mac"
elif [ "$(expr substr $(uname -s) 1 5)" = "Linux" ]; then
    OS="linux"
elif [ "$(expr substr $(uname -s) 1 10)" = "MINGW32_NT" ]; then
    OS="windows"
fi

# Set up the environment

# If OS is mac, then map to homebrew lib
if [ "$OS" = "mac" ]; then
    export JQ_LIB_DIR=/opt/homebrew/lib
else
    export JQ_LIB_DIR=/usr/lib
fi

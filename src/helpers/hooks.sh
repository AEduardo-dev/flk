#!/usr/bin/env bash

# TODO: migrate base hooks to external lib to obfuscate it from the user
# TODO: check if custom signal could yield same result than exit code

refresh() {
    kill -SIGUSR1 $PPID
    exit 0
}

switch() {
    echo "$1" >/tmp/devshell-action-$$
    kill -SIGUSR1 $PPID
    exit 0
}

#!/usr/bin/env bash
# Helper for working pre-commit in idea.
set -ex

CARGO_BIN_DIR=$HOME/.cargo/bin/

if [[ -d $CARGO_BIN_DIR ]]
then
  export PATH="$CARGO_BIN_DIR:$PATH"
fi

cargo "$@"

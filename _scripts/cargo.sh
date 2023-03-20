#!/usr/bin/env bash
# Helper for working pre-commit in idea.
set -e
CARGO_BIN_DIR=$HOME/.cargo/bin/

if [[ -d $CARGO_BIN_DIR ]]
then
  export PATH="$CARGO_BIN_DIR:$PATH"
fi
if [[ -f './.env' ]]
then
  export $(cat ./.env)
fi
cargo "$@"

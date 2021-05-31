#!/bin/bash

set -eu

ORDER=(types proc-macros utils http-client http-server ws-client ws-server jsonrpsee)

function read_toml () {
        NAME=""
        VERSION=""
        NAME=$(grep "^name" ./Cargo.toml | sed -e 's/.*"\(.*\)"/\1/')
        VERSION=$(grep "^version" ./Cargo.toml | sed -e 's/.*"\(.*\)"/\1/')
}

function check_version() {
  for CRATE_DIR in ${ORDER[@]}; do
          cd $CRATE_DIR > /dev/null
          read_toml
          if [[ "$USER_VERSION" != "$VERSION" ]]; then
            echo "Unexpected version in crate: $NAME:$VERSION; expected: $USER_VERSION"
            exit 1;
          fi
          echo "$NAME:$VERSION"
          cd - > /dev/null
  done
}

function dry_run () {
  for CRATE_DIR in ${ORDER[@]}; do
          cd $CRATE_DIR > /dev/null
          cargo publish --dry-run
          cd - > /dev/null
  done
}

function publish () {
  for CRATE_DIR in ${ORDER[@]}; do
          cd $CRATE_DIR > /dev/null
          read_toml
          read -p ">>>>  Really publish $NAME:$VERSION?. Press [enter] to continue. "
          cargo publish
          cd - > /dev/null
  done
}

if [[ $# -ne 1 ]]; then
        echo "$0 <SEMVER VERSION>"
        exit 1
fi

USER_VERSION="$1"

check_version
dry_run
git fetch --tags
git tag -a -s v$USER_VERSION -m "Version $USER_VERSION"
publish

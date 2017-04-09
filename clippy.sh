#!/bin/sh

mode="$1"

if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
  if [ "$mode" = "install" ]; then
    cargo install clippy
    exit $?
  fi

  if [ "$mode" = "test" ]; then
    cargo clippy --verbose -- -D warnings
    exit $?
  fi
fi

exit 0

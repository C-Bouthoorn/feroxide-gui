#!/bin/bash

mode="$1"

if [ "$TRAVIS_RUST_VERSION" != "nightly" ] && [ "$RUSTUP_NIGHTLY" != "yes" ]; then
  exit 0
fi

CARGO="cargo"

if [ "$RUSTUP_NIGHTLY" = "yes" ]; then
  CARGO="rustup run nightly cargo"
fi


if [ "$mode" = "install" ]; then
  # Test if clippy is already installed (cache)
  if $CARGO clippy -V; then
    exit 0
  fi

  # Clippy could be installed, but failed on the version check, added --force for that case
  $CARGO install clippy --force --verbose
  exit $?
elif [ "$mode" = "test" ]; then
  $CARGO clippy --verbose -- -D warnings
  exit $?
else
  echo "Mode not specified or unknown" >&2
  exit 1
fi

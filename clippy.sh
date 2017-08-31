#!/bin/sh

mode="$1"

if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
  if [ "$mode" = "install" ]; then
    # Test if clippy is already installed (cache)
    $CARGO clippy &> /dev/null
    if [ $? -ne 101 ]; then
      exit 0
    fi

    cargo install clippy
    exit $?
  fi

  if [ "$mode" = "test" ]; then
    cargo clippy --verbose -- -D warnings
    exit $?
  fi
fi

exit 0

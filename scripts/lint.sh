#!/bin/bash

set -eu

cargo_feature() {
    echo "checking with features $1"
    cargo clippy --manifest-path=Cargo.toml --all-targets --features "$1" --no-default-features -- \
    `# Exit with a nonzero code if there are clippy warnings` \
    -Dwarnings
}

if rustup component add clippy; then
  cargo clippy --all-targets --all-features -- \
    `# Exit with a nonzero code if there are clippy warnings` \
    -Dwarnings

  cargo_feature "rustls-tls"
  cargo_feature "native-tls"
fi

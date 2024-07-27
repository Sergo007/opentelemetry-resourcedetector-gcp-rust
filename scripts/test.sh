#!/bin/bash

set -eu

cargo test --all --all-features "$@" -- --test-threads=1


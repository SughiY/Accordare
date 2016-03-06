#!/bin/bash
curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
cargo build
cargo run 

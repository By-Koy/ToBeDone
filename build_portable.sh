#!/bin/zsh

export TBD_NOTES_HOME="."

cargo build --release
cp target/release/ToBeDone ~/Downloads/ToBeDone-Portable
cargo clean
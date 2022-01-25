_default:
  @just --list --unsorted

tool := "dusage"

build:
    cargo build

test: build
    cargo test  --verbose --all
    ./target/debug/dusage
_default:
  @just --list --unsorted

tool := "dusage"

build:
    cargo build

test: build
    cargo test  --verbose --all
    ./target/debug/dusage --debug
    python3 -m unittest discover test/
    df -h
    df

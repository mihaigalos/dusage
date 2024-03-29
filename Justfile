_default:
  @just --list --unsorted

tool := "dusage"

build:
    cargo build

test: build
    cargo test  --verbose --all
    ./target/debug/{{ tool }} --debug
    df -h
    df
    python3 -m unittest discover test/

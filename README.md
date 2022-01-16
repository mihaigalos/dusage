# dusage

[![CI](https://github.com/mihaigalos/dusage/actions/workflows/ci.yaml/badge.svg)](https://github.com/mihaigalos/dusage/actions/workflows/ci.yaml)
[![CD](https://github.com/mihaigalos/dusage/actions/workflows/cd.yaml/badge.svg)](https://github.com/mihaigalos/dusage/actions/workflows/cd.yaml)
[![crates.io](https://img.shields.io/crates/d/dusage.svg)](https://crates.io/crates/dusage)
[![LoC](https://tokei.rs/b1/github/mihaigalos/dusage)](https://github.com/mihaigalos/dusage)

A command line disk usage tool.

![dusage_disks](screenshots/dusage_disks.png)
![dusage_inodes](screenshots/dusage_inodes.png)

### Why?

A better interface for `df`.

### Features

* bargraph with disk and inode usage.
    * background: inodes, foreground: disks.
* grouping of filesystems.
* separate coloring of `/`, `/boot` and `/mnt` for easy spotting.
* [log2ram](https://github.com/azlux/log2ram) filesystem displayed last for easy spotting of log drive usage on Raspberry Pi.
* display of detailed inode usage (similar to `df -i`).

### Installation

##### Building from source

```bash
cargo install dusage
```

##### Using precompiled binaries

Precompiled binaries are available for multiple architectures in [Releases](https://github.com/mihaigalos/dusage/releases).

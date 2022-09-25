# dusage

[![CI](https://github.com/mihaigalos/dusage/actions/workflows/ci.yaml/badge.svg)](https://github.com/mihaigalos/dusage/actions/workflows/ci.yaml) [![CD](https://github.com/mihaigalos/dusage/actions/workflows/cd.yaml/badge.svg)](https://github.com/mihaigalos/dusage/actions/workflows/cd.yaml) [![crates.io](https://img.shields.io/crates/d/dusage.svg)](https://crates.io/crates/dusage) [![LoC](https://tokei.rs/b1/github/mihaigalos/dusage)](https://github.com/mihaigalos/dusage)

A command line disk usage information tool.

![dusage_disks](screenshots/dusage_disks.png)
![dusage_inodes](screenshots/dusage_inodes.png)

## Why?

A better interface for `df`.

## BTW

You might also like [`musage`](https://github.com/mihaigalos/musage).

Both can be i.e. automatically executed upon login via `ssh` to a remote machine by invoking them in the remote's `.bashrc` or `.zshrc`.

## Features

- bargraph with disk and inode usage.
  - background: inodes, foreground: disks.
- grouping of filesystems.
- separate coloring of `/`, `/boot` and `/mnt` for easy spotting.
- [log2ram](https://github.com/azlux/log2ram) filesystem displayed last for
  easy spotting of log drive usage on Raspberry Pi.
- display of detailed inode usage (similar to `df -i`).
- copy-friendly output (via the `--copy-friendly` flag:

```text
Filesystem                 Size     Used    Avail   Use%        Disk / INodes Mounted on
/dev/sdb1                  4.6G   270.1M     4.1G     6% ■■□□□□□□□□□□□□□□□□□□ /boot
/dev/mapper/sdb5_crypt   452.7G   231.6G   198.0G    51% ■■■■■■■■■■■□□□□□□□□□ /
```

## Installation

### Building from source

```bash
cargo install dusage
```

### NetBSD ([Official repositories](https://pkgsrc.se/sysutils/dusage))

```bash
pkgin install dusage
```

Or if you prefer to build it from source:

```bash
cd /usr/pkgsrc/sysutils/dusage
make install
```

### Using precompiled binaries

Precompiled binaries are available for multiple architectures in [Releases](https://github.com/mihaigalos/dusage/releases).

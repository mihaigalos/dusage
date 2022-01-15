# dusage

A command line disk usage tool.

![dusage_disks](screenshots/dusage_disks.png)
![dusage_inodes](screenshots/dusage_inodes.png)

### Why?

A better interface for `df` that is still column-compatible but shows other infos as well.
Features:
* bargraph with disk and inode usage.
* grouping of filesystems.
* separate coloring of `/`, `/boot` and `/mnt` for easy spotting.
* [log2ram](https://github.com/azlux/log2ram) filesystem displayed last for easy spotting of log drive usage on Raspberry Pi.
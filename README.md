# m3u-rs

A simple tool to create m3u playlists. 

## Usage

```
USAGE:
    m3u-rs [FLAGS] <playlist name> <path>

ARGS:
    <playlist name>    name of the playlist
    <path>             the path to the directory with the music files

FLAGS:
    -a               append new files to existing playlist
    -h, --help       Prints help information
    -r               search recursively for music files
    -V, --version    Prints version information
```

Yes I could've used a oneliner to archive the same:

```sh
find . -type f \( -name '*.mp3' -o -name '*.flac' -o -name '*.loss' -o -name '*.aiff' -o -name '*.aif' \) -printf "%P\n" > playlist.m3u
```

But where is the fun in simple solutions for simple problems.

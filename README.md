# m3u-rs

A simple tool to create m3u playlists. Yes I could've used a oneliner to archive the same:

```sh
find . -type f \( -name '*.mp3' -o -name '*.flac' -o -name '*.loss' -o -name '*.aiff' -o -name '*.aif' \) -printf "%P\n" > playlist.m3u
```

But where is the fun in simple solutions for simple problems.

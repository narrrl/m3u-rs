use std::{
    env,
    fs::{canonicalize, read_dir, File, OpenOptions},
    io::{prelude::*, Error, ErrorKind, Result},
    path::PathBuf,
};

use clap::App;

fn main() -> Result<()> {
    let matches = App::new("m3u-rs")
        .version("0.1")
        .author("Nils Pukropp")
        .arg("-r... 'search recursively'")
        .arg("-a... 'append new files to existing playlist'")
        .arg("<playlist name> 'name of the playlist'")
        .arg("<path>... 'the path or multiple paths to directories with music files or just single music files'")
        .get_matches();

    let mut current_dir = env::current_dir()?;

    let paths: Vec<PathBuf> = match matches.values_of("path") {
        Some(values) => {
            let mut paths = Vec::new();
            for val in values {
                paths.push(PathBuf::from(val));
            }
            paths
        }
        None => vec![env::current_dir()?],
    };

    let recursively = matches.is_present("r");

    let mut can_paths = Vec::new();
    for path in paths {
        let path: PathBuf = match canonicalize(&path) {
            Ok(path) => path,
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Invalid path {:#?}", &path),
                ));
            }
        };
        can_paths.push(path);
    }

    let mut songs = Vec::new();
    for path in can_paths {
        for song in get_music_files(&path, recursively)? {
            songs.push(song);
        }
    }

    let playlist_name = matches
        .value_of("playlist name")
        .expect("Expected a playlist name");

    current_dir.push(format!("{}.m3u", playlist_name));

    let mut playlist_file: File;

    if matches.is_present("a") {
        playlist_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&current_dir)?;
    } else {
        playlist_file = File::create(&current_dir)?;
    }

    for song in songs {
        let path_to_song = match song.to_str() {
            Some(str) => str,
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Couldn't read path of {:#?}", song),
                ));
            }
        };
        playlist_file.write_all(path_to_song.as_bytes())?;
        playlist_file.write_all(b"\n")?;
    }

    Ok(())
}

fn get_music_files(path: &PathBuf, recursively: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if !path.is_dir() && is_music_file(&path) {
        files.push(path.clone());
        return Ok(files);
    }

    for f in read_dir(path)? {
        let fpath = f?.path();
        if fpath.is_dir() && recursively {
            for f2 in get_music_files(&fpath, recursively)? {
                files.push(f2);
            }
        } else if is_music_file(&fpath) {
            files.push(fpath);
        }
    }

    Ok(files)
}

fn is_music_file(path: &PathBuf) -> bool {
    let ext = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => ext,
            None => return false,
        },
        None => return false,
    };

    match ext {
        "flac" | "mp3" | "wav" | "ogg" | "m4a" => true,
        _ => false,
    }
}

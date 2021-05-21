use std::{
    env,
    fs::{canonicalize, read_dir},
    io::Result,
    path::PathBuf,
};

use clap::App;

fn main() -> Result<()> {
    let matches = App::new("m3u-rs")
        .version("0.1")
        .author("Nils Pukropp")
        .arg("-r... 'Search for music files recursively'")
        .arg("<path> 'Sets the path to directory with the music files'")
        .get_matches();

    let mut path = match matches.value_of("path") {
        Some(path) => PathBuf::from(path),
        None => env::current_dir()?,
    };

    let recursively = matches.is_present("r");

    path = canonicalize(path)?;

    for f in get_music_files(path, recursively)? {
        println!("{:#?}", f);
    }

    Ok(())
}

fn get_music_files(path: PathBuf, recursively: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for f in read_dir(path)? {
        let fpath = f?.path();
        if fpath.is_dir() && recursively {
            for f2 in get_music_files(fpath, recursively)? {
                files.push(f2);
            }
        } else {
            files.push(fpath);
        }
    }

    Ok(files)
}

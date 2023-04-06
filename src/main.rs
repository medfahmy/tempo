use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    process,
};

fn main() -> Result<(), ()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    let path = args.first()
        .map(|path| PathBuf::from(path))
        .unwrap_or(std::env::current_dir().map_err(|err| {
            eprintln!("error accessing current working directory; error={}", err);
            process::exit(1);
        })?);

    if path.is_file() {
        eprintln!("path='{}' is not a directory", path.display());
        process::exit(1);
    }

    let dirs = walk_dir(&path)?;
    dbg!(dirs);

    Ok(())
}

fn walk_dir(path: &Path) -> Result<Vec<PathBuf>, ()> {
    let entries: Vec<_> = read_dir(path)
        .map_err(|err| {
            eprintln!("error reading directory contents; error={}", err);
            process::exit(1);
        })?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .map_err(|err| {
            eprintln!("error reading directory children; error={}", err);
            process::exit(1);
        })?;

    let dirs = entries.into_iter().filter(|path| path.is_dir()).collect();

    // TODO: walk children directories

    Ok(dirs)
}

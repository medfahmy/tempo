use std::{
    path::PathBuf,
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
        eprintln!("'{}' is not a directory", path.display());
        process::exit(1);
    }

    Ok(())
}

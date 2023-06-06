use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// A tool to hardcopy directories
#[derive(Parser, Debug)]
struct Args {
    /// The source file/directory
    source: PathBuf,
    /// The target file/directory (defaults to .)
    target: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let source_dir = args.source;
    let target_dir = args.target.unwrap_or(".".into());

    if !target_dir.exists() {
        fs::create_dir(&target_dir).expect("Failed to create target directory")
    }

    copy_dir(&source_dir, &target_dir)
        .map_err(|e| println!("Linking failed: {}", e))
        .unwrap();
}

fn copy_dir(source: &PathBuf, target: &PathBuf) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(source)? {
        let entry = entry.unwrap();
        let target = target.join(entry.file_name());

        if entry.path().is_dir() {
            fs::create_dir(&target)?;
            copy_dir(&entry.path(), &target)?;
        } else if entry.path().is_file() {
            println!("linking {} -> {}", source.display(), target.display());
            fs::hard_link(entry.path(), target)?;
        } else {
            println!("Unkown file, skipping: {}", entry.path().display());
        }
    }
    Ok(())
}

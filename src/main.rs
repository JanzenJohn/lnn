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
    /// Skip unknown files
    #[arg(short, long)]
    skip_unknown: bool,
}

fn main() {
    let args = Args::parse();

    let source_dir = args.source;
    let target_dir = args.target.unwrap_or(".".into());

    if !source_dir.is_dir() {
        eprintln!("Could not read source directory {}", source_dir.display());
        return;
    }

    if !target_dir.exists() {
        fs::create_dir(&target_dir).expect("Failed to create target directory")
    }

    copy_dir(&source_dir, &target_dir, args.skip_unknown)
        .map_err(|e| println!("Linking failed: {}", e))
        .unwrap();
}

fn copy_dir(source: &PathBuf, target: &PathBuf, skip_unknown: bool) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = target.join(entry.file_name());

        if entry.path().is_dir() {
            fs::create_dir(&target)?;
            copy_dir(&entry.path(), &target, skip_unknown)?;
        } else if entry.path().is_file() {
            println!("linking {} -> {}", source.display(), target.display());
            fs::hard_link(entry.path(), target)?;
        } else if skip_unknown {
            println!("Unkown file, skipping: {}", entry.path().display());
        } else {
            return Err(std::io::ErrorKind::Other.into());
        }
    }
    Ok(())
}

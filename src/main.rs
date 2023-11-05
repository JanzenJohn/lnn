use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// A tool to hardcopy directories{n}{n}
/// Copyright (C) 2023  John Janzen{n}
/// This program is free software: you can redistribute it and/or modify{n}
/// it under the terms of the GNU General Public License as published by{n}
/// the Free Software Foundation{n}
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The source file/directory
    source: PathBuf,
    /// The target file/directory (defaults to .)
    target: Option<PathBuf>,
    /// Skip unknown files
    #[arg(short, long)]
    skip_unknown: bool,
    /// Skip existing files
    #[arg(short='i', long)]
    skip_existing: bool,
}

fn main() {
    let args = Args::parse();

    let source_dir = args.source;
    let target_dir = args.target.unwrap_or(".".into());

    if !source_dir.is_dir() {
        eprintln!(
            "[ERROR] Could not read source directory {}",
            source_dir.display()
        );
        std::process::exit(1);
    }

    if !target_dir.exists() {
        fs::create_dir(&target_dir).expect("[ERROR] Failed to create target directory")
    }

    match copy_dir(&source_dir, &target_dir, args.skip_unknown, args.skip_existing) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!(
                "[ERROR] Failed to link {} to {}:\n{:#?}",
                &source_dir.display(),
                &target_dir.display(),
                e
            );
            std::process::exit(1);
        }
    };
}

fn copy_dir(source: &PathBuf, target: &PathBuf, skip_unknown: bool, skip_existing: bool) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = target.join(entry.file_name());

        if entry.path().is_dir() {
            fs::create_dir(&target)?;
            copy_dir(&entry.path(), &target, skip_unknown, skip_existing)?;
        } else if entry.path().is_file() && entry.path().exists() && skip_existing {
            println!("skipping {} already exists", target.display());
        } else if entry.path().is_file() {
            println!("linking {} -> {}", source.display(), target.display());
            fs::hard_link(entry.path(), target)?;
        } else if skip_unknown {
            println!(
                "[WARNING] Unkown file, skipping: {}",
                entry.path().display()
            );
        } else {
            eprintln!("[ERROR] Uknown file: {}", entry.path().display());
            return Err(std::io::ErrorKind::Unsupported.into());
        }
    }
    Ok(())
}

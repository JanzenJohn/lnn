use std::env;
use std::path;
use std::fs;

fn main() {
    let default_str = ".".to_string();
    let args = env::args().collect::<Vec<String>>();
    let source = args.get(1).expect("No source provided");
    let target = args.get(2).unwrap_or(&default_str);

    let source_dir = std::path::Path::new(&source);
    let target_dir = std::path::Path::new(&target);
    if !source_dir.exists() {
        eprintln!("Source directory does not exist");
        return;
    }
    if !source_dir.is_dir() {
        eprintln!("Source is not a directory");
        return;
    }

    if !target_dir.exists() {
        std::fs::create_dir(target_dir).expect("someone created the target directory before us");
    }
    copy_dir(source_dir, target_dir, 0).unwrap();



}

fn copy_dir(source: &path::Path, target: &path::Path, depth: i32) -> Result<(), String> {
    if depth > 4 {
        return Err(format!("too deep"));
    }
    println!("copying dir {} to {}", source.display(), target.display());
    for (_i, entry) in fs::read_dir(source).unwrap().enumerate() {
        let entry_path_buf = entry.unwrap().path();
        let entry_path = entry_path_buf.as_path();
        let target_path_buf = target.join(entry_path.file_name().unwrap());
        let target_path = target_path_buf.as_path();
        if entry_path.is_dir() {
            fs::create_dir(target_path).expect(format!("failed to create directory: {}", target_path.display()).as_str());
            copy_dir(entry_path, target_path, depth + 1)?;
        }
        else if entry_path.is_file() {
            fs::hard_link(entry_path, target_path).expect(format!("failed to hard link file: {}", entry_path.display()).as_str());
        } else {
            return Err(format!("unknown file type: {}", entry_path.display()));
        }
    }
    Ok(())
}
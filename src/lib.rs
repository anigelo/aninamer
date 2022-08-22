use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
use lazy_static::lazy_static;

const ALLOWED_EXTENSIONS: [&str; 4] = ["mkv", "avi", "mp4", "M4V"];

pub fn rename(path: PathBuf, can_rename: bool) {
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|file| file.ok()) {
        let file = entry.into_path();
        if is_allowed_extension(&file) {
            if let Some(new_name) = matches_for_renaming(&file) {
                println!("{:?} will be renamed to {}", file, new_name);
                let new_name = format!("{}.{}", new_name,
                                       file.extension().unwrap().to_string_lossy());
                let new_name = file.with_file_name(new_name);
                if can_rename {
                    if let Err(e) = std::fs::rename(&file, new_name) {
                        println!("Error renaming {}", e);
                    }
                }
            } else {
                println!("{:?} is OK", file);
            }
        }
    }
}

fn is_allowed_extension(file: &PathBuf) -> bool {
    if let Some(extension) = file.extension() {
        ALLOWED_EXTENSIONS.contains(&&*extension.to_string_lossy())
    } else {
        false
    }
}

fn matches_for_renaming(file: &PathBuf) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"( |x)(\d{2})").unwrap();
    }

    if let Some(file_name) = file.file_name() {
        if let Some(captures) = RE.captures(&file_name.to_string_lossy()) {
            if let Some(group) = captures.get(2) {
                return Some(group.as_str().to_string());
            }
        }
    }

    None
}
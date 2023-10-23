use std::fs;
use std::path::Path;

pub fn scan_directory_recursive<F>(directory_path: &str, callback: &F)
where
    F: Fn(&Path),
{
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "html" {
                            callback(&path);
                        }
                    }
                } else if path.is_dir() {
                    scan_directory_recursive(path.to_str().unwrap(), callback);
                }
            }
        }
    } else {
        eprintln!("Failed to read the directory: {}", directory_path);
    }
}

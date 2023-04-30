mod formatt;

use formatt::format_model_file;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file or directory path as an argument");
    }

    let path = PathBuf::from(&args[1]);

    let metadata = fs::metadata(&path)?;

    if metadata.is_file() {
        process_file(&path)?;
    } else if metadata.is_dir() {
        process_directory(&path)?;
    } else {
        panic!("Invalid path specified");
    }

    Ok(())
}

fn process_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let string = format_model_file(file_path.to_str().unwrap())?;
    dump_string_to_file(string, file_path)?;
    Ok(())
}

fn process_directory(dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if path.file_name() == Some("model".as_ref()) {
                process_file(&path)?;
            }
        } else {
            process_directory(&path)?;
        }
    }

    Ok(())
}

fn dump_string_to_file(contents: String, file_path: &Path) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

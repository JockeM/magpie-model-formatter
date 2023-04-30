mod format;

use format::format_model_file;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file or directory path as an argument");
    }

    let path = PathBuf::from(&args[1]);

    let metadata = fs::metadata(&path)?;

    let mut num_files_processed = 0;
    let start_time = Instant::now();

    if metadata.is_file() {
        if path.file_name() != Some("model".as_ref()) {
            panic!("Invalid file type. Only model files with the name 'model' are allowed.");
        }
        process_file(&path, &mut num_files_processed)?;
    } else if metadata.is_dir() {
        process_directory(&path, &mut num_files_processed)?;
    } else {
        panic!("Invalid path specified");
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "Processed {} file(s) in {} ms",
        num_files_processed,
        elapsed_time.as_millis()
    );

    Ok(())
}

fn process_file(
    file_path: &Path,
    num_files_processed: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let string = format_model_file(contents.as_ref())?;
    write_to_file(string.as_ref(), file_path)?;
    *num_files_processed += 1;
    println!("Formatted file: {:?}", file_path);

    Ok(())
}

fn process_directory(
    dir_path: &Path,
    num_files_processed: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if path.file_name() == Some("model".as_ref()) {
                process_file(&path, num_files_processed)?;
            }
        } else {
            process_directory(&path, num_files_processed)?;
        }
    }

    Ok(())
}

fn write_to_file(contents: &str, file_path: &Path) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

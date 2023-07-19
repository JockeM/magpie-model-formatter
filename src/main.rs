mod format;

use format::format_model_file;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file or directory path as an argument");
    }

    let path = PathBuf::from(&args[1]);

    let metadata = fs::metadata(&path)?;

    let start_time = Instant::now();

    if metadata.is_file() {
        if path.file_name() != Some("model".as_ref()) {
            panic!("Invalid file type. Only model files with the name 'model' are allowed.");
        }
        process_file(&path)?;
    } else if metadata.is_dir() {
        process_directory(&path)?;
    } else {
        panic!("Invalid path specified");
    }

    let elapsed_time = start_time.elapsed();

    println!("Processed files in {} ms", elapsed_time.as_millis());

    Ok(())
}

fn process_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut reader = BufReader::new(&file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    let string = format_model_file(buffer.as_ref())?;

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let mut writer = BufWriter::new(&file);
    writer.write_all(string.as_bytes())?;

    println!("Formatted file: {}", file_path.display());

    Ok(())
}

fn process_directory(dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir_path)?;

    entries.into_iter().par_bridge().for_each(|entry| {
        let path = entry.unwrap().path();
        if path.is_file() {
            if path.file_name() == Some("model".as_ref()) {
                process_file(&path).unwrap();
            }
        } else {
            process_directory(&path).unwrap();
        }
    });

    Ok(())
}

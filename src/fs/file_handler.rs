use core::panic;
use std::fmt::Display;
use std::fs::File;

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
pub struct FileHandler;

impl FileHandler {
    pub fn read<T>(file_path: T) -> Vec<String>
    where
        T: Into<String> + AsRef<Path> + Display,
    {
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(err) => panic!(
                "Unable to open file: {}.\n Reason: {}",
                file_path,
                err.to_string()
            ),
        };

        BufReader::new(file)
            .lines()
            .map(|s| match s {
                Ok(file_content) => file_content,
                Err(err) => panic!("Failed to read file.\n Reason: {}", err.to_string()),
            })
            .collect()
    }

    pub fn write<T>(file_path: T, buffer: Vec<String>)
    where
        T: Into<String> + AsRef<Path> + Display,
    {
        let mut file = match File::create(&file_path) {
            Ok(file) => file,
            Err(err) => panic!(
                "Failed to create output file: {}.\n Reason: {}",
                file_path,
                err.to_string()
            ),
        };

        for line in &buffer {
            file.write_all(line.as_bytes()).expect(
                format!("Failed to write line: {} on file: {}.\n", line, file_path).as_str(),
            )
        }
    }
}

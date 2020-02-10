use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result as IOResult};
use std::path::Path;

pub fn create_or_open<P: AsRef<Path>>(path: P) -> IOResult<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(path)
}

pub fn create_or_open_overwrite<P: AsRef<Path>>(path: P) -> IOResult<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
}


pub fn num_lines<P: AsRef<Path>>(path: P) -> usize {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    lines.count()
}
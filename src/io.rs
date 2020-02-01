use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result as IOResult};
use std::path::Path;

use rand::{self, Rng, distributions::{Distribution, Uniform}};

pub fn create_or_open<P: AsRef<Path>>(path: P) -> IOResult<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(path)
}

pub fn num_lines<P: AsRef<Path>>(path: P) -> usize {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    lines.count()
}

pub fn random_line(path: &str) -> String {
    let num_lines = num_lines(Path::new(&path));
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(1, num_lines);
    let line_num = range.sample(&mut rng);
    let file = File::open(Path::new(&path)).unwrap();
    let mut lines = BufReader::new(file).lines();
    for _ in 0..line_num-1 {
        lines.next();
    }
    lines.next().unwrap().unwrap()
}
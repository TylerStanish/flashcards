use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Result as IOResult, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use chrono::{DateTime, Utc};

static CARDS_FILE: &'static str = "cards.txt";

fn create_or_open<P: AsRef<Path>>(path: P) -> IOResult<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(path)
}

pub struct Card {
    word: String,
    translation: String,
    last_practiced: DateTime<Utc>,
}

impl Card {
    pub fn new(word: String, translation: String) -> Self {
        Card {
            word,
            translation,
            last_practiced: Utc::now(),
        }
    }

    pub fn persist(&self) {
        let mut file = create_or_open(CARDS_FILE).unwrap();
        writeln!(file, "some card").expect("Could not write to card file");
    }
}

pub fn list_cards() {
    let file = create_or_open(CARDS_FILE).expect("Could not create a flashcards file");
    let reader = BufReader::new(file);

    let pager = env::var("PAGER").unwrap_or(
        if cfg!(target_os = "windows") {
            String::from("more")
        } else {
            String::from("less")
        }
    );
    let mut res = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", &pager]).stdin(Stdio::piped()).stdout(Stdio::inherit()).spawn().unwrap()
    } else {
        Command::new("sh").args(&["-c", &pager]).stdin(Stdio::piped()).stdout(Stdio::inherit()).spawn().unwrap()
    };
    let stdin = res.stdin.as_mut().unwrap();
    for line in reader.lines() {
        let unwrapped = match line {
            Ok(line) => line,
            Err(err) => continue,
        };
        stdin.write(&[unwrapped.as_bytes(), "\n".as_bytes()].concat()).unwrap();
    }
    res.wait().unwrap();
}

#[cfg(test)]
mod tests {
    use tempfile;
    use std::fs::File;
    use uuid::Uuid;

    fn new_temp_dir(loc: &str) -> tempfile::TempDir {
        tempfile::tempdir_in(loc).unwrap()
    }

    fn new_temp_file_in(dir: &str) -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::new_in(dir).unwrap()
    }

    fn new_temp_file(filename: &str) -> File {
        tempfile::NamedTempFile::new().unwrap().persist(filename).unwrap()
    }

    #[test]
    fn creates_cards_file_if_not_exists() {
        let filename = Uuid::new_v4().to_string();
        let directory = new_temp_dir(".");
        let res = super::create_or_open(directory.path().join(filename));
        res.unwrap();
    }

    #[test]
    fn creates_cards_file_if_exists() {
        let file = new_temp_file_in(".");
        let res = super::create_or_open(file.path());
        res.unwrap();
    }
}
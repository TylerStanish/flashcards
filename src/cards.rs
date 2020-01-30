use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Error as IOError, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};


fn create_or_open<P: AsRef<Path>>(path: P) -> Result<File, IOError> {
    File::open(&path).or(File::create(&path).and(File::open(&path)))
}


pub fn list_cards() {
    let file = create_or_open("cards.txt").expect("Could not create a flashcards file");
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
    use uuid::Uuid;

    #[test]
    fn creates_cards_file_if_not_exists() {
        let filename = Uuid::new_v4().to_string();
        let directory = tempfile::tempdir_in(".").unwrap();
        let res = super::create_or_open(directory.path().join(filename));
        if !res.is_ok() {
            res.unwrap();
        }
    }

    #[test]
    fn creates_cards_file_if_exists() {
        let file = tempfile::NamedTempFile::new_in(".").unwrap();
        let res = super::create_or_open(file.path());
        if !res.is_ok() {
            res.unwrap();
        }
    }
}
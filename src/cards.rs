use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};


pub fn list_cards() {
    let mut file = match File::open("cards.txt") {
        Ok(file) => file,
        Err(err) => match File::create("cards.txt") {
            // for some reason although File::create adds write permissions,
            // we have to open the file after creating it or else we get a bad file descriptor or permission denied
            Ok(file) => File::open("cards.txt").unwrap(),
            Err(err) => {
                panic!("Could not create a flashcards file");
            }
        },
    };
    let reader = BufReader::new(file);

    let pager = match env::var("PAGER") {
        Ok(pager) => pager,
        Err(err) => {
            if cfg!(target_os = "windows") {
                String::from("more")
            } else {
                String::from("less")
            }
        },
    };
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
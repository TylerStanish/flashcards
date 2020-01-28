use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};


pub fn list_cards() {
    let file = match File::open("cards.txt") {
        Ok(file) => file,
        Err(err) => match File::create("cards.txt") {
            Ok(file) => file,
            Err(err) => {
                panic!("Could not create a flashcards file");
            }
        },
    };
    let reader = BufReader::new(file);

    let mut res = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "more"]).stdin(Stdio::piped()).stdout(Stdio::inherit()).spawn().unwrap()
    } else {
        Command::new("sh").args(&["-c", "less"]).stdin(Stdio::piped()).stdout(Stdio::inherit()).spawn().unwrap()
    };
    let mut stdin = res.stdin.as_mut().unwrap();
    // stdin.write(reader.bytes().into()).unwrap();
    for line in reader.lines() {
        stdin.write(&[line.unwrap().as_bytes(), "\n".as_bytes()].concat()).unwrap();
        stdin.flush();
    }
    res.wait().unwrap();
}
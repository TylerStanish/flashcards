use std::env;
use std::fs::{File};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use rand::{self, Rng, distributions::{Distribution, Uniform}};
use crate::io::{create_or_open, create_or_open_overwrite};

static CARDS_FILE: &'static str = "cards.txt";

pub struct Card {
    word: String,
    translation: String,
    last_practiced: DateTime<Utc>,
}

impl Card {
    pub fn create(word: String, translation: String) -> Self {
        Card::new(word, translation, Utc::now())
    }

    pub fn new(word: String, translation: String, last_practiced: DateTime<Utc>) -> Self {
        Card {
            word,
            translation,
            last_practiced: last_practiced,
        }
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let word = split.next().unwrap();
        let translation = split.next().unwrap();
        let date_str = split.next().unwrap();
        let last_practiced = DateTime::parse_from_rfc3339(date_str).unwrap().with_timezone(&Utc);
        Ok(Card::new(word.to_string(), translation.to_string(), last_practiced))
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.word, self.translation, self.last_practiced.to_rfc3339())
    }
}

pub fn get_cards() -> Vec<Card> {
    let file = create_or_open(CARDS_FILE).expect("Could not create or open the flashcards file");
    let reader = BufReader::new(file);
    let mut cards = Vec::new();
    for line in reader.lines() {
        cards.push(Card::from_str(&line.unwrap()).unwrap());
    }
    cards
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

pub fn create_card(cards: &mut Vec<Card>) {
    print!("Word: ");
    // according to this: https://internals.rust-lang.org/t/create-a-flushing-version-of-print/9870/12
    // we don't need to lock() explicitly
    io::stdout().flush().unwrap();
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();

    let mut translation = String::new();
    print!("Translation: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut translation).unwrap();
    cards.push(Card::create(word.trim().to_string(), translation.trim().to_string()));
    save_cards(cards);
}

fn save_cards(cards: &Vec<Card>) {
    let mut writer = BufWriter::new(create_or_open_overwrite(CARDS_FILE).unwrap());
    for card in cards {
        writer.write(card.to_string().as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}

pub fn random_index<T>(items: &[T]) -> usize {
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(0, items.len() - 1);
    let index = range.sample(&mut rng);
    index
}

pub fn practice(cards: &mut Vec<Card>) {
    // get random word
    let index = random_index(cards);
    let card = cards.iter().nth(index).unwrap();
    println!("{}", card.translation);
    print!("Enter word for this translation: ");
    io::stdout().flush().unwrap();
    let mut attempt = String::new();
    io::stdin().read_line(&mut attempt).unwrap();
    if !attempt.trim().eq_ignore_ascii_case(&card.word) {
        println!("Incorrect.");
    } else {
        println!("Correct!");
        // TODO update last_practiced in file
        cards[index].last_practiced = Utc::now();
        save_cards(cards);
    }
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
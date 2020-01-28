use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::validate::ValidationResult::Valid;
use rustyline::{CompletionType, Context};
use rustyline::{Editor, Helper};
use std::borrow::Cow;

use crate::cards::list_cards;

static ACTIONS: &[&str] = &["ls", "pr"];
static PROMPT: &str = "> ";

fn byebye() {
    println!("bye bye.");
}

struct OurHelper {
    _highlighter: MatchingBracketHighlighter,
}
impl OurHelper {
    pub fn new() -> Self {
        OurHelper {
            _highlighter: MatchingBracketHighlighter::new(),
        }
    }
}
impl Completer for OurHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        Ok((42, vec![]))
    }
    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {}
}
impl Validator for OurHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        // TODO validate or skip
        Ok(Valid(Some("".to_string())))
    }
    fn validate_while_typing(&self) -> bool {
        true
    }
}
impl Hinter for OurHelper {
    fn hint(&self, line: &str, pos: usize, ctx: &Context) -> Option<String> {
        // TODO add hints
        None
        // Some("hi".to_string())
    }
}
impl Highlighter for OurHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self._highlighter.highlight(line, pos)
    }
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        self._highlighter.highlight_prompt(PROMPT, default)
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        self._highlighter.highlight_hint(hint)
    }
    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str,
        completion: CompletionType,
    ) -> Cow<'c, str> {
        self._highlighter.highlight_candidate(candidate, completion)
    }
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self._highlighter.highlight_char(line, pos)
    }
}
impl Helper for OurHelper {}

fn repl<T: rustyline::Helper>(editor: &mut Editor<T>) {
    loop {
        let line = editor.readline(PROMPT);
        match line {
            Ok(readline) => match readline.as_ref() {
                "exit" => {
                    byebye();
                    break;
                },
                "ls" => list_cards(),
                // TODO 
                "practice" => panic!("Do this"),
                // TODO 
                "save" => panic!("Do this"),
                _ => (),
            },
            Err(ReadlineError::Eof) => {
                println!("{}exit", PROMPT);
                byebye();
                break;
            }
            Err(_) => (),
        };
    }
}

pub fn start() {
    let mut editor = Editor::<OurHelper>::new();
    editor.set_helper(Some(OurHelper::new()));
    // TODO don't unwrap, instead create the history file if it doesn't exist
    editor.load_history("history.txt").unwrap();
    repl(&mut editor);
}
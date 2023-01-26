//! Completion module for `rustyline`.
//!

//use anyhow::{anyhow, Result};
//use log::{debug, error};

use rustyline::completion::{Candidate, Completer};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::MatchingBracketValidator;
use rustyline::Context;
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};

#[derive(Helper, Completer, Hinter, Validator)]
pub struct MyHelper {
    #[rustyline(Completer)]
    completer: KeywordCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

pub struct KeywordCompleter {}

impl Completer for KeywordCompleter {
    type Candidate = ();

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        todo!()
    }
    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {
        todo!()
    }
}

impl Candidate for KeywordCompleter {
    fn display(&self) -> &str {
        todo!()
    }
    fn replacement(&self) -> &str {
        todo!()
    }
}

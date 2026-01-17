use std::collections::HashMap;
use rustyline::completion::{Completer, Pair};
use rustyline::Context;
use rustyline::error::ReadlineError;
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

use crate::engine::Command;

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct DiceCompleter {
    pub commands: HashMap<String, Command>,
}

impl Completer for DiceCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let (start, word) = if let Some(last_space) = line[..pos].rfind(' ') {
            (last_space + 1, &line[last_space + 1..pos])
        } else {
            (0, &line[..pos])
        };

        let matches: Vec<Pair> = self.commands.keys()
            .filter(|name| name.starts_with(word))
            .map(|name| Pair {
                display: name.clone(),
                replacement: name.clone(),
            })
            .collect();

        Ok((start, matches))
    }
}

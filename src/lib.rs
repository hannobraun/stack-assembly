//! # The interpreter for the StackAssembly programming language

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::collections::VecDeque;

#[cfg(test)]
mod tests;

/// # The ongoing evaluation of a script
#[derive(Debug)]
pub struct Eval {
    /// # The remaining tokens that we haven't evaluated yet
    pub tokens: VecDeque<String>,

    /// # The operand stack
    pub stack: Vec<u32>,

    /// # The active effect, if one has triggered
    pub effect: Option<Effect>,
}

impl Eval {
    /// # Start evaluating the provided script
    ///
    /// Returns an `Eval` instance that is ready. To evaluate any tokens in the
    /// provided script, you still have to explicitly call [`Eval::step`] or
    /// [`Eval::run`].
    pub fn start(script: &str) -> Self {
        Self {
            tokens: script
                .split_whitespace()
                .map(|token| token.to_owned())
                .collect(),
            stack: Vec::new(),
            effect: None,
        }
    }

    /// # Advance the evaluation by one step
    pub fn step(&mut self) -> bool {
        if self.effect.is_some() {
            return false;
        }

        let Some(token) = self.tokens.pop_front() else {
            return false;
        };

        if let Ok(value) = token.parse::<i32>() {
            let value = u32::from_le_bytes(value.to_le_bytes());
            self.stack.push(value);
        } else if token == "yield" {
            self.effect = Some(Effect::Yield);
        } else {
            self.effect = Some(Effect::UnknownIdentifier);
        }

        true
    }

    /// # Advance the evaluation until it triggers an effect or completes
    pub fn run(&mut self) {
        while self.step() {}
    }
}

/// # An effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

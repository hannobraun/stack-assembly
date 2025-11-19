//! # The interpreter for the StackAssembly programming language

#![deny(missing_docs)]

use std::collections::VecDeque;

#[cfg(test)]
mod tests;

/// # The ongoing evaluation of a script
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
        let Some(token) = self.tokens.pop_front() else {
            return false;
        };

        if let Ok(value) = token.parse::<i32>() {
            let value = u32::from_le_bytes(value.to_le_bytes());
            self.stack.push(value);
        } else {
            self.effect = Some(Effect::IntegerOverflow);
        }

        true
    }

    /// # Advance the evaluation until it completes
    pub fn run(&mut self) {
        while self.step() {}
    }
}

/// # An effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # The evaluation of an integer operator triggered an overflow
    IntegerOverflow,
}

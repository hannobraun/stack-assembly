//! # The interpreter for the StackAssembly programming language

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::collections::VecDeque;

mod stack;

#[cfg(test)]
mod tests;

pub use self::stack::Stack;

/// # The ongoing evaluation of a script
#[derive(Debug)]
pub struct Eval {
    /// # The remaining tokens that we haven't evaluated yet
    pub tokens: VecDeque<String>,

    /// # The active effect, if one has triggered
    pub effect: Option<Effect>,

    /// # The stack
    pub stack: Stack,
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
            effect: None,
            stack: Stack { values: Vec::new() },
        }
    }

    /// # Advance the evaluation by one step
    pub fn step(&mut self) -> bool {
        if self.effect.is_some() {
            return false;
        }

        if let Err(effect) = self.evaluate_token() {
            self.effect = Some(effect);
            return false;
        }

        true
    }

    fn evaluate_token(&mut self) -> Result<(), Effect> {
        let Some(token) = self.tokens.pop_front() else {
            return Err(Effect::OutOfTokens);
        };

        if let Ok(value) = token.parse::<i32>() {
            self.stack.push(value);
        } else if token == "*" {
            let b = self.stack.pop()?;
            let a = self.stack.pop()?;

            self.stack.push(a.wrapping_mul(b));
        } else if token == "+" {
            let b = self.stack.pop()?;
            let a = self.stack.pop()?;

            self.stack.push(a.wrapping_add(b));
        } else if token == "-" {
            let b = self.stack.pop()?;
            let a = self.stack.pop()?;

            self.stack.push(a.wrapping_sub(b));
        } else if token == "/" {
            let b = self.stack.pop()?;
            let a = self.stack.pop()?;

            let [a, b] =
                [a, b].map(|value| i32::from_le_bytes(value.to_le_bytes()));

            if b == 0 {
                return Err(Effect::DivisionByZero);
            }
            if a == i32::MIN && b == -1 {
                return Err(Effect::IntegerOverflow);
            }

            let quotient = a / b;
            let remainder = a % b;

            self.stack.push(quotient);
            self.stack.push(remainder);
        } else if token == "yield" {
            return Err(Effect::Yield);
        } else {
            return Err(Effect::UnknownIdentifier);
        }

        Ok(())
    }

    /// # Advance the evaluation until it triggers an effect or completes
    pub fn run(&mut self) {
        while self.step() {}
    }
}

/// # An effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to divide by zero
    DivisionByZero,

    /// # Evaluating an operation resulted in integer overflow
    IntegerOverflow,

    /// # The evaluation ran out of tokens to evaluate
    OutOfTokens,

    /// # Tried popping a value from an empty stack
    StackUnderflow,

    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

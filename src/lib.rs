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

        let Some(token) = self.tokens.pop_front() else {
            return false;
        };

        if let Err(effect) = evaluate_token(&token, &mut self.stack) {
            self.effect = Some(effect);
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
    /// # Tried popping a value from an empty stack
    StackUnderflow,

    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

fn evaluate_token(token: &str, stack: &mut Stack) -> Result<(), Effect> {
    if let Ok(value) = token.parse::<i32>() {
        let value = u32::from_le_bytes(value.to_le_bytes());
        stack.values.push(value);
    } else if token == "*" {
        let b = stack.pop()?;
        let a = stack.pop()?;

        stack.values.push(a.wrapping_mul(b));
    } else if token == "+" {
        let b = stack.pop()?;
        let a = stack.pop()?;

        stack.values.push(a.wrapping_add(b));
    } else if token == "-" {
        let b = stack.pop()?;
        let a = stack.pop()?;

        stack.values.push(a.wrapping_sub(b));
    } else if token == "/" {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let [a, b] =
            [a, b].map(|value| i32::from_le_bytes(value.to_le_bytes()));

        let quotient = a / b;
        let remainder = a % b;

        let [quotient, remainder] = [quotient, remainder]
            .map(|value| u32::from_le_bytes(value.to_le_bytes()));

        stack.values.push(quotient);
        stack.values.push(remainder);
    } else if token == "yield" {
        return Err(Effect::Yield);
    } else {
        return Err(Effect::UnknownIdentifier);
    }

    Ok(())
}

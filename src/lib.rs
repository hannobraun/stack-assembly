//! # The interpreter for the StackAssembly programming language

#![deny(missing_docs)]

#[cfg(test)]
mod tests;

/// # The ongoing evaluation of a script
pub struct Eval {
    /// # The script that we're evaluating
    pub script: String,

    /// # The operand stack
    pub stack: Vec<u32>,
}

impl Eval {
    /// # Start evaluating the provided script
    ///
    /// Returns an `Eval` instance that is ready. To evaluate any tokens in the
    /// provided script, you still have to explicitly call [`Eval::run`].
    pub fn start(script: &str) -> Self {
        Self {
            script: script.to_string(),
            stack: Vec::new(),
        }
    }

    /// # Advance the evaluation until it completes
    pub fn run(&mut self) {
        for token in self.script.split_whitespace() {
            if let Ok(value) = token.parse::<i32>() {
                let value = u32::from_le_bytes(value.to_le_bytes());
                self.stack.push(value);
            }
        }
    }
}

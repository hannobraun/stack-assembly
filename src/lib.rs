//! # The interpreter for the StackAssembly programming language

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

#[cfg(test)]
mod tests;

/// # The ongoing evaluation of a script
#[derive(Debug)]
pub struct Eval {
    /// # The operators of the script we're evaluating
    pub operators: Vec<Operator>,

    /// # The index of the next token to evaluate
    pub next_token: usize,

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
        let mut operators = Vec::new();

        for token in script.split_whitespace() {
            let operator = if let Ok(value) = token.parse::<i32>() {
                Operator::Integer { value }
            } else {
                Operator::Identifier {
                    name: token.to_string(),
                }
            };

            operators.push(operator);
        }

        Self {
            operators,
            next_token: 0,
            stack: Vec::new(),
            effect: None,
        }
    }

    /// # Advance the evaluation by one step
    pub fn step(&mut self) -> bool {
        if self.effect.is_some() {
            return false;
        }

        let Some(operator) = self.operators.get(self.next_token) else {
            return false;
        };

        match operator {
            Operator::Identifier { name } => {
                if name == "yield" {
                    self.effect = Some(Effect::Yield);
                } else if name == "jump" {
                    let Some(index) = self.stack.pop() else {
                        panic!("Stack underflow");
                    };
                    let Ok(index) = index.try_into() else {
                        panic!("Operator index out of bounds");
                    };

                    self.next_token = index;

                    // By default, we increment `self.next_token` below. Since
                    // we just set that to the exact value we want, we need to
                    // bypass that.
                    return true;
                } else {
                    self.effect = Some(Effect::UnknownIdentifier);
                }
            }
            Operator::Integer { value } => {
                let value = u32::from_le_bytes(value.to_le_bytes());
                self.stack.push(value);
            }
        }

        self.next_token += 1;

        true
    }

    /// # Advance the evaluation until it triggers an effect or completes
    pub fn run(&mut self) {
        while self.step() {}
    }
}

/// # An operator
///
/// Operators are a type of token that can be evaluated.
#[derive(Debug)]
pub enum Operator {
    /// # The operator is an identifier
    Identifier {
        /// # The name of the identifier
        name: String,
    },

    /// # The operator is an integer
    Integer {
        /// # The value of the integer
        value: i32,
    },
}

/// # An effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

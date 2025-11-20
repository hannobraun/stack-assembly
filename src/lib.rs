//! # The interpreter for the StackAssembly programming language

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod stack;

#[cfg(test)]
mod tests;

pub use self::stack::Stack;

/// # The ongoing evaluation of a script
#[derive(Debug)]
pub struct Eval {
    /// # The operators of the script we're evaluating
    pub operators: Vec<Operator>,

    /// # The index of the next operator to evaluate
    pub next_operator: usize,

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
        let mut operators = Vec::new();

        for token in script.split_whitespace() {
            let operator = if let Ok(value) = token.parse::<i32>() {
                Operator::Integer { value }
            } else {
                Operator::Identifier {
                    value: token.to_string(),
                }
            };

            operators.push(operator);
        }

        Self {
            operators,
            next_operator: 0,
            effect: None,
            stack: Stack { values: Vec::new() },
        }
    }

    /// # Advance the evaluation by one step
    pub fn step(&mut self) -> bool {
        if self.effect.is_some() {
            return false;
        }

        if let Err(effect) = self.evaluate_next_operator() {
            self.effect = Some(effect);
            return false;
        }

        true
    }

    fn evaluate_next_operator(&mut self) -> Result<(), Effect> {
        let Some(operator) = self.operators.get(self.next_operator) else {
            return Err(Effect::OutOfTokens);
        };

        match operator {
            Operator::Identifier { value: identifier } => {
                if identifier == "*" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    self.stack.push(a.wrapping_mul(b));
                } else if identifier == "+" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    self.stack.push(a.wrapping_add(b));
                } else if identifier == "-" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    self.stack.push(a.wrapping_sub(b));
                } else if identifier == "/" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

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
                } else if identifier == "jump" {
                    let index = self.stack.pop()?.to_operator_index();
                    self.next_operator = index;

                    // By default, we increment `self.next_token` below. Since
                    // we just set that to the exact value we want, we need to
                    // bypass that.
                    return Ok(());
                } else if identifier == "yield" {
                    return Err(Effect::Yield);
                } else {
                    return Err(Effect::UnknownIdentifier);
                }
            }
            Operator::Integer { value } => {
                self.stack.push(*value);
            }
        }

        self.next_operator += 1;

        Ok(())
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
        /// # The value of the identifier
        value: String,
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

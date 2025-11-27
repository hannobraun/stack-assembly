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

    /// # The labels of the script we're evaluating
    pub labels: Vec<Label>,

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
        let mut labels = Vec::new();

        for token in script.split_whitespace() {
            let operator = if let Some((name, "")) = token.rsplit_once(":") {
                labels.push(Label {
                    name: name.to_string(),
                    index: operators.len(),
                });
                continue;
            } else if let Some(("", name)) = token.split_once("@") {
                Operator::Reference {
                    name: name.to_string(),
                }
            } else if let Ok(value) = token.parse::<i32>() {
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
            labels,
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
                } else if identifier == "<" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a < b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == "<=" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a <= b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == "=" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    let c = if a == b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == ">" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a > b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == ">=" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a >= b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == "and" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = a & b;

                    self.stack.push(c);
                } else if identifier == "copy" {
                    let index_from_top = self.stack.pop()?.to_usize();
                    let value = self.stack.get(index_from_top)?;
                    self.stack.push(value);
                } else if identifier == "drop" {
                    let index_from_top = self.stack.pop()?.to_usize();
                    self.stack.remove(index_from_top)?;
                } else if identifier == "jump" {
                    let index = self.stack.pop()?.to_usize();
                    self.next_operator = index;

                    // By default, we increment `self.next_token` below. Since
                    // we just set that to the exact value we want, we need to
                    // bypass that.
                    return Ok(());
                } else if identifier == "jump_if" {
                    let index = self.stack.pop()?.to_usize();
                    let condition = self.stack.pop()?.to_u32();

                    if condition != 0 {
                        self.next_operator = index;

                        // By default, we increment `self.next_token` below.
                        // Since we just set that to the exact value we want, we
                        // need to bypass that.
                        return Ok(());
                    }
                } else if identifier == "yield" {
                    return Err(Effect::Yield);
                } else {
                    return Err(Effect::UnknownIdentifier);
                }
            }
            Operator::Integer { value } => {
                self.stack.push(*value);
            }
            Operator::Reference { name } => {
                let label =
                    self.labels.iter().find(|label| &label.name == name);

                if let Some(&Label { ref name, index }) = label {
                    let Ok(index) = index.try_into() else {
                        panic!(
                            "Operator index `{index}` of label `{name}` is out \
                            of bounds. This can only happen on platforms where \
                            the width of Rust's `usize` is wider than 32 bits, \
                            with a script that consists of at least 2^32 \
                            operators.\n\
                            \n\
                            Scripts that large seem barely realistic in the \
                            first place, more so on a 32-bit platform. At \
                            best, this is a niche use case that StackAssembly \
                            happens to not support, making this panic an \
                            acceptable outcome."
                        );
                    };
                    let index: u32 = index;

                    self.stack.push(index);
                } else {
                    return Err(Effect::InvalidReference);
                }
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

    /// # The operator is a reference
    Reference {
        /// # The name of the operator that the reference refers to
        name: String,
    },
}

/// # A label
///
/// Labels are a type of token that exist in the code, but not at runtime. They
/// assign a name to the operator they precede.
#[derive(Debug)]
pub struct Label {
    /// # The name that the label assigns to the operator it precedes
    pub name: String,

    /// # The index of the operator that the label precedes
    pub index: usize,
}

/// # An effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to divide by zero
    DivisionByZero,

    /// # Evaluating an operation resulted in integer overflow
    IntegerOverflow,

    /// # Evaluated a reference that is not paired with a matching label
    InvalidReference,

    /// # An index that supposedly refers to a value on the stack doesn't
    InvalidStackIndex,

    /// # The evaluation ran out of tokens to evaluate
    OutOfTokens,

    /// # Tried popping a value from an empty stack
    StackUnderflow,

    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

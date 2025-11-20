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

    /// # The labels of the script we're evaluating
    pub labels: Vec<Label>,

    /// # The index of the next operator to evaluate
    pub next_operator: usize,

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
                    name: token.to_string(),
                }
            };

            operators.push(operator);
        }

        Self {
            operators,
            labels,
            next_operator: 0,
            stack: Vec::new(),
            effect: None,
        }
    }

    /// # Advance the evaluation by one step
    pub fn step(&mut self) -> bool {
        if self.effect.is_some() {
            return false;
        }

        let Some(operator) = self.operators.get(self.next_operator) else {
            return false;
        };

        match operator {
            Operator::Identifier { name } => {
                if name == "yield" {
                    self.effect = Some(Effect::Yield);
                } else if name == "jump" {
                    // TASK: Stack underflow should trigger an effect.
                    //
                    //       It might be better to implement the arithmetic
                    //       operators first. Those would require the same
                    //       capability, but would make for a smaller pull
                    //       request that wouldn't be as overloaded by this.
                    let Some(index) = self.stack.pop() else {
                        panic!("Stack underflow");
                    };
                    // TASK: Open issue about this check.
                    //
                    //       This should probably trigger an effect instead.
                    //       Once this library supports `no_std`, having values
                    //       that exceed the available resources (like in this
                    //       case, the maximum number of operators), could
                    //       become a common occurrence.
                    let Ok(index) = index.try_into() else {
                        panic!("Operator index out of bounds");
                    };

                    self.next_operator = index;

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
            Operator::Reference { name } => {
                let label =
                    self.labels.iter().find(|label| &label.name == name);

                if let Some(label) = label {
                    // TASK: See task above. Also, unify the handling of both
                    //       cases.
                    let Ok(index) = label.index.try_into() else {
                        panic!("Operator index out of bounds");
                    };

                    self.stack.push(index);
                } else {
                    // TASK: This should trigger an effect.
                    panic!("Invalid reference");
                }
            }
        }

        self.next_operator += 1;

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

    /// # The operator is a reference
    Reference {
        /// # The name of the label the reference refers to
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
    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

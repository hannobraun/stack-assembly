use std::iter;

use crate::Effect;

/// # A compiled script
///
/// To evaluate a script, you must first compile its textual representation into
/// an instance of this struct, using [`Script::compile`]. Afterwards, you can
/// evaluate the script using [`Eval`].
///
/// [`Eval`]: crate::Eval
#[derive(Debug)]
pub struct Script {
    operators: Vec<Operator>,
    labels: Vec<Label>,
}

impl Script {
    /// # Compile the source text of a script into an instance of `Script`
    pub fn compile(script: &str) -> Self {
        let mut operators = Vec::new();
        let mut labels = Vec::new();

        enum State {
            Initial,
            Comment,
            Token { start: usize },
        }
        let mut state = State::Initial;

        for (i, ch) in script.char_indices() {
            match (&state, ch) {
                (State::Initial, '#') => {
                    state = State::Comment;
                }
                (State::Initial, ch) if !ch.is_whitespace() => {
                    state = State::Token { start: i };
                }
                (State::Initial, _) => {
                    // Token won't start until we're past the whitespace.
                }
                (State::Comment, '\n') => {
                    state = State::Initial;
                }
                (State::Comment, _) => {
                    // Ignoring characters in comments.
                }
                (State::Token { start }, ch) if ch.is_whitespace() => {
                    let token = &script[*start..i];
                    parse_token(token, &mut operators, &mut labels);

                    state = State::Initial;
                }
                (State::Token { start: _ }, _) => {
                    // We already remembered the start of the token. Nothing
                    // else to do until it's over.
                }
            }
        }

        if let State::Token { start } = state {
            let token = &script[start..script.len()];
            parse_token(token, &mut operators, &mut labels);
        }

        Self { operators, labels }
    }

    pub(crate) fn get_operator(
        &self,
        index: OperatorIndex,
    ) -> Result<&Operator, InvalidOperatorIndex> {
        let Ok(index): Result<usize, _> = index.value.try_into() else {
            // We can at most store `usize::MAX` operators, so if we can't make
            // this conversion, then the index definitely doesn't point to an
            // operator.
            return Err(InvalidOperatorIndex);
        };

        let Some(operator) = self.operators.get(index) else {
            return Err(InvalidOperatorIndex);
        };

        Ok(operator)
    }

    pub(crate) fn resolve_reference(
        &self,
        name: &str,
    ) -> Result<OperatorIndex, InvalidReference> {
        let label = self.labels.iter().find(|label| label.name == name);

        let Some(&Label { name: _, operator }) = label else {
            return Err(InvalidReference);
        };

        Ok(operator)
    }

    /// # Iterate over all operators in the script
    pub fn operators(
        &self,
    ) -> impl Iterator<Item = (OperatorIndex, &Operator)> {
        let indices =
            iter::successors(Some(OperatorIndex::default()), |index| {
                Some(OperatorIndex {
                    value: index.value + 1,
                })
            });

        indices.zip(&self.operators)
    }
}

fn parse_token(
    token: &str,
    operators: &mut Vec<Operator>,
    labels: &mut Vec<Label>,
) {
    let operator = if let Some((name, "")) = token.rsplit_once(":") {
        let Ok(index) = operators.len().try_into() else {
            panic!(
                "Trying to create a label for an operator whose index can't be \
                represented as `u32`. This is only possible on 64-bit \
                platforms, when there are more than `u32::MAX` operators in a \
                script.\n\
                \n\
                That this limit can practically be reached with the language \
                as it currently is, seems highly unlikely. This makes this \
                panic an acceptable outcome.\n\
                \n\
                Long-term, once the API supports compiler errors, this case \
                should result in an such an error instead."
            );
        };

        labels.push(Label {
            name: name.to_string(),
            operator: OperatorIndex { value: index },
        });

        return;
    } else if let Some(("", name)) = token.split_once("@") {
        Operator::Reference {
            name: name.to_string(),
        }
    } else if let Some(("", value)) = token.split_once("0x")
        && let Ok(value) = i32::from_str_radix(value, 16)
    {
        Operator::Integer { value }
    } else if let Some(("", value)) = token.split_once("0x")
        && let Ok(value) = u32::from_str_radix(value, 16)
    {
        Operator::integer_u32(value)
    } else if let Ok(value) = token.parse::<i32>() {
        Operator::Integer { value }
    } else if let Ok(value) = token.parse::<u32>() {
        Operator::integer_u32(value)
    } else {
        Operator::Identifier {
            value: token.to_string(),
        }
    };

    operators.push(operator);
}

#[derive(Debug)]
pub enum Operator {
    Identifier { value: String },
    Integer { value: i32 },
    Reference { name: String },
}

impl Operator {
    pub fn integer_u32(value: u32) -> Self {
        Self::Integer {
            value: i32::from_le_bytes(value.to_le_bytes()),
        }
    }
}

/// # Refers to an operator in a script
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OperatorIndex {
    pub(crate) value: u32,
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub operator: OperatorIndex,
}

#[derive(Debug)]
pub struct InvalidOperatorIndex;

impl From<InvalidOperatorIndex> for Effect {
    fn from(InvalidOperatorIndex: InvalidOperatorIndex) -> Self {
        Effect::OutOfOperators
    }
}

#[derive(Debug)]
pub struct InvalidReference;

impl From<InvalidReference> for Effect {
    fn from(InvalidReference: InvalidReference) -> Self {
        Effect::InvalidReference
    }
}

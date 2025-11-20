use crate::Effect;

/// # The stack
#[derive(Debug)]
pub struct Stack {
    /// # The values on the stack
    pub values: Vec<u32>,
}

impl Stack {
    /// # Pop a value from the stack
    pub fn pop(&mut self) -> Result<u32, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }
}

/// # A stack underflow error
///
/// See [`Stack::pop`].
#[derive(Debug)]
pub struct StackUnderflow;

impl From<StackUnderflow> for Effect {
    fn from(StackUnderflow: StackUnderflow) -> Self {
        Effect::StackUnderflow
    }
}

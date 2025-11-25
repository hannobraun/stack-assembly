use crate::Effect;

/// # The stack
#[derive(Debug)]
pub struct Stack {
    /// # The values on the stack
    pub values: Vec<u32>,
}

impl Stack {
    /// # Push a value to the stack
    pub fn push(&mut self, value: u32) {
        self.values.push(value);
    }

    /// # Pop a value from the stack
    pub fn pop(&mut self) -> Result<u32, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }

    /// # Access the stack as a slice of `u32` values
    pub fn to_u32_slice(&self) -> &[u32] {
        &self.values
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

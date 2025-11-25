use crate::Effect;

/// # The stack
#[derive(Debug)]
pub struct Stack {
    /// # The values on the stack
    pub values: Vec<Value>,
}

impl Stack {
    /// # Push a value to the stack
    pub fn push(&mut self, value: u32) {
        self.values.push(Value::from(value));
    }

    /// # Pop a value from the stack
    pub fn pop(&mut self) -> Result<u32, StackUnderflow> {
        self.values
            .pop()
            .ok_or(StackUnderflow)
            .map(|value| value.inner)
    }

    /// # Access the stack as a slice of `u32` values
    pub fn to_u32_slice(&self) -> &[u32] {
        bytemuck::cast_slice(&self.values)
    }
}

/// # A unit of data
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Value {
    inner: u32,
}

impl From<u32> for Value {
    fn from(inner: u32) -> Self {
        Self { inner }
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

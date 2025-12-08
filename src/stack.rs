use crate::{Effect, Value};

/// # The operand stack
///
/// StackAssembly's evaluation model is based on an implicit stack which
/// stores all operands.
///
/// Aside from this, the stack is an important communication channel between
/// script and host. Please refer to [`Eval`]'s [`stack`] field for more
/// information on that.
///
/// [`Eval`]: crate::Eval
/// [`stack`]: struct.Eval.html#structfield.stack
#[derive(Debug)]
pub struct Stack {
    /// # The values on the stack
    pub values: Vec<Value>,
}

impl Stack {
    /// # Push a value to top of the stack
    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    /// # Pop a value from the top of the stack
    ///
    /// Return [`StackUnderflow`], if no value is available on the stack, which
    /// provides an automatic conversion to [`Effect`].
    pub fn pop(&mut self) -> Result<Value, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }

    /// # Access the stack as a slice of `u32` values
    pub fn to_u32_slice(&self) -> &[u32] {
        bytemuck::cast_slice(&self.values)
    }
}

/// # Tried to pop a value from an empty stack
///
/// See [`Stack::pop`].
#[derive(Debug)]
pub struct StackUnderflow;

impl From<StackUnderflow> for Effect {
    fn from(StackUnderflow: StackUnderflow) -> Self {
        Effect::StackUnderflow
    }
}

#[derive(Debug)]
pub struct InvalidStackIndex;

impl From<InvalidStackIndex> for Effect {
    fn from(InvalidStackIndex: InvalidStackIndex) -> Self {
        Effect::InvalidStackIndex
    }
}

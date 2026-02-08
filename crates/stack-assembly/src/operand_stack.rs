use crate::{Effect, Value};

/// # The operand stack
///
/// StackAssembly's evaluation model is based on an implicit stack which
/// stores all operands.
///
/// Aside from this, the stack is an important communication channel between
/// script and host. Please refer to [`Eval`]'s [`operand_stack`] field for more
/// information on that.
///
/// [`Eval`]: crate::Eval
/// [`operand_stack`]: struct.Eval.html#structfield.operand_stack
#[derive(Debug, Default)]
pub struct OperandStack {
    /// # The values on the stack
    pub values: Vec<Value>,
}

impl OperandStack {
    /// # Push a value to top of the stack
    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    /// # Pop a value from the top of the stack
    ///
    /// Return [`OperandStackUnderflow`], if no value is available on the stack,
    /// which provides an automatic conversion to [`Effect`].
    pub fn pop(&mut self) -> Result<Value, OperandStackUnderflow> {
        self.values.pop().ok_or(OperandStackUnderflow)
    }

    /// # Access the stack as a slice of `i32` values
    pub fn to_i32_slice(&self) -> &[i32] {
        bytemuck::cast_slice(&self.values)
    }

    /// # Access the stack as a slice of `u32` values
    pub fn to_u32_slice(&self) -> &[u32] {
        bytemuck::cast_slice(&self.values)
    }
}

/// # Tried to pop a value from an empty stack
///
/// See [`OperandStack::pop`].
#[derive(Debug)]
pub struct OperandStackUnderflow;

impl From<OperandStackUnderflow> for Effect {
    fn from(OperandStackUnderflow: OperandStackUnderflow) -> Self {
        Effect::OperandStackUnderflow
    }
}

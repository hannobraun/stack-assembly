use crate::{Effect, Value};

/// # The operand stack
///
/// StackAssembly's evaluation model is based on an implicit stack on which
/// operands are stored. An operators output is pushed to that stack, and
/// any of its inputs are popped from there.
///
/// A host may access the stack to communicate with a script that has triggered
/// [`Effect::Yield`]. A host may also access the stack under any other
/// circumstances. This is considered non-standard and should be avoided under
/// most circumstances, as it interferes with the evaluation of the script.
///
/// The stack for a given evaluation is stored in [`Eval`]'s [`stack`] field.
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

    /// # Pop a value from the stack
    ///
    /// Return [`StackUnderflow`], if no value is available on the stack, which
    /// provides an automatic conversion to [`Effect`].
    pub fn pop(&mut self) -> Result<Value, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }

    /// # Access the value at the given index
    ///
    /// Stack indices start at the top, meaning `0` refers to the topmost value
    /// on the stack.
    ///
    /// Return [`InvalidStackIndex`], if the provided index does not refer to a
    /// value on the stack, which provides an automatic conversion to
    /// [`Effect`].
    pub fn get(
        &self,
        index_from_top: usize,
    ) -> Result<Value, InvalidStackIndex> {
        let index_from_bottom = self.convert_index(index_from_top)?;

        let Some(value) = self.values.get(index_from_bottom).copied() else {
            unreachable!(
                "We computed the index from the top, based on the \
                number of values on the stack. Since that did not \
                result in an integer overflow, it's not possible \
                that we ended up with an out-of-range index."
            );
        };

        Ok(value)
    }

    /// # Remove the value at the given index
    ///
    /// Stack indices start at the top, meaning `0` refers to the topmost value
    /// on the stack.
    ///
    /// Return [`InvalidStackIndex`], if the provided index does not refer to a
    /// value on the stack, which provides an automatic conversion to
    /// [`Effect`].
    pub fn remove(
        &mut self,
        index_from_top: usize,
    ) -> Result<(), InvalidStackIndex> {
        let index_from_bottom = self.convert_index(index_from_top)?;

        // This could theoretically panic, but actually won't, for the same
        // reason that the index must be valid in `get`.
        self.values.remove(index_from_bottom);

        Ok(())
    }

    fn convert_index(
        &self,
        index_from_top: usize,
    ) -> Result<usize, InvalidStackIndex> {
        let index_from_bottom = self
            .values
            .len()
            .checked_sub(1)
            .and_then(|index| index.checked_sub(index_from_top));

        index_from_bottom.ok_or(InvalidStackIndex)
    }

    /// # Access the stack as a slice of `u32` values
    pub fn to_u32_slice(&self) -> &[u32] {
        bytemuck::cast_slice(&self.values)
    }
}

/// # An invalid index was used to access the stack
///
/// See [`Stack::get`] and [`Stack::remove`].
#[derive(Debug)]
pub struct InvalidStackIndex;

impl From<InvalidStackIndex> for Effect {
    fn from(InvalidStackIndex: InvalidStackIndex) -> Self {
        Effect::InvalidStackIndex
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

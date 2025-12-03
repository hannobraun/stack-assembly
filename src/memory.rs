use std::fmt;

use crate::Value;

/// # A linear memory, freely addressable per word
///
/// The memory can be accessed from a script through the `read` and `write`
/// operators.
///
/// A host may access the memory to communicate with a script that has triggered
/// [`Effect::Yield`]. A host may also access the memory under any other
/// circumstances. This is considered non-standard and should be avoided under
/// most circumstances, as it interferes with the evaluation of the script.
///
/// The memory for a given evaluation is stored in [`Eval`]'s [`memory`] field.
///
/// [`Effect::Yield`]: crate::Effect::Yield
/// [`Eval`]: crate::Eval
/// [`memory`]: struct.Eval.html#structfield.memory
pub struct Memory {
    /// # The values in the memory
    pub values: Vec<Value>,
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This is not perfect, but it's way more compact than the derived
        // implementation. That was so verbose as to be barely usable.

        let mut values = self.values.iter().peekable();

        write!(f, "[")?;

        while let Some(value) = values.next() {
            write!(f, "{value:?}")?;

            if values.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")?;

        Ok(())
    }
}

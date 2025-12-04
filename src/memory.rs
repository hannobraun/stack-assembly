use std::fmt;

use crate::Value;

/// # A linear memory, freely addressable per word
///
/// The memory can be accessed from a script through the `read` and `write`
/// operators.
///
/// Aside from this, the stack is an important communication channel between
/// script and host. Please refer to [`Eval`]'s [`memory`] field for more
/// information on that.
///
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

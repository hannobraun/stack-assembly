use std::fmt;

use crate::{Effect, Value};

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

impl Memory {
    /// # Read the value at the provided address
    pub fn read(&self, address: u32) -> Result<Value, InvalidAddress> {
        let Ok(address): Result<usize, _> = address.try_into() else {
            // It is not possible to have memories larger than what can be
            // addressed by `usize`. So by definition, any address that's too
            // large to convert to `usize`, can not be valid.
            return Err(InvalidAddress);
        };

        let Some(value) = self.values.get(address).copied() else {
            return Err(InvalidAddress);
        };

        Ok(value)
    }

    /// # Write a value to an address
    pub fn write(
        &mut self,
        address: u32,
        value: Value,
    ) -> Result<(), InvalidAddress> {
        let Ok(address): Result<usize, _> = address.try_into() else {
            // It is not possible to have memories larger than what can be
            // addressed by `usize`. So by definition, any address that's too
            // large to convert to `usize`, can not be valid.
            return Err(InvalidAddress);
        };

        if address >= self.values.len() {
            return Err(InvalidAddress);
        }

        self.values[address] = value;

        Ok(())
    }

    /// # Access the memory as a slice of `i32` values
    pub fn to_i32_slice(&self) -> &[i32] {
        bytemuck::cast_slice(&self.values)
    }

    /// # Access the memory as a slice of `u32` values
    pub fn to_u32_slice(&self) -> &[u32] {
        bytemuck::cast_slice(&self.values)
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // This is not perfect, but it's way more compact than the derived
        // implementation.

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

#[derive(Debug)]
pub struct InvalidAddress;

impl From<InvalidAddress> for Effect {
    fn from(InvalidAddress: InvalidAddress) -> Self {
        Effect::InvalidAddress
    }
}

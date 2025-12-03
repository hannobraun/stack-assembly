use crate::Value;

/// # A linear memory, freely addressable per word
#[derive(Debug)]
pub struct Memory {
    /// # The values in the memory
    pub values: Vec<Value>,
}

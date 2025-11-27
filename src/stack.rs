use crate::Effect;

/// # The stack
#[derive(Debug)]
pub struct Stack {
    /// # The values on the stack
    pub values: Vec<Value>,
}

impl Stack {
    /// # Push a value to the stack
    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    /// # Pop a value from the stack
    pub fn pop(&mut self) -> Result<Value, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }

    /// # Access the value at the given index
    ///
    /// Stack indices start at the top, meaning `0` refers to the topmost value
    /// on the stack.
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

/// # A unit of data
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Value {
    inner: u32,
}

impl Value {
    pub fn to_i32(self) -> i32 {
        i32::from_le_bytes(self.inner.to_le_bytes())
    }

    pub fn to_u32(self) -> u32 {
        self.inner
    }

    pub fn to_usize(self) -> usize {
        let Ok(index) = self.inner.try_into() else {
            panic!(
                "Can't convert value `{value}` to `usize`. This should only be \
                possible on platforms where Rust's `usize` is less than 32 \
                bits wide. This is a niche use case that isn't fully \
                supported, making this panic an acceptable outcome.\n\
                \n\
                Additionally, since `usize` is only used for storage of values \
                or operators, the value was invalid in the first place \
                (meaning the StackAssembly program has a bug), as it wouldn't \
                be possible to store as many item as the value implies should \
                be there.",
                value = self.inner,
            );
        };

        index
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        let inner = u32::from_le_bytes(value.to_le_bytes());
        Self { inner }
    }
}

impl From<u32> for Value {
    fn from(inner: u32) -> Self {
        Self { inner }
    }
}

/// # An invalid index was used to access the stack
///
/// See [`Stack::by_index`].
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

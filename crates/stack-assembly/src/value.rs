use std::fmt;

/// # A unit of data
///
/// StackAssembly is an _untyped_ languages. All of its values, both on the
/// [`OperandStack`] and in [`Memory`], are 32 bits wide. Depending on the
/// situation, they may be interpreted as unsigned or signed.
///
/// You can create an instance of `Value` through its `From` implementations.
///
/// ```
/// use stack_assembly::Value;
///
/// Value::from(3i32);
/// Value::from(5u32);
/// ```
///
/// [`OperandStack`]: crate::OperandStack
/// [`Memory`]: crate::Memory
#[derive(Clone, Copy, Eq, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Value {
    inner: u32,
}

impl Value {
    /// # Convert the value to an `i32`
    ///
    /// Since all values are 32 bits wide, this is always possible. Interprets
    /// the bits of the value as a signed (two's complement) integer.
    pub fn to_i32(self) -> i32 {
        i32::from_le_bytes(self.inner.to_le_bytes())
    }

    /// # Convert the value to a `u32`
    ///
    /// Since all values are 32 bits wide, this is always possible. Interprets
    /// the bits of the value as an unsigned integer.
    pub fn to_u32(self) -> u32 {
        self.inner
    }

    /// # Convert to a `usize`
    ///
    /// This is usually possible, unless this library runs on a platform where
    /// `usize` is less than 32 bits wide. That is considered a niche use case
    /// which is not fully supported.
    ///
    /// ## Panics
    ///
    /// Panics, if `usize` can not represent this value.
    pub fn to_usize(self) -> usize {
        let Ok(index) = self.inner.try_into() else {
            panic!(
                "Can't convert value `{value}` to `usize`. This can only \
                happen  on platforms where Rust's `usize` is less than 32 bits \
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

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Let's bypass this type and format the inner value. This is just a
        // wrapper anyway, and showing it in debug output is unnecessary noise.
        self.inner.fmt(f)
    }
}

/// # An event triggered by scripts, to signal a specific condition
///
/// Evaluating an operator can trigger an effect. Active effects are stored in
/// in [`Eval`]'s [`effect`] field. Please refer to the documentation of that
/// field, for more information on effects and how to handle them.
///
/// [`Eval`]: crate::Eval
/// [`effect`]: struct.Eval.html#structfield.effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to divide by zero
    ///
    /// Can trigger when evaluating the `/` operator, if its second input is
    /// `0`.
    DivisionByZero,

    /// # Division resulted in integer overflow
    ///
    /// Can only trigger when evaluating the `/` operator, if its first input is
    /// the lowest signed (two's complement) 32-bit integer, and its second
    /// input is `-1`.
    ///
    /// All other arithmetic operators wrap on overflow and don't trigger this
    /// effect.
    IntegerOverflow,

    /// # A memory address is out of bounds
    ///
    /// Can trigger when evaluating the `read` or `write` operators, if their
    /// _address_ input (when interpreted as an unsigned 32-bit integer) does
    /// not refer to an address that is within the bounds of the memory.
    InvalidAddress,

    /// # Evaluated a reference that is not paired with a matching label
    ///
    /// Can trigger when evaluating a reference, if that reference does not
    /// refer to a label.
    InvalidReference,

    /// # An index that supposedly refers to a value on the stack, doesn't
    ///
    /// Can trigger when evaluating the `copy` or `drop` operators, if their
    /// _index_ input is too large to refer to a value on the stack.
    InvalidStackIndex,

    /// # Ran out of operators to evaluate
    ///
    /// Triggers when evaluation reaches the end of the script, where no more
    /// operators are available. This signals the regular end of the evaluation.
    OutOfOperators,

    /// # Tried popping a value from an empty stack
    ///
    /// Can trigger when evaluating any operator that has more inputs than the
    /// number of values currently on the stack.
    StackUnderflow,

    /// # Evaluated an identifier that the language does not recognize
    ///
    /// Can trigger when evaluating an identifier, if that identifier does not
    /// refer to a known operation.
    UnknownIdentifier,

    /// # The evaluating script yields control to the host
    ///
    /// Triggers when evaluating the `yield` operator.
    Yield,
}

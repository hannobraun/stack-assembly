/// # An event triggered by scripts, to signal a specific condition
///
/// Effects moderate the communication between script and host. The effect
/// itself only relays _which_ effect has triggered, but that may signal to
/// the host that a different communication channel (like operand stack or
/// memory) is ready to be accessed.
///
/// ## Handling Effects
///
/// The host may handle effects however it wishes. But since most effects
/// signal error conditions that the script would not expect to recover
/// from, a well-behaving host must be careful not to handle effects in
/// a way that make reasoning about the script's behavior difficult.
///
/// Abandoning the evaluation and reporting an error in the appropriate
/// manner, is the only reasonable way to handle most effects. The
/// exception to that is [`Effect::Yield`], which does not signal an error
/// condition. A script would expect to continue afterwards.
///
/// To make that possible, the host must clear the effect by setting this
/// field to `None`.
///
/// ### Example
///
/// ```
/// use stack_assembly::{Effect, Eval, Script};
///
/// // This script increments a number in a loop, yielding control to the
/// // host every time it did so.
/// let script = Script::compile("
///     0
///
///     increment:
///         1 +
///         yield
///         @increment jump
/// ");
///
/// let mut eval = Eval::new();
///
/// // When running the script for the first time, we expect that it has
/// // incremented the number once, before yielding.
/// let (effect, _) = eval.run(&script);
/// assert_eq!(effect, Effect::Yield);
/// assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
///
/// // To allow the script to continue, we must clear the effect.
/// eval.clear_effect();
///
/// // Since we handled the effect correctly, we can now assume that the
/// // script has incremented the number a second time, before yielding
/// // again.
/// let (effect, _) = eval.run(&script);
/// assert_eq!(effect, Effect::Yield);
/// assert_eq!(eval.operand_stack.to_u32_slice(), &[2]);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Effect {
    /// # An assertion failed
    ///
    /// Can trigger when evaluating `assert`, if its input is zero.
    AssertionFailed,

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

    /// # Index doesn't refer to valid value on the operand stack
    ///
    /// Can trigger when evaluating the `copy` or `drop` operators, if their
    /// _index_ input is too large to refer to a value on the operand stack.
    InvalidOperandStackIndex,

    /// # Evaluated a reference that is not paired with a matching label
    ///
    /// Can trigger when evaluating a reference, if that reference does not
    /// refer to a label.
    InvalidReference,

    /// # Tried popping a value from an empty operand stack
    ///
    /// Can trigger when evaluating any operator that has more inputs than the
    /// number of values currently on the operand stack.
    OperandStackUnderflow,

    /// # Ran out of operators to evaluate
    ///
    /// Triggers when evaluation reaches the end of the script, where no more
    /// operators are available. This is not an error, which makes it one of the
    /// ways to signal the regular end of evaluation, alongside
    /// [`Effect::Return`].
    OutOfOperators,

    /// # Evaluated `return` while call stack was empty
    ///
    /// This is not an error, which makes it one of the ways to signal the
    /// regular end of evaluation, alongside [`Effect::OutOfOperators`].
    Return,

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

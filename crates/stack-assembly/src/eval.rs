use crate::{
    Effect, Memory, OperandStack,
    script::{Operator, OperatorIndex, Script},
};

/// # The ongoing evaluation of a script
///
/// This is the main entry point into this library's API. To evaluate a script,
/// you can pass it to [`Eval::new`], then use [`Eval::run`] or [`Eval::step`]
/// to advance the evaluation.
///
/// ## Example
///
/// ```
/// use stack_assembly::{Eval, Script};
///
/// let script = Script::compile("1 2 +");
///
/// let mut eval = Eval::new();
/// eval.run(&script);
///
/// assert_eq!(eval.operand_stack.to_i32_slice(), &[3]);
/// ```
#[derive(Debug, Default)]
pub struct Eval {
    next_operator: OperatorIndex,
    call_stack: Vec<OperatorIndex>,

    /// # The active effect, if one has triggered
    ///
    /// Effects moderate the communication between script and host. The effect
    /// itself only relays _which_ effect has triggered, but that may signal to
    /// the host that a different communication channel (like [`operand_stack`]
    /// or [`memory`]) is ready to be accessed.
    ///
    /// [`Eval::new`] initializes this field to `None`. [`Eval::run`] and
    /// [`Eval::step`] may store an effect here, if the script triggers one. If
    /// that is the case, the host may handle the effect, to allow evaluation
    /// to continue.
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
    /// eval.run(&script);
    /// assert_eq!(eval.effect, Some(Effect::Yield));
    /// assert_eq!(eval.operand_stack.to_u32_slice(), &[1]);
    ///
    /// // To allow the script to continue, we must clear the effect.
    /// eval.effect = None;
    ///
    /// // Since we handled the effect correctly, we can now assume that the
    /// // script has incremented the number a second time, before yielding
    /// // again.
    /// eval.run(&script);
    /// assert_eq!(eval.effect, Some(Effect::Yield));
    /// assert_eq!(eval.operand_stack.to_u32_slice(), &[2]);
    /// ```
    ///
    /// [`next_operator`]: #structfield.next_operator
    /// [`operand_stack`]: #structfield.operand_stack
    /// [`memory`]: #structfield.memory
    pub effect: Option<Effect>,

    /// # The operand stack
    ///
    /// StackAssembly's evaluation model is based on an implicit stack which
    /// stores all operands. An operator's output is pushed to that stack, and
    /// any of its inputs are popped from there.
    ///
    /// Alongside [`memory`], this field is the primary channel for
    /// communication between script and host.
    ///
    /// Most hosts should restrict modifications to this field to when the
    /// script triggers [`Effect::Yield`], and then only do so in a
    /// well-reasoned and documented manner. Anything else might make reasoning
    /// about the script's behavior very difficult.
    ///
    /// None the less, the host has full access to this field, as to not
    /// restrict any experimental or non-standard use cases.
    ///
    /// [`memory`]: #structfield.memory
    pub operand_stack: OperandStack,

    /// # The memory
    ///
    /// StackAssembly provides a linear memory that is freely addressable per
    /// word.
    ///
    /// Alongside [`operand_stack`], this field is the primary channel for
    /// communication between script and host.
    ///
    /// Most hosts should restrict modifications to this field to when the
    /// script triggers [`Effect::Yield`], and then only do so in a
    /// well-reasoned and documented manner. Anything else might make reasoning
    /// about the script's behavior very difficult.
    ///
    /// None the less, the host has full access to this field, as to not
    /// restrict any experimental or non-standard use cases.
    ///
    /// [`operand_stack`]: #structfield.operand_stack
    pub memory: Memory,
}

impl Eval {
    /// # Start evaluating the provided script
    ///
    /// Compile the provided script and return an `Eval` instance that is ready
    /// for evaluation. To evaluate any operators, you must call [`Eval::run`]
    /// or [`Eval::step`].
    pub fn new() -> Self {
        Self::default()
    }

    /// # Advance the evaluation until it triggers an effect
    ///
    /// If an effect is currently active (see [`effect`] field), do nothing and
    /// return immediately. Otherwise, keep evaluating operators until one
    /// triggers an effect.
    ///
    /// If you need more control over the evaluation, consider using
    /// [`Eval::step`] instead.
    ///
    /// [`effect`]: #structfield.effect
    /// [`next_operator`]: #structfield.next_operator
    pub fn run(&mut self, script: &Script) -> &mut Effect {
        while self.effect.is_none() {
            self.step(script);
        }

        // It's a bit of a shame we have to unwrap the `Option` like this, but
        // I tried doing it from within the loop, and failed due to the borrow
        // checker.
        let Some(effect) = &mut self.effect else {
            unreachable!(
                "An effect must have triggered, or we wouldn't have exited the \
                loop just now."
            );
        };

        effect
    }

    /// # Advance the evaluation by one step
    ///
    /// If an effect is currently active (see [`effect`] field), do nothing and
    /// return immediately. Otherwise, evaluate the next operator. If that
    /// triggers an effect, store that in the [`effect`] field.
    ///
    /// This function may be used for advancing the evaluation of the script in
    /// a controlled manner. If you just want to keep evaluating until the next
    /// effect, consider using [`Eval::run`] instead.
    ///
    /// [`effect`]: #structfield.effect
    /// [`next_operator`]: #structfield.next_operator
    pub fn step(&mut self, script: &Script) {
        if self.effect.is_some() {
            return;
        }

        if let Err(effect) = self.evaluate_next_operator(script) {
            self.effect = Some(effect);
        }
    }

    fn evaluate_next_operator(
        &mut self,
        script: &Script,
    ) -> Result<(), Effect> {
        let operator = script.get_operator(self.next_operator)?;
        self.next_operator.value += 1;

        match operator {
            Operator::Identifier { value: identifier } => {
                if identifier == "*" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.wrapping_mul(b));
                } else if identifier == "+" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.wrapping_add(b));
                } else if identifier == "-" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.wrapping_sub(b));
                } else if identifier == "/" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    if b == 0 {
                        return Err(Effect::DivisionByZero);
                    }
                    if a == i32::MIN && b == -1 {
                        return Err(Effect::IntegerOverflow);
                    }

                    self.operand_stack.push(a / b);
                    self.operand_stack.push(a % b);
                } else if identifier == "<" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a < b);
                } else if identifier == "<=" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a <= b);
                } else if identifier == "=" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a == b);
                } else if identifier == ">" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a > b);
                } else if identifier == ">=" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a >= b);
                } else if identifier == "and" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a & b);
                } else if identifier == "or" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a | b);
                } else if identifier == "xor" {
                    let b = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a ^ b);
                } else if identifier == "count_ones" {
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.count_ones());
                } else if identifier == "leading_zeros" {
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.leading_zeros());
                } else if identifier == "trailing_zeros" {
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.trailing_zeros());
                } else if identifier == "rotate_left" {
                    let num_positions = self.operand_stack.pop()?.to_u32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.rotate_left(num_positions));
                } else if identifier == "rotate_right" {
                    let num_positions = self.operand_stack.pop()?.to_u32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a.rotate_right(num_positions));
                } else if identifier == "shift_left" {
                    let num_positions = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a << num_positions);
                } else if identifier == "shift_right" {
                    let num_positions = self.operand_stack.pop()?.to_i32();
                    let a = self.operand_stack.pop()?.to_i32();

                    self.operand_stack.push(a >> num_positions);
                } else if identifier == "copy" {
                    let index_from_top = self.operand_stack.pop()?.to_u32();
                    let index_from_bottom = convert_operand_stack_index(
                        &self.operand_stack,
                        index_from_top,
                    )?;

                    let Some(value) = self
                        .operand_stack
                        .values
                        .get(index_from_bottom)
                        .copied()
                    else {
                        unreachable!(
                            "We computed the index from the top, based on the \
                            number of values on the stack. Since that did not \
                            result in an integer overflow, it's not possible \
                            that we ended up with an out-of-range index."
                        );
                    };

                    self.operand_stack.push(value);
                } else if identifier == "drop" {
                    let index_from_top = self.operand_stack.pop()?.to_u32();
                    let index_from_bottom = convert_operand_stack_index(
                        &self.operand_stack,
                        index_from_top,
                    )?;

                    // This could theoretically panic, but actually won't, for
                    // the same reason that the index must be valid in the
                    // implementation of `copy`.
                    self.operand_stack.values.remove(index_from_bottom);
                } else if identifier == "jump" {
                    let index = self.operand_stack.pop()?.to_u32();

                    self.next_operator.value = index;
                } else if identifier == "jump_if" {
                    let index = self.operand_stack.pop()?.to_u32();
                    let condition = self.operand_stack.pop()?.to_bool();

                    if condition {
                        self.next_operator.value = index;
                    }
                } else if identifier == "call" {
                    self.call_stack.push(self.next_operator);

                    let index = self.operand_stack.pop()?.to_u32();

                    self.next_operator.value = index;
                } else if identifier == "call_either" {
                    self.call_stack.push(self.next_operator);

                    let else_ = self.operand_stack.pop()?.to_u32();
                    let then = self.operand_stack.pop()?.to_u32();
                    let condition = self.operand_stack.pop()?.to_bool();

                    self.next_operator = {
                        let value = if condition { then } else { else_ };
                        OperatorIndex { value }
                    };
                } else if identifier == "return" {
                    let Some(index) = self.call_stack.pop() else {
                        return Err(Effect::Return);
                    };

                    self.next_operator = index;
                } else if identifier == "assert" {
                    let condition = self.operand_stack.pop()?.to_bool();

                    if !condition {
                        return Err(Effect::AssertionFailed);
                    }
                } else if identifier == "yield" {
                    return Err(Effect::Yield);
                } else if identifier == "read" {
                    let address = self.operand_stack.pop()?.to_u32();

                    let value = self.memory.read(address)?;

                    self.operand_stack.push(value);
                } else if identifier == "write" {
                    let value = self.operand_stack.pop()?;
                    let address = self.operand_stack.pop()?.to_u32();

                    self.memory.write(address, value)?;
                } else {
                    return Err(Effect::UnknownIdentifier);
                }
            }
            Operator::Integer { value } => {
                self.operand_stack.push(*value);
            }
            Operator::Reference { name } => {
                let operator = script.resolve_reference(name)?;
                self.operand_stack.push(operator.value);
            }
        }

        Ok(())
    }
}

fn convert_operand_stack_index(
    operand_stack: &OperandStack,
    index_from_top: u32,
) -> Result<usize, Effect> {
    let Ok(index_from_top): Result<usize, _> = index_from_top.try_into() else {
        // It is not possible to have a stack larger than what `usize` can
        // address. So by definition, any index that's too large to convert to
        // `usize`, can not be valid.
        return Err(Effect::InvalidOperandStackIndex);
    };

    let index_from_bottom = operand_stack
        .values
        .len()
        .checked_sub(1)
        .and_then(|index| index.checked_sub(index_from_top));

    index_from_bottom.ok_or(Effect::InvalidOperandStackIndex)
}

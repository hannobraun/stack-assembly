use crate::{Effect, Memory, Stack, Value};

/// # The ongoing evaluation of a script
///
/// This is the main entry point into this library's API. To evaluate a script,
/// you can pass it to [`Eval::start`], then use [`Eval::run`] or [`Eval::step`]
/// to advance the evaluation.
///
/// ## Example
///
/// ```
/// use stack_assembly::Eval;
///
/// let script = "1 2 +";
///
/// let mut eval = Eval::start(script);
/// eval.run();
///
/// assert_eq!(eval.stack.to_i32_slice(), &[3]);
/// ```
#[derive(Debug)]
pub struct Eval {
    operators: Vec<Operator>,
    labels: Vec<Label>,
    next_operator: usize,

    /// # The active effect, if one has triggered
    ///
    /// Effects moderate the communication between script and host. The effect
    /// itself only relays _which_ effect has triggered, but that may signal to
    /// the host that a different communication channel (like [`stack`] or
    /// [`memory`]) is ready to be accessed.
    ///
    /// [`Eval::start`] initializes this field to `None`. [`Eval::run`] and
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
    /// use stack_assembly::{Effect, Eval};
    ///
    /// // This script increments a number in a loop, yielding control to the
    /// // host every time it did so.
    /// let script = "
    ///     0
    ///
    ///     increment:
    ///         1 +
    ///         yield
    ///         @increment jump
    /// ";
    ///
    /// let mut eval = Eval::start(script);
    ///
    /// // When running the script for the first time, we expect that it has
    /// // incremented the number once, before yielding.
    /// eval.run();
    /// assert_eq!(eval.effect, Some(Effect::Yield));
    /// assert_eq!(eval.stack.to_u32_slice(), &[1]);
    ///
    /// // To allow the script to continue, we must clear the effect.
    /// eval.effect = None;
    ///
    /// // Since we handled the effect correctly, we can now assume that the
    /// // script has incremented the number a second time, before yielding
    /// // again.
    /// eval.run();
    /// assert_eq!(eval.effect, Some(Effect::Yield));
    /// assert_eq!(eval.stack.to_u32_slice(), &[2]);
    /// ```
    ///
    /// [`next_operator`]: #structfield.next_operator
    /// [`stack`]: #structfield.stack
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
    pub stack: Stack,

    /// # The memory
    ///
    /// StackAssembly provides a linear memory that is freely addressable per
    /// word.
    ///
    /// Alongside [`stack`], this field is the primary channel for
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
    /// [`stack`]: #structfield.stack
    pub memory: Memory,
}

impl Eval {
    /// # Start evaluating the provided script
    ///
    /// Compile the provided script and return an `Eval` instance that is ready
    /// for evaluation. To evaluate any operators, you must call [`Eval::run`]
    /// or [`Eval::step`].
    pub fn start(script: &str) -> Self {
        let mut operators = Vec::new();
        let mut labels = Vec::new();

        for line in script.lines() {
            for token in line.split_whitespace() {
                if token.starts_with("#") {
                    // This is a comment. Ignore the rest of the line.
                    break;
                }

                let operator = if let Some((name, "")) = token.rsplit_once(":")
                {
                    labels.push(Label {
                        name: name.to_string(),
                        operator: operators.len(),
                    });
                    continue;
                } else if let Some(("", name)) = token.split_once("@") {
                    Operator::Reference {
                        name: name.to_string(),
                    }
                } else if let Some(("", value)) = token.split_once("0x")
                    && let Ok(value) = i32::from_str_radix(value, 16)
                {
                    Operator::Integer { value }
                } else if let Ok(value) = token.parse::<i32>() {
                    Operator::Integer { value }
                } else if let Ok(value) = token.parse::<u32>() {
                    Operator::integer_u32(value)
                } else {
                    Operator::Identifier {
                        value: token.to_string(),
                    }
                };

                operators.push(operator);
            }
        }

        Self {
            operators,
            labels,
            next_operator: 0,
            effect: None,
            stack: Stack { values: Vec::new() },
            memory: Memory {
                values: vec![Value::from(0); 1024],
            },
        }
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
    pub fn run(&mut self) -> &mut Effect {
        while self.effect.is_none() {
            self.step();
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
    pub fn step(&mut self) {
        if self.effect.is_some() {
            return;
        }

        if let Err(effect) = self.evaluate_next_operator() {
            self.effect = Some(effect);
        }
    }

    fn evaluate_next_operator(&mut self) -> Result<(), Effect> {
        let Some(operator) = self.operators.get(self.next_operator) else {
            return Err(Effect::OutOfOperators);
        };
        self.next_operator += 1;

        match operator {
            Operator::Identifier { value: identifier } => {
                if identifier == "*" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    self.stack.push(a.wrapping_mul(b));
                } else if identifier == "+" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    self.stack.push(a.wrapping_add(b));
                } else if identifier == "-" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    self.stack.push(a.wrapping_sub(b));
                } else if identifier == "/" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    if b == 0 {
                        return Err(Effect::DivisionByZero);
                    }
                    if a == i32::MIN && b == -1 {
                        return Err(Effect::IntegerOverflow);
                    }

                    let quotient = a / b;
                    let remainder = a % b;

                    self.stack.push(quotient);
                    self.stack.push(remainder);
                } else if identifier == "<" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a < b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == "<=" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a <= b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == "=" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a == b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == ">" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a > b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == ">=" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = if a >= b { 1 } else { 0 };

                    self.stack.push(c);
                } else if identifier == "and" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = a & b;

                    self.stack.push(c);
                } else if identifier == "or" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = a | b;

                    self.stack.push(c);
                } else if identifier == "xor" {
                    let b = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let c = a ^ b;

                    self.stack.push(c);
                } else if identifier == "count_ones" {
                    let a = self.stack.pop()?.to_i32();
                    let b = a.count_ones();
                    self.stack.push(b);
                } else if identifier == "leading_zeros" {
                    let a = self.stack.pop()?.to_i32();
                    let b = a.leading_zeros();
                    self.stack.push(b);
                } else if identifier == "trailing_zeros" {
                    let a = self.stack.pop()?.to_i32();
                    let b = a.trailing_zeros();
                    self.stack.push(b);
                } else if identifier == "rotate_left" {
                    let num_positions = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_i32();

                    let b = a.rotate_left(num_positions);

                    self.stack.push(b);
                } else if identifier == "rotate_right" {
                    let num_positions = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_i32();

                    let b = a.rotate_right(num_positions);

                    self.stack.push(b);
                } else if identifier == "shift_left" {
                    let num_positions = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let b = a << num_positions;

                    self.stack.push(b);
                } else if identifier == "shift_right" {
                    let num_positions = self.stack.pop()?.to_i32();
                    let a = self.stack.pop()?.to_i32();

                    let b = a >> num_positions;

                    self.stack.push(b);
                } else if identifier == "copy" {
                    let index_from_top = self.stack.pop()?.to_usize();
                    let index_from_bottom =
                        convert_stack_index(&self.stack, index_from_top)?;

                    let Some(value) =
                        self.stack.values.get(index_from_bottom).copied()
                    else {
                        unreachable!(
                            "We computed the index from the top, based on the \
                            number of values on the stack. Since that did not \
                            result in an integer overflow, it's not possible \
                            that we ended up with an out-of-range index."
                        );
                    };

                    self.stack.push(value);
                } else if identifier == "drop" {
                    let index_from_top = self.stack.pop()?.to_usize();
                    let index_from_bottom =
                        convert_stack_index(&self.stack, index_from_top)?;

                    // This could theoretically panic, but actually won't, for
                    // the same reason that the index must be valid in the
                    // implementation of `copy`.
                    self.stack.values.remove(index_from_bottom);
                } else if identifier == "jump" {
                    let index = self.stack.pop()?.to_usize();
                    self.next_operator = index;
                } else if identifier == "jump_if" {
                    let index = self.stack.pop()?.to_usize();
                    let condition = self.stack.pop()?.to_i32();

                    if condition != 0 {
                        self.next_operator = index;
                    }
                } else if identifier == "assert" {
                    let value = self.stack.pop()?.to_i32();

                    if value == 0 {
                        return Err(Effect::AssertionFailed);
                    }
                } else if identifier == "yield" {
                    return Err(Effect::Yield);
                } else if identifier == "read" {
                    let address = self.stack.pop()?.to_usize();

                    let Some(value) = self.memory.values.get(address).copied()
                    else {
                        return Err(Effect::InvalidAddress);
                    };

                    self.stack.push(value);
                } else if identifier == "write" {
                    let value = self.stack.pop()?;
                    let address = self.stack.pop()?.to_usize();

                    if address < self.memory.values.len() {
                        self.memory.values[address] = value;
                    } else {
                        return Err(Effect::InvalidAddress);
                    }
                } else {
                    return Err(Effect::UnknownIdentifier);
                }
            }
            Operator::Integer { value } => {
                self.stack.push(*value);
            }
            Operator::Reference { name } => {
                let label =
                    self.labels.iter().find(|label| &label.name == name);

                if let Some(&Label { ref name, operator }) = label {
                    let Ok(operator) = operator.try_into() else {
                        panic!(
                            "Operator index `{operator}` of label `{name}` is \
                            out of bounds. This can only happen on platforms \
                            where the width of Rust's `usize` is wider than 32 \
                            bits, with a script that consists of at least 2^32 \
                            operators.\n\
                            \n\
                            Scripts that large seem barely realistic in the \
                            first place, more so on a 32-bit platform. At \
                            best, this is a niche use case that StackAssembly \
                            happens to not support, making this panic an \
                            acceptable outcome."
                        );
                    };
                    let operator: u32 = operator;

                    self.stack.push(operator);
                } else {
                    return Err(Effect::InvalidReference);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Operator {
    Identifier { value: String },
    Integer { value: i32 },
    Reference { name: String },
}

impl Operator {
    pub fn integer_u32(value: u32) -> Self {
        Self::Integer {
            value: i32::from_le_bytes(value.to_le_bytes()),
        }
    }
}

#[derive(Debug)]
struct Label {
    pub name: String,
    pub operator: usize,
}

fn convert_stack_index(
    stack: &Stack,
    index_from_top: usize,
) -> Result<usize, Effect> {
    let index_from_bottom = stack
        .values
        .len()
        .checked_sub(1)
        .and_then(|index| index.checked_sub(index_from_top));

    index_from_bottom.ok_or(Effect::InvalidStackIndex)
}

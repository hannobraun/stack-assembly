//! # StackAssembly
//!
//! StackAssembly is a minimalist, stack-based, assembly-like programming
//! language.
//!
//! ```text
//! 1 2 +
//! ```
//!
//! It serves as a foundation for my personal research into programming language
//! design and development. Even though I want it to be complete enough for real
//! code too, that is not its main purpose. Don't expect that it will work for
//! whatever project you might have in mind.
//!
//! Please check out the [repository on GitHub][repository] to learn more about
//! StackAssembly. This documentation, while it contains some information about
//! the language itself, is focused on how to use this library, which contains
//! the StackAssembly interpreter.
//!
//! [repository]: https://github.com/hannobraun/stack-assembly
//!
//! ## Usage
//!
//! This library contains the interpreter for StackAssembly. It is intentionally
//! minimalist. You provide a **script**, and the library gives you an API to
//! evaluate it.
//!
//! ```
//! use stack_assembly::Eval;
//!
//! let script = "1 2 +";
//!
//! let mut eval = Eval::start(script);
//! eval.run();
//!
//! assert_eq!(eval.stack.to_u32_slice(), &[3]);
//! ```
//!
//! [`Eval`] is the main entry point to the library's API.
//!
//! ### Hosts
//!
//! [`Eval`] runs scripts in a sandboxed environment. It does not provide them
//! access to the system it itself runs on, meaning StackAssembly scripts cannot
//! do much by themselves.
//!
//! A **host** is Rust code that uses this library to evaluate a StackAssembly
//! script. The host can choose to provide additional capabilities to the script
//! it runs.
//!
//! ```
//! use stack_assembly::{Effect, Eval};
//!
//! // A script that seems to want to print the value `3`.
//! let script = "
//!     3 @print jump
//!
//!     print:
//!         yield
//! ";
//!
//! // Start the evaluation and advance it until the script triggers an effect.
//! let mut eval = Eval::start(script);
//! eval.run();
//!
//! // `run` has returned, meaning an effect has been triggered. Let's make sure
//! // it's as we expect.
//! assert_eq!(eval.effect, Some(Effect::Yield));
//! let Ok(value) = eval.stack.pop() else {
//!     unreachable!("We know that the script pushes a value before yielding.");
//! };
//!
//! // The script calls `yield` at a label named `print`. I guess it expects us
//! // to print the value.
//! println!("{value:?}");
//! ```
//!
//! When the script triggers the "yield" effect, this host prints the value
//! that's currently on top of the stack.
//!
//! This is just a simple example. A more full-featured host would provide more
//! services in addition to printing values. Such a host could determine which
//! service the script means to request by inspecting which other values it put
//! on the stack, or into memory.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

mod memory;
mod stack;
mod value;

#[cfg(test)]
mod tests;

pub use self::{
    memory::Memory,
    stack::{InvalidStackIndex, Stack, StackUnderflow},
    value::Value,
};

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
/// assert_eq!(eval.stack.to_u32_slice(), &[3]);
/// ```
#[derive(Debug)]
pub struct Eval {
    /// # The operators of the evaluating script
    ///
    /// [`Eval::start`] compiles the script you provide and populates this
    /// field with the resulting operators.
    ///
    /// ## References Into This Field
    ///
    /// Various locations in the code, like the [`next_operator`] field and
    /// [`Label`]'s [`operator`] field, refer to operators in this field by
    /// their index.
    ///
    /// The host has unrestricted access to this field, and must make sure that
    /// any changes it makes to the field do not invalidate those references,
    /// or break things in other ways.
    ///
    /// [`next_operator`]: #structfield.next_operator
    /// [`operator`]: struct.Label.html#structfield.operator
    pub operators: Vec<Operator>,

    /// # The labels of the evaluating script
    ///
    /// [`Eval::start`] compiles the script you provide and populates this
    /// field with the labels it finds.
    ///
    /// The host has unrestricted access to this field, and must make sure that
    /// any change is makes do not break anything.
    pub labels: Vec<Label>,

    /// # The index of the next operator to evaluate
    ///
    /// This is an index into the [`operators`] field. On the next call to
    /// [`Eval::run`] or [`Eval::step`], evaluation continues with the operator
    /// identified by this index.
    ///
    /// When [handling an effect](#handling-effects), the host likely has to
    /// increment this field, to allow evaluation to proceed after clearing the
    /// effect.
    ///
    /// [`operators`]: #structfield.operators
    pub next_operator: usize,

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
    /// To make that possible, the host must do two things:
    ///
    /// - Clear the effect by setting this field to `None`.
    /// - Increment the [`next_operator`] field, or the same operator would
    ///   presumably trigger the same effect again.
    ///
    /// None the less, the host has full access to this field, as to not
    /// restrict any experimental or non-standard use cases.
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
    /// // To allow the script to continue, we must clear the effect and advance
    /// // to the next operator. Otherwise, `yield` would execute again
    /// // immediately, and the evaluation would make no progress.
    /// eval.effect = None;
    /// eval.next_operator += 1;
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
    /// script triggered [`Effect::Yield`], and then only do so in a
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
    /// script triggered [`Effect::Yield`], and then only do so in a
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
                if token == "#" {
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
                } else if let Ok(value) = token.parse::<i32>() {
                    Operator::Integer { value }
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
    /// return immediately. Otherwise, keep evaluating operators (starting at
    /// the one identified by [`next_operator`]) until one triggers an effect.
    ///
    /// If you need more control over the evaluation, consider using
    /// [`Eval::step`] instead.
    ///
    /// [`effect`]: #structfield.effect
    /// [`next_operator`]: #structfield.next_operator
    pub fn run(&mut self) {
        while self.effect.is_none() {
            self.step();
        }
    }

    /// # Advance the evaluation by one step
    ///
    /// If an effect is currently active (see [`effect`] field), do nothing and
    /// return immediately. Otherwise, evaluate the next operator (as defined by
    /// the [`next_operator`] field). If that triggers an effect, store that in
    /// the [`effect`] field.
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

        match operator {
            Operator::Identifier { value: identifier } => {
                if identifier == "*" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    self.stack.push(a.wrapping_mul(b));
                } else if identifier == "+" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    self.stack.push(a.wrapping_add(b));
                } else if identifier == "-" {
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

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
                    let b = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

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
                    let a = self.stack.pop()?.to_u32();
                    let b = a.count_ones();
                    self.stack.push(b);
                } else if identifier == "leading_zeros" {
                    let a = self.stack.pop()?.to_u32();
                    let b = a.leading_zeros();
                    self.stack.push(b);
                } else if identifier == "trailing_zeros" {
                    let a = self.stack.pop()?.to_u32();
                    let b = a.trailing_zeros();
                    self.stack.push(b);
                } else if identifier == "rotate_left" {
                    let num_positions = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

                    let b = a.rotate_left(num_positions);

                    self.stack.push(b);
                } else if identifier == "rotate_right" {
                    let num_positions = self.stack.pop()?.to_u32();
                    let a = self.stack.pop()?.to_u32();

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
                    let value = self.stack.get(index_from_top)?;
                    self.stack.push(value);
                } else if identifier == "drop" {
                    let index_from_top = self.stack.pop()?.to_usize();
                    self.stack.remove(index_from_top)?;
                } else if identifier == "jump" {
                    let index = self.stack.pop()?.to_usize();
                    self.next_operator = index;

                    // By default, we increment `self.next_token` below. Since
                    // we just set that to the exact value we want, we need to
                    // bypass that.
                    return Ok(());
                } else if identifier == "jump_if" {
                    let index = self.stack.pop()?.to_usize();
                    let condition = self.stack.pop()?.to_u32();

                    if condition != 0 {
                        self.next_operator = index;

                        // By default, we increment `self.next_token` below.
                        // Since we just set that to the exact value we want, we
                        // need to bypass that.
                        return Ok(());
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

        self.next_operator += 1;

        Ok(())
    }
}

/// # A token present at runtime, that is the unit of evaluation
///
/// StackAssembly scripts consist of _tokens_. Operators are the type of token
/// that have a representation at runtime and can be evaluated. This happens
/// inside of [`Eval::run`] or [`Eval::step`].
///
/// Operators are stored in [`Eval`]'s [`operators`] field. Evaluating an
/// operator may affect any of [`Eval`]'s fields, except for [`operators`]
/// itself.
///
/// The other type of tokens, beside operators, are [`Label`]s.
///
/// [`operators`]: struct.Eval.html#structfield.operators
#[derive(Debug)]
pub enum Operator {
    /// # The operator is an identifier
    ///
    /// Identifiers are the most general type of operator, syntactically
    /// speaking. Any token that can't be parsed as something more specific,
    /// ends up as an identifier.
    ///
    /// Identifiers may be known to the language, in which case evaluating them
    /// may affect the fields of [`Eval`] in whatever specific way this known
    /// identifier is supposed to.
    ///
    /// If an operator is unknown, evaluating it triggers
    /// [`Effect::UnknownIdentifier`].
    Identifier {
        /// # The value of the identifier
        ///
        /// This is how the identifier shows up in the source code.
        value: String,
    },

    /// # The operator is an integer
    ///
    /// A token parses as an integer, if it consists of base-10 digits, and the
    /// resulting number falls into the range of a signed (two's complement)
    /// 32-bit integer.
    ///
    /// Perhaps counterintuitively, this means that tokens that look like
    /// numbers but don't fall into this range, are parsed as identifiers. See
    /// [`Operator::Identifier`] for more information on those.
    ///
    /// Evaluating an integer pushes its value to the stack.
    Integer {
        /// # The value of the integer
        value: i32,
    },

    /// # The operator is a reference
    ///
    /// References are tokens that start with the character `@`, and that
    /// haven't been parsed as a [`Label`].
    ///
    /// Evaluating a reference that has the same name as a label, pushes the
    /// index of the operator that the label precedes to the stack.
    ///
    /// A reference without a matching label is invalid. Evaluating it triggers
    /// [`Effect::InvalidReference`].
    Reference {
        /// # The name of the operator that the reference refers to
        ///
        /// This name, with a preceding `@`, is how the label shows up in the
        /// source code.
        name: String,
    },
}

/// # A token with no runtime representation, that names an operator
///
/// Labels are a type of token that exist in the code, but do not have a direct
/// representation at runtime. They don't get evaluated, like operators are.
/// (Though they are accessed at runtime to resolve labels. This is just an
/// implementation detail though, and subject to change.)
///
/// Labels assign a name to the operator they precede. They are stored in
/// [`Eval`]'s [`labels`] field.
///
/// [`labels`]: struct.Eval.html#structfield.labels
#[derive(Debug)]
pub struct Label {
    /// # The name that the label assigns to the operator it precedes
    ///
    /// This name, with an added `:` at the end, is how the label shows up in
    /// the source code.
    ///
    /// References (see [`Operator::Reference`]) can be used to refer to the
    /// same operator.
    pub name: String,

    /// # The index of the operator that the label precedes
    ///
    /// This is an index into [`Eval`]'s [`operators`] field.
    ///
    /// [`operators`]: struct.Eval.html#structfield.operators
    pub operator: usize,
}

/// # An event triggered by scripts, to signal a specific condition
///
/// Evaluating an [`Operator`] can trigger an effect. Active effects are stored
/// in [`Eval`]'s [`effect`] field. Please refer to the documentation of that
/// field, for more information on effects and how to handle them.
///
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
    /// Can trigger when evaluating a reference. See [`Operator::Reference`] for
    /// more information on references.
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
    /// Can trigger when evaluating an identifier. See [`Operator::Identifier`]
    /// for more information on identifiers.
    UnknownIdentifier,

    /// # The evaluating script yields control to the host
    ///
    /// Triggers when evaluating the `yield` operator.
    Yield,
}

//! # Interpreter for the StackAssembly programming language
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
//! Please check out the [repository on GitHub][repository] for more information
//! about the language in general.
//!
//! [repository]: https://github.com/hannobraun/stack-assembly
//!
//! ## Usage
//!
//! This library contains the interpreter for StackAssembly. It is intentionally
//! minimalist. You provide a **script**, and the library gives you an API to
//! run it.
//!
//! ```
//! use stack_assembly::Eval;
//!
//! let mut eval = Eval::start("1 2 +");
//! eval.run();
//!
//! assert_eq!(eval.stack.to_u32_slice(), &[3]);
//! ```
//!
//! [`Eval`], as shown here, is the main entry point.
//!
//! ### Hosts
//!
//! [`Eval`] runs scripts in a sandboxed environment. It does not provide them
//! access to the system it runs on, meaning StackAssembly scripts cannot do
//! much by themselves.
//!
//! A **host** is Rust code that uses this library to run a StackAssembly
//! script. The host can choose to provide additional capabilities to the script
//! it runs.
//!
//! ```
//! use stack_assembly::{Effect, Eval};
//!
//! let script = "
//!     3 @print jump
//!
//!     print:
//!         yield
//! ";
//!
//! let mut eval = Eval::start(script);
//! eval.run();
//!
//! assert_eq!(eval.effect, Some(Effect::Yield));
//! let Ok(value) = eval.stack.pop() else {
//!     unreachable!("We know that the script pushes a value when yielding.");
//! };
//!
//! // The script `yield`s at a label called `print`, so I guess we're expected
//! // to print the value.
//! println!("{value:?}");
//! ```
//!
//! This host prints the value currently at the top of the stack, when the
//! script triggers the "yield" effect. This is just a simple example.
//!
//! A more full-featured host would provide additional services, and could
//! determine which service the script means to request by inspecting which
//! other values it put on the stack, or into memory.

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
/// This is the main entry point into this library's API. You can provide a
/// script to [`Eval::start`], then run it with [`Eval::run`] or [`Eval::step`].
///
/// ## Example
///
/// ```
/// use stack_assembly::Eval;
///
/// let mut eval = Eval::start("1 2 +");
/// eval.run();
///
/// assert_eq!(eval.stack.to_u32_slice(), &[3]);
/// ```
#[derive(Debug)]
pub struct Eval {
    /// # The operators of the script we're evaluating
    ///
    /// When you provide a script to [`Eval::start`], it compiles that and
    /// populates this field with the resulting operators.
    ///
    /// Various places refer to operators via their index in this field.
    /// Specifically, the [`next_operator`] field and [`Label`]'s [`index`]
    /// field to that..
    ///
    /// [`next_operator`]: #structfield.next_operator
    /// [`index`]: struct.Label.html#structfield.index
    pub operators: Vec<Operator>,

    /// # The labels of the script we're evaluating
    ///
    /// When you provide a script to [`Eval::start`], it compiles that and
    /// populates this field with all the labels it finds.
    pub labels: Vec<Label>,

    /// # The index of the next operator to evaluate
    ///
    /// This is an index into the [`operators`] field. On the next call to
    /// [`Eval::run`] or [`Eval::step`], evaluation will continue at the
    /// operator identified by this index.
    ///
    /// When [handling an effect](#handling-effects), a host may need to
    /// increment this field for evaluation to succeed.
    ///
    /// [`operators`]: #structfield.operators
    pub next_operator: usize,

    /// # The active effect, if one has triggered
    ///
    /// [`Eval::start`] always initializes this field to `None`.
    ///
    /// [`Eval::run`] and [`Eval::step`] may store an effect here, if the
    /// script has triggered one. If that is the case, you must handle the
    /// effect, if you want evaluation to continue.
    ///
    /// ## Handling Effects
    ///
    /// A host may handle effects in any way it wishes. However, since most
    /// effects signal irrecoverable error conditions, a well-behaving host
    /// would only effects that don't, ending evaluation and reporting an error
    /// otherwise.
    ///
    /// If the host decides to handle an effect, it must set this field to
    /// `None`, before evaluation can continue. Most likely, it would also need
    /// to increment the [`next_operator`] by one. Otherwise, the same operator
    /// would evaluate again, presumably triggering the same effect again.
    ///
    /// However, since the host has full control over the script, it may also
    /// decide to _not_ update [`next_operator`] and remove the conditions that
    /// caused the effect instead. This should be considered non-standard, and
    /// limited to specific and experimental hosts only.
    ///
    /// ### Example
    ///
    /// ```
    /// use stack_assembly::{Effect, Eval};
    ///
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
    /// eval.run();
    /// assert_eq!(eval.effect, Some(Effect::Yield));
    /// assert_eq!(eval.stack.to_u32_slice(), &[1]);
    ///
    /// // We want the script to continue executing normally after `yield`. So
    /// // let's clear the effect and move on to the next operator.
    /// eval.effect = None;
    /// eval.next_operator += 1;
    ///
    /// eval.run();
    /// assert_eq!(eval.effect, Some(Effect::Yield));
    /// assert_eq!(eval.stack.to_u32_slice(), &[2]);
    /// ```
    ///
    /// [`next_operator`]: #structfield.next_operator
    pub effect: Option<Effect>,

    /// # The operand stack
    ///
    /// StackAssembly's evaluation model is based on an implicit stack on which
    /// operands are stored. An operators output is pushed to that stack, and
    /// any of its inputs are popped from there.
    pub stack: Stack,

    /// # The memory
    ///
    /// StackAssembly provides a linear memory that is freely addressable per
    /// word.
    pub memory: Memory,
}

impl Eval {
    /// # Start evaluating the provided script
    ///
    /// Compile the provided script and return an `Eval` instance that is ready
    /// for evaluation. To actually evaluate any of its operators, you still
    /// need to explicitly call [`Eval::run`] or [`Eval::step`].
    pub fn start(script: &str) -> Self {
        let mut operators = Vec::new();
        let mut labels = Vec::new();

        for token in script.split_whitespace() {
            let operator = if let Some((name, "")) = token.rsplit_once(":") {
                labels.push(Label {
                    name: name.to_string(),
                    index: operators.len(),
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
    /// If you need more control over the evaluation, please consider using
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
    /// [`effect`].
    ///
    /// This function may be used for advancing the evaluation of the script in
    /// a controlled manner. If you just want to keep evaluating up until the
    /// next effect, please consider [`Eval::run`].
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
            return Err(Effect::OutOfTokens);
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

                if let Some(&Label { ref name, index }) = label {
                    let Ok(index) = index.try_into() else {
                        panic!(
                            "Operator index `{index}` of label `{name}` is out \
                            of bounds. This can only happen on platforms where \
                            the width of Rust's `usize` is wider than 32 bits, \
                            with a script that consists of at least 2^32 \
                            operators.\n\
                            \n\
                            Scripts that large seem barely realistic in the \
                            first place, more so on a 32-bit platform. At \
                            best, this is a niche use case that StackAssembly \
                            happens to not support, making this panic an \
                            acceptable outcome."
                        );
                    };
                    let index: u32 = index;

                    self.stack.push(index);
                } else {
                    return Err(Effect::InvalidReference);
                }
            }
        }

        self.next_operator += 1;

        Ok(())
    }
}

/// # An operator
///
/// Operators are a type of token that can be evaluated.
#[derive(Debug)]
pub enum Operator {
    /// # The operator is an identifier
    Identifier {
        /// # The value of the identifier
        value: String,
    },

    /// # The operator is an integer
    Integer {
        /// # The value of the integer
        value: i32,
    },

    /// # The operator is a reference
    Reference {
        /// # The name of the operator that the reference refers to
        name: String,
    },
}

/// # A label
///
/// Labels are a type of token that exist in the code, but not at runtime. They
/// assign a name to the operator they precede.
#[derive(Debug)]
pub struct Label {
    /// # The name that the label assigns to the operator it precedes
    pub name: String,

    /// # The index of the operator that the label precedes
    pub index: usize,
}

/// # An effect
#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    /// # Tried to divide by zero
    DivisionByZero,

    /// # Evaluating an operation resulted in integer overflow
    IntegerOverflow,

    /// # A memory address is out of bounds
    InvalidAddress,

    /// # Evaluated a reference that is not paired with a matching label
    InvalidReference,

    /// # An index that supposedly refers to a value on the stack doesn't
    InvalidStackIndex,

    /// # The evaluation ran out of tokens to evaluate
    OutOfTokens,

    /// # Tried popping a value from an empty stack
    StackUnderflow,

    /// # Evaluated an identifier that the language does not recognize
    UnknownIdentifier,

    /// # The evaluating script has yielded control to the host
    Yield,
}

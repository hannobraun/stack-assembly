//! # StackAssembly
//!
//! StackAssembly is a minimalist, stack-based, assembly-like programming
//! language.
//!
//! ```text
//! 0
//!
//! loop:
//!     1 +
//!     yield
//!     @loop jump
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
//! assert_eq!(eval.stack.to_i32_slice(), &[3]);
//! ```
//!
//! [`Eval`] is the main entry point to the library's API.
//!
//! ### Hosts
//!
//! [`Eval`] evaluates scripts in a sandboxed environment, not giving them any
//! access to the system it itself runs on. StackAssembly scripts by themselves
//! cannot do much.
//!
//! To change that, we need a **host**. A host is Rust code that uses this
//! library to drive the evaluation of a StackAssembly script. It can choose to
//! provide additional capabilities to the script.
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
//! // `run` has returned, meaning an effect has triggered. Let's make sure that
//! // went as expected.
//! assert_eq!(eval.effect, Some(Effect::Yield));
//! let Ok(value) = eval.stack.pop() else {
//!     unreachable!("We know that the script pushes a value before yielding.");
//! };
//!
//! // The script calls `yield` at a label named `print`. I guess it expects us
//! // to print the value then.
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

mod effect;
mod eval;
mod memory;
mod stack;
mod value;

#[cfg(test)]
mod tests;

pub use self::{
    effect::Effect,
    eval::Eval,
    memory::Memory,
    stack::{Stack, StackUnderflow},
    value::Value,
};

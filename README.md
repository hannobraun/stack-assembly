# StackAssembly

## About

StackAssembly is a minimalist, stack-based, assembly-like programming language. Here's a small taste:

```
# Push `0` to the stack.
0

increment:
    # Increment the value on the stack by `1`.
    1 +

    # If the value on the stack is smaller than `255`, jump to `increment:`.
    0 copy 255 <
    @increment
        jump_if

# Looks like we didn't jump to `increment:` that last time, so the value must be
# `255` now.
255 = assert
```

## Status

StackAssembly was a vehicle for my personal research into programming language design and implementation. I decided to go into a different direction with that though, and I consider StackAssembly to be complete now.

It is possible to to use the language for non-trivial code (as the [Snake game](snake/) in this repository proves), though this is very, very tedious.

## Documentation

### Examples

Check out the `examples/` directory to see some StackAssembly code. To run a script, follow these steps:

1. Clone this repository. Regular Git will do, but I generally recommend using [Jujutsu] instead.
2. Make sure you have a recent version of [Rust] installed on your system.
3. From within this repository, run the following command: `cargo run -- path/to/script.stack`

For example, to run the "arithmetic" example from the root directory of this repository, execute this command: `cargo run -- examples/arithmetic.stack`

For a larger-scale example of StackAssembly code, check out the [Snake game](snake/) in this repository.

[Jujutsu]: https://github.com/jj-vcs/jj
[Rust]: https://rust-lang.org/

### Interpreter API

The StackAssembly interpreter is packaged as a library. [That library's documentation][api] covers mainly the interpreter API, but also provides information about the language.

[api]: https://docs.rs/stack-assembly/latest/stack_assembly/

With a working [Rust] setup, you can also build this documentation locally, by running `cargo doc --open`.

### Additional Documentation

[The initial design document][design] can serve as an introduction to the language, though it's not fully complete. The [test suite] covers the full language in all its detail, basically serving the role of a specification, but is not as friendly to approach.

[design]: https://www.hannobraun.com/designing-stack-assembly/
[test suite]: src/tests/

## License

This project is open source, licensed under the terms of the [Zero-Clause BSD License][0BSD] (0BSD, for short). This basically means you can do anything with the code, without restrictions, but you can't hold the authors liable for any problems.

See [LICENSE.md] for details.

[0BSD]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md

# StackAssembly

## About

StackAssembly is a minimalist, stack-based, assembly-like programming language.

```
1 2 +
```

It serves as a foundation for my personal research into programming language design and development. Even though I want it to be complete enough for real code too, that is not its main purpose. Don't expect that it will work for whatever project you might have in mind.

## Status

I'm currently implementing the initial design and have not released a first version yet.

## Examples

Check out the `examples/` directory to see some StackAssembly code. To run a script, follow these steps:

1. Clone this repository. Git will do for this, but I generally recommend using [Jujutsu] instead.
2. Make sure you have a recent version of [Rust] installed on your system.
3. From within this repository, run this command: `cargo run -- path/to/script.stack`

To run the "loop" example from the root directory of this repository, for example, run this command: `cargo run -- examples/loop.stack`

[Jujutsu]: https://github.com/jj-vcs/jj
[Rust]: https://rust-lang.org/

## Documentation

Currently, [the initial design document][design] is the best available introduction into the language. The [test suite] provides a less friendly but more complete picture, basically serving the role of a specification.

[design]: https://www.hannobraun.com/designing-stack-assembly/
[test suite]: src/tests/

The StackAssembly interpreter is packaged as a library. That library's documentation documents the API, but also provides some information on the language. Right now, you must build the documentation locally to access it. With a working [Rust] setup, you can do so by running `cargo doc --open`.

[Rust]: https://rust-lang.org/

## License

This project is open source, licensed under the terms of the [Zero-Clause BSD License][0BSD] (0BSD, for short). This basically means you can do anything with the code, without restrictions, but you can't hold the authors liable for any problems.

See [LICENSE.md] for details.

[0BSD]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md

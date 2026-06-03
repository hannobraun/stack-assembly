# Snake written in StackAssembly

## About

This is a simple Snake game written in StackAssembly, together with a custom host to provide input, graphics, and access to random numbers.

The purpose of this is to serve as a non-trivial example of StackAssembly code. It is not intended to be a particularly interesting game.

## How to Run

From this directory, run `cargo run`.

This requires a bunch of stuff to work:

- A working installation of [Rust], including Cargo.
- System libraries for Vulkan and Wayland, or whatever other libraries are needed on your system to open a window and draw to it.

In case you're using [Nix], there's a `shell.nix` file that works on my system (NixOS with Wayland), but may require modifications to work on yours. If you're using [direnv], there's also an `.envrc` file that should pick up the `shell.nix` automatically.

[Rust]: https://rust-lang.org/
[Nix]: https://nixos.org/
[direnv]: https://direnv.net/

## How to Play

Control your green snake with the cursor keys. Eat the red fruit to grow. Try not to run run into your own body, or you lose and the game resets.

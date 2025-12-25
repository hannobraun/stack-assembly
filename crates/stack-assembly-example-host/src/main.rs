use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
    process, thread,
    time::Duration,
};

use clap::Parser;
use stack_assembly::{Effect, Eval, Stack};

fn main() -> io::Result<()> {
    /// Example host for the StackAssembly programming language
    #[derive(clap::Parser)]
    struct Args {
        /// The path to the script that the parser should evaluate
        path: PathBuf,
    }
    let args = Args::parse();

    let mut script = String::new();
    File::open(args.path)?.read_to_string(&mut script)?;

    let mut eval = Eval::start(&script);

    loop {
        match eval.run() {
            Effect::OutOfOperators => {
                eprintln!();
                eprintln!("Evaluation has finished.");

                print_stack(&eval.stack);

                process::exit(0);
            }
            Effect::Yield => {
                print_stack(&eval.stack);
                eval.effect = None;

                // Let's not execute scripts that fast, to give the user a
                // chance to read the output.
                thread::sleep(Duration::from_millis(20));

                continue;
            }
            effect => {
                eprintln!();
                eprintln!("Script triggered effect: {effect:?}");

                print_stack(&eval.stack);

                process::exit(2);
            }
        }
    }
}

fn print_stack(stack: &Stack) {
    let mut values = stack.values.iter().peekable();

    print!("Stack: ");

    while let Some(value) = values.next() {
        print!("{value:?}");

        if values.peek().is_some() {
            print!(" ");
        }
    }

    println!();
}

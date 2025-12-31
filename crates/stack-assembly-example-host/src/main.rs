use std::{fs::File, io::Read, path::PathBuf, process, thread, time::Duration};

use anyhow::Context;
use clap::Parser;
use stack_assembly::{Effect, Eval, OperandStack};

fn main() -> anyhow::Result<()> {
    /// Example host for the StackAssembly programming language
    #[derive(clap::Parser)]
    struct Args {
        /// The path to the script that the parser should evaluate
        path: PathBuf,
    }
    let args = Args::parse();

    let mut script = String::new();
    File::open(args.path)
        .context("Opening script file.")?
        .read_to_string(&mut script)
        .context("Reading from script file.")?;

    let mut eval = Eval::start(&script);

    loop {
        match eval.run() {
            Effect::OutOfOperators | Effect::Return => {
                eprintln!();
                eprintln!("Evaluation has finished.");

                print_operand_stack(&eval.operand_stack);

                process::exit(0);
            }
            Effect::Yield => {
                print_operand_stack(&eval.operand_stack);
                eval.effect = None;

                // Let's not execute scripts that fast, to give the user a
                // chance to read the output.
                thread::sleep(Duration::from_millis(20));

                continue;
            }
            effect => {
                eprintln!();
                eprintln!("Script triggered effect: {effect:?}");

                print_operand_stack(&eval.operand_stack);

                process::exit(2);
            }
        }
    }
}

fn print_operand_stack(operand_stack: &OperandStack) {
    let mut values = operand_stack.values.iter().peekable();

    print!("Operand Stack: ");

    while let Some(value) = values.next() {
        print!("{value:?}");

        if values.peek().is_some() {
            print!(" ");
        }
    }

    println!();
}

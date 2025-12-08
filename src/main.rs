use std::{
    env,
    fs::File,
    io::{self, Read},
    process, thread,
    time::Duration,
};

use stack_assembly::{Effect, Eval, Stack};

fn main() -> io::Result<()> {
    let mut args = env::args();

    let Some(_executable) = args.next() else {
        panic!("Expected the first argument to be the path to the executable.");
    };

    let Some(path) = args.next() else {
        print_usage_and_exit("Expected a single argument; found none.");
    };
    let None = args.next() else {
        print_usage_and_exit("Expected a single argument; found multiple.");
    };

    let mut script = String::new();
    File::open(path)?.read_to_string(&mut script)?;

    let mut eval = Eval::start(&script);

    loop {
        eval.run();

        let Some(effect) = &eval.effect else {
            unreachable!(
                "Script must have triggered effect, or `Eval::run` would not \
                have returned."
            );
        };

        match effect {
            Effect::OutOfOperators => {
                eprintln!();
                eprintln!("Evaluation has finished.");

                print_stack(&eval.stack);

                process::exit(0);
            }
            Effect::Yield => {
                print_stack(&eval.stack);
                eval.effect = None;

                // Let's not execute scripts that fast. Give the user a chance
                // to read the output.
                thread::sleep(Duration::from_millis(20));

                continue;
            }
            effect => {
                eprintln!("Script triggered effect: {effect:?}");
                eprintln!("Stack at end of script: {:?}", eval.stack);
                process::exit(2);
            }
        }
    }
}

fn print_usage_and_exit(error: &str) -> ! {
    eprintln!();
    eprintln!("{error}");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("cargo run -- path/to/script.stack");

    process::exit(1);
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

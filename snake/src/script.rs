use std::{
    fs::File,
    io::Read,
    ops::Range,
    path::Path,
    time::{Duration, Instant},
};

use crossbeam_channel::{
    Receiver, RecvError, SendError, Sender, after, bounded, select, unbounded,
};
use notify::{RecursiveMode, Watcher};
use stack_assembly::{Effect, Eval, Script, Value};

use crate::{BYTES_PER_PIXEL, Input, PIXELS_SIZE_BYTES, Pixels};

mod memory {
    use crate::PIXELS_SIZE;

    pub struct Region {
        pub start: usize,
        pub size: usize,
    }

    impl Region {
        pub const fn end(&self) -> usize {
            self.start + self.size
        }

        pub fn iter(&self) -> impl Iterator<Item = usize> {
            self.start..self.end()
        }
    }

    pub const PIXELS: Region = Region {
        start: 0,
        size: PIXELS_SIZE,
    };
    pub const TIMER: Region = Region {
        start: PIXELS.end(),
        size: 1,
    };
    pub const INPUT_METADATA: Region = Region {
        start: TIMER.end(),
        size: 3,
    };
    pub const INPUT: Region = Region {
        start: INPUT_METADATA.end(),
        size: 8,
    };
}

pub fn run(
    input_rx: Receiver<Input>,
    pixels_tx: Sender<Pixels>,
) -> anyhow::Result<()> {
    let path = Path::new("snake.stack");

    let (notify_tx, notify_rx) = unbounded();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    let mut run = 0;
    let (mut script, mut eval, mut source) = load(path)?;

    let mut now = Instant::now();

    loop {
        let time_passed_ms = now.elapsed().as_millis() as u32;
        now = Instant::now();

        eval.memory.values[memory::TIMER.start] = Value::from(
            eval.memory.values[memory::TIMER.start].to_u32() + time_passed_ms,
        );

        for input in input_rx.try_iter() {
            let read_index = memory::INPUT_METADATA.start + 1;
            let write_index = read_index + 1;

            let [read, write] = [read_index, write_index]
                .map(|i| eval.memory.values[i].to_u32() as usize);

            if write - read >= memory::INPUT.size {
                // Ring buffer is full. Throw away the input event.
                continue;
            }

            eval.memory.values
                [memory::INPUT.start + (write & (memory::INPUT.size - 1))] =
                Value::from(input as u32);
            eval.memory.values[write_index] =
                Value::from((write as u32).wrapping_add(1));
        }

        let (effect, operator) = eval.run(&script);

        match effect {
            Effect::Yield => {
                let Ok(parameter) = eval.operand_stack.pop() else {
                    panic!("Expected `yield` to have a parameter.");
                };

                match parameter.to_u32() {
                    0 => {
                        let mut pixels = [0; PIXELS_SIZE_BYTES];
                        for i in memory::PIXELS.iter() {
                            let pixel =
                                eval.memory.values[i].to_u32().to_be_bytes();
                            pixels[i * BYTES_PER_PIXEL
                                ..i * BYTES_PER_PIXEL + BYTES_PER_PIXEL]
                                .copy_from_slice(&pixel);
                        }

                        // `pixels_tx` is bounded, with capacity zero, so this
                        // will block until the pixels are being drawn, tying
                        // the frame rate of the script to the frame rate of the
                        // I/O.
                        if let Err(SendError(_)) = pixels_tx.send(pixels) {
                            // Other end has hung up, which means we need to
                            // quit too.
                            return Ok(());
                        }
                    }
                    1 => {
                        let value = fastrand::u32(..);
                        eval.operand_stack.push(value);
                    }
                    2 => {
                        eprintln!("{stack:?}", stack = eval.operand_stack);
                    }
                    3 => {
                        eprintln!("{memory:?}", memory = eval.memory);
                    }
                    parameter => {
                        panic!("Unknown `yield` parameter: {parameter}");
                    }
                }

                eval.clear_effect();
                continue;
            }
            effect => {
                let Ok(op_range) = script.map_operator_to_source(&operator)
                else {
                    unreachable!(
                        "This operator index (`{operator}`) was returned from \
                        `run`, which means it must point to an operator."
                    );
                };

                let line = op_range_to_line(&source, &op_range);

                eprintln!("{run}: Script triggered effect: {effect:?}");
                eprintln!("\tat {}: {}", line, &source[op_range]);

                for operator in eval.call_stack() {
                    let Ok(op_range) = script.map_operator_to_source(&operator)
                    else {
                        unreachable!(
                            "This operator index was returned from \
                            `call_stack`, which means it must point to an \
                            operator."
                        );
                    };

                    let line = op_range_to_line(&source, &op_range);

                    eprintln!("\tcalled from: {}: {}", line, &source[op_range]);
                }

                match wait_for_change(&mut run, &notify_rx, &input_rx)? {
                    WaitForChangeOutcome::ScriptHasChanged => {
                        (script, eval, source) = load(path)?;
                        continue;
                    }
                    WaitForChangeOutcome::InputReceived { input } => {
                        // Right now, the only way to recover from an unhandled
                        // effect is to restart the evaluation. The new
                        // evaluation has no use for input from the old one, so
                        // we can just throw it away.
                        let _ = input;
                    }
                    WaitForChangeOutcome::MustQuit => {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn load(path: &Path) -> anyhow::Result<(Script, Eval, String)> {
    let mut source = String::new();
    File::open(path)?.read_to_string(&mut source)?;

    let script = Script::compile(&source);
    let mut eval = Eval::new();

    eval.memory.values = vec![Value::from(0); memory::INPUT.end() * 4];

    Ok((script, eval, source))
}

fn wait_for_change(
    run: &mut u64,
    notify_rx: &Receiver<notify::Result<notify::Event>>,
    input_rx: &Receiver<Input>,
) -> anyhow::Result<WaitForChangeOutcome> {
    // We don't intend to ever trigger a timeout using this channel. We might
    // overwrite the receiver later though.
    let (_timeout_tx, mut timeout_rx) = bounded::<Instant>(0);

    let mut event_received = false;

    loop {
        select! {
            recv(notify_rx) -> event => {
                let event = event??;

                if event_received {
                    // We have already received an event and are currently
                    // debouncing it.
                    continue;
                }

                let notify::EventKind::Modify(_) = event.kind else {
                    // We are only interested in changes to the script. Ignore.
                    continue;
                };

                // This is a change to the script, which we are interested in.
                // Set off the timer, so we can debounce the event before
                // returning.
                event_received = true;
                timeout_rx = after(Duration::from_millis(20));
            }
            recv(input_rx) -> message => {
                let outcome = match message {
                    Ok(input) => {
                        WaitForChangeOutcome::InputReceived { input }
                    }
                    Err(RecvError) => {
                        // Sender has been dropped. We're done.
                        WaitForChangeOutcome::MustQuit
                    }
                };

                return Ok(outcome);
            }
            recv(timeout_rx) -> _ => {
                *run += 1;
                return Ok(WaitForChangeOutcome::ScriptHasChanged);
            }
        }
    }
}

enum WaitForChangeOutcome {
    ScriptHasChanged,
    InputReceived { input: Input },
    MustQuit,
}

fn op_range_to_line(source: &str, op_range: &Range<usize>) -> usize {
    let num_line_breaks = source.chars().filter(|&ch| ch == '\n').count();

    let line_from_0 = source
        .char_indices()
        .filter_map(|(index, ch)| (ch == '\n').then_some(index))
        .enumerate()
        .find_map(|(line_from_0, index)| {
            (op_range.start < index).then_some(line_from_0)
        });

    line_from_0.unwrap_or(num_line_breaks) + 1
}

use std::{panic, thread};

use crossbeam_channel::{bounded, unbounded};

mod io;
mod script;

#[derive(Debug)]
#[repr(u8)]
enum Input {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

const GRID_SIZE: usize = 32;
const PIXELS_SIZE: usize = GRID_SIZE * GRID_SIZE;
const BYTES_PER_PIXEL: usize = 4;
const PIXELS_SIZE_BYTES: usize = PIXELS_SIZE * BYTES_PER_PIXEL;

type Pixels = [u8; PIXELS_SIZE_BYTES];

fn main() -> anyhow::Result<()> {
    let (input_tx, input_rx) = unbounded();
    let (pixels_tx, pixels_rx) = bounded(0);

    let handle = thread::spawn(|| script::run(input_rx, pixels_tx));
    io::start_and_wait(input_tx, pixels_rx)?;

    match handle.join() {
        Ok(result) => result?,
        Err(err) => {
            panic::resume_unwind(err);
        }
    }

    Ok(())
}

use std::{num::NonZeroU32, sync::Arc};

use anyhow::anyhow;
use crossbeam_channel::{Receiver, SendError, Sender, TryRecvError};
use softbuffer::{SoftBufferError, Surface};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, OwnedDisplayHandle},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{BYTES_PER_PIXEL, GRID_SIZE, Input, PIXELS_SIZE_BYTES, Pixels};

pub fn start_and_wait(
    input_tx: Sender<Input>,
    pixels_rx: Receiver<Pixels>,
) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut app = WindowApp {
        window: None,
        renderer: None,
        input_tx,
        pixels_rx,
        pixels: [0; PIXELS_SIZE_BYTES],
    };
    event_loop.run_app(&mut app)?;

    Ok(())
}

struct WindowApp {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    input_tx: Sender<Input>,
    pixels_rx: Receiver<Pixels>,

    /// # A copy of the pixels to render
    ///
    /// It seems sensible to assume that we only need to draw when the pixels
    /// have changed. If that were the case, we wouldn't need this field and
    /// could always just react to new pixels arriving through the channel.
    ///
    /// This would be an incorrect assumption though. We always need to be able
    /// to draw, even if nothing changed about the pixels, to react to the
    /// window being resized. When that happens, we can redraw using the pixels
    /// stored in this field.
    pixels: Pixels,
}

impl WindowApp {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        let window = {
            let window = event_loop.create_window(
                WindowAttributes::default().with_title("Snake | StackAssembly"),
            )?;

            Arc::new(window)
        };

        let surface = {
            let context =
                softbuffer::Context::new(event_loop.owned_display_handle())
                    .map_err(|err| {
                        anyhow!("Failed to create context: {err:?}")
                    })?;

            Surface::new(&context, window.clone())
                .map_err(|err| anyhow!("Failed to create surface: {err:?}"))?
        };

        self.window = Some(window);
        self.renderer = Some(Renderer { surface });

        Ok(())
    }

    fn handle_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: WindowEvent,
    ) -> anyhow::Result<()> {
        let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer)
        else {
            return Ok(());
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                let input = match key {
                    NamedKey::ArrowUp => Input::Up,
                    NamedKey::ArrowLeft => Input::Left,
                    NamedKey::ArrowDown => Input::Down,
                    NamedKey::ArrowRight => Input::Right,

                    _ => {
                        return Ok(());
                    }
                };

                if let Err(SendError(_)) = self.input_tx.send(input) {
                    // The other end has hung up, but we don't have to do
                    // anything about it. We already handle this case when
                    // receiving pixels.
                }
            }
            WindowEvent::RedrawRequested => {
                // We must check the channel for updated pixels exactly here. We
                // could also do it elsewhere, like the top of this function,
                // where it would happen on any event, but that would be wrong.
                //
                // Only this location is correct, because the channel is
                // bounded, meaning the sender blocks until we receive. This is
                // deliberate, to sync the sender to the redraw rate of the
                // display.
                //
                // If we received from the channel elsewhere, we would not be
                // syncing the sender to the redraw rate.
                loop {
                    match self.pixels_rx.try_recv() {
                        Ok(pxs) => {
                            self.pixels = pxs;
                        }
                        Err(TryRecvError::Empty) => {
                            break;
                        }
                        Err(TryRecvError::Disconnected) => {
                            event_loop.exit();
                            return Ok(());
                        }
                    }
                }

                renderer
                    .draw(window, self.pixels)
                    .map_err(|err| anyhow!("Failed to draw pixels: {err:?}"))?;
            }

            _ => {}
        }

        Ok(())
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(err) = self.init(event_loop) {
            eprintln!("Error creating window: {err:?}");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        if let Err(err) = self.handle_event(event_loop, event) {
            eprintln!("Error handling event: {err:?}");
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let Some(window) = &self.window else {
            return;
        };

        // We want to redraw on every frame. Otherwise, the window will only
        // redraw in response to some events, like losing or gaining focus.
        window.request_redraw();
    }
}

struct Renderer {
    surface: Surface<OwnedDisplayHandle, Arc<Window>>,
}

impl Renderer {
    pub fn draw(
        &mut self,
        window: &Window,
        pixels: [u8; 4096],
    ) -> Result<(), SoftBufferError> {
        let size = window.inner_size();
        let [Some(width), Some(height)] =
            [size.width, size.height].map(NonZeroU32::new)
        else {
            return Ok(());
        };
        self.surface.resize(width, height)?;

        let [Ok(width_usize), Ok(height_usize)]: [Result<usize, _>; 2] =
            [width, height].map(NonZeroU32::get).map(TryInto::try_into)
        else {
            unreachable!("Surface dimensions can be represented as `usize`.");
        };

        let mut buffer = self.surface.buffer_mut()?;

        for (target_index, target) in buffer.iter_mut().enumerate() {
            let target_x = target_index % width_usize;
            let target_y = target_index / width_usize;

            let source_x = target_x * GRID_SIZE / width_usize;
            let source_y = target_y * GRID_SIZE / height_usize;

            let source_i = (source_y * GRID_SIZE + source_x) * BYTES_PER_PIXEL;

            let [r, g, b, _] = pixels[source_i..source_i + BYTES_PER_PIXEL]
            else {
                unreachable!(
                    "Four-element slice destructures into 4 elements."
                );
            };

            let source = u32::from_be_bytes([0, r, g, b]);

            *target = source;
        }

        buffer.present()?;

        Ok(())
    }
}

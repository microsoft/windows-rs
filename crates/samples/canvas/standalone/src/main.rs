#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::*;
use windows_window::*;

struct Graphics {
    _device: GpuDevice,
    chain: SwapChain,
}

struct State {
    graphics: Option<Graphics>,
    width: u32,
    height: u32,
    dirty: bool,
}

impl State {
    fn resize(&mut self, width: i32, height: i32) {
        self.width = width.max(0) as u32;
        self.height = height.max(0) as u32;
        self.dirty = true;
    }

    fn render(&mut self, window: &Window) -> Result<bool> {
        if self.graphics.is_none() && self.width != 0 && self.height != 0 {
            let device = GpuDevice::new()?;
            let chain = device.create_swap_chain_for_window(window, self.width, self.height)?;
            self.graphics = Some(Graphics {
                _device: device,
                chain,
            });
        }
        self.render_existing()?;
        Ok(self.graphics.is_none() && self.dirty && self.width != 0 && self.height != 0)
    }

    fn render_existing(&mut self) -> Result<()> {
        if !self.dirty || self.width == 0 || self.height == 0 {
            return Ok(());
        }
        self.dirty = false;

        let Some(graphics) = self.graphics.as_mut() else {
            self.dirty = true;
            return Ok(());
        };
        if (graphics.chain.width() != self.width || graphics.chain.height() != self.height)
            && let Err(error) = graphics.chain.resize(self.width, self.height)
        {
            if !is_device_lost(error.code()) {
                return Err(error);
            }
            self.graphics = None;
            self.dirty = true;
            return Ok(());
        }

        match draw_frame(&mut graphics.chain) {
            Ok(true) => Ok(()),
            Ok(false) => {
                self.graphics = None;
                self.dirty = true;
                Ok(())
            }
            Err(error) if is_device_lost(error.code()) => {
                self.graphics = None;
                self.dirty = true;
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
}

fn draw_frame(chain: &mut SwapChain) -> Result<bool> {
    let width = chain.width() as f32;
    let height = chain.height() as f32;
    let session = chain.begin_draw()?;
    session.clear(ColorF::DARK_SLATE_BLUE);
    let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    let r = width.min(height) * 0.3;

    session.fill_ellipse(
        &Ellipse::circle(Vector2::new(width / 2.0, height / 2.0), r),
        &brush,
    );

    brush.set_color(ColorF::WHITE);

    let format = TextFormat::new("Segoe UI", 24.0)?
        .with_alignment(TextAlignment::Center)
        .with_paragraph_alignment(ParagraphAlignment::Center);

    session.draw_text(
        "Hello from windows-canvas!",
        &format,
        &Rect::new(0.0, 0.0, width, height),
        &brush,
    );

    drop(session);
    chain.present()
}

fn main() -> Result<()> {
    let state = Rc::new(RefCell::new(State {
        graphics: None,
        width: 0,
        height: 0,
        dirty: true,
    }));

    let resize = Rc::clone(&state);
    let window = Window::new("Canvas Standalone")
        .client_size(800, 600)
        .on_resize(move |width, height| {
            let mut state = resize.borrow_mut();
            state.resize(width, height);
            if state.graphics.is_some() {
                state.render_existing().unwrap();
            }
        })
        .create()?;

    let (width, height) = window.client_size();
    state.borrow_mut().resize(width, height);

    run_with(|| state.borrow_mut().render(&window))
}

#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::f32::consts::TAU;
use std::rc::Rc;
use windows_canvas::Ellipse as CanvasEllipse;
use windows_canvas::*;
use windows_reactor::*;

const SIZE: f32 = 320.0;

#[derive(Default)]
struct Graphics {
    device: Option<GpuDevice>,
    surface: Option<CanvasImageSource>,
}

struct Sample {
    count: u32,
    scale: f64,
    graphics: Rc<RefCell<Graphics>>,
    image: ElementRef<Image>,
}

#[derive(Clone)]
enum Message {
    Add,
    Remove,
    Scale(f64),
}

impl Component for Sample {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            count: 6,
            scale: 1.0,
            graphics: Rc::new(RefCell::new(Graphics::default())),
            image: ElementRef::new(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Add => self.count += 1,
            Message::Remove => self.count = self.count.saturating_sub(1),
            Message::Scale(scale) => self.scale = scale,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas Image Source");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        let image = self.image.clone();
        let sender = context.sender();
        context.use_effect_guard("image-scale", (), move || {
            image.observe_rasterization_scale(move |scale| {
                sender.send(Message::Scale(scale));
            })
        });

        let graphics = Rc::clone(&self.graphics);
        let image = self.image.clone();
        let count = self.count;
        let scale = self.scale;
        context.use_effect("draw", (count, scale), move || {
            if let Err(error) = redraw(&graphics, &image, count, scale as f32) {
                eprintln!("failed to redraw surface: {error}");
            }
            None
        });

        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
            .margin(Thickness::uniform(16.0))
            .children((
                TextBlock::new()
                    .text("On-demand surface - redraws only when the count changes:")
                    .grid_row(0),
                Image::new()
                    .element_ref(&self.image)
                    .width(SIZE as f64)
                    .height(SIZE as f64)
                    .grid_row(1),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .grid_row(2)
                    .children((
                        Button::new()
                            .on_click(context.message(Message::Add))
                            .content("Add dot"),
                        Button::new()
                            .on_click(context.message(Message::Remove))
                            .content("Remove dot"),
                        format!("{count} dots"),
                    )),
            ))
    }
}

fn redraw(
    graphics: &RefCell<Graphics>,
    image: &ElementRef<Image>,
    count: u32,
    scale: f32,
) -> Result<()> {
    let mut graphics = graphics.borrow_mut();
    if graphics
        .surface
        .as_ref()
        .is_some_and(|surface| surface.scale() != scale)
    {
        graphics.surface = None;
    }

    for attempt in 0..2 {
        if graphics.device.is_none() {
            graphics.device = Some(GpuDevice::new_or_warp()?);
        }
        if graphics.surface.is_none() {
            let surface =
                CanvasImageSource::new(graphics.device.as_ref().unwrap(), SIZE, SIZE, scale)?;
            let _ = surface.attach(image);
            graphics.surface = Some(surface);
        }
        match graphics
            .surface
            .as_ref()
            .unwrap()
            .draw(ColorF::CORNFLOWER_BLUE, |session| draw_dial(session, count))?
        {
            true => return Ok(()),
            false if attempt == 0 => {
                let _ = image.request_set_native_source(None, |_| {});
                graphics.surface = None;
                graphics.device = None;
            }
            false => {
                return Err(windows_core::Error::from_hresult(windows_core::HRESULT(
                    0x80004005_u32 as _,
                )));
            }
        }
    }
    unreachable!()
}

fn draw_dial(session: &DrawingSession, count: u32) -> Result<()> {
    let center = Vector2::new(SIZE / 2.0, SIZE / 2.0);
    let radius = SIZE / 2.0 - 44.0;
    let hub = session.create_solid_brush(ColorF::WHITE)?;
    session.fill_ellipse(&CanvasEllipse::circle(center, 16.0), &hub);

    let count = count.max(1);
    for i in 0..count {
        let phase = i as f32 / count as f32;
        let angle = phase * TAU - TAU / 4.0;
        let position = Vector2::new(
            center.x + angle.cos() * radius,
            center.y + angle.sin() * radius,
        );
        let brush = session.create_solid_brush(ColorF::new(
            0.5 + 0.5 * (phase * TAU).cos(),
            0.5 + 0.5 * ((phase + 0.33) * TAU).cos(),
            0.5 + 0.5 * ((phase + 0.66) * TAU).cos(),
            1.0,
        ))?;
        session.fill_ellipse(&CanvasEllipse::circle(position, 20.0), &brush);
    }
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

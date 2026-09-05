#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::*;
use windows_canvas::{Ellipse as CanvasEllipse, Rect as CanvasRect};
use windows_reactor::*;

const COLS: usize = 4;
const ROWS: usize = 3;
const TILES: usize = COLS * ROWS;
const TILE: f32 = 132.0;

#[derive(Default)]
struct Graphics {
    device: Option<GpuDevice>,
    surfaces: Vec<CanvasImageSource>,
}

struct Sample {
    scale: f64,
    graphics: Rc<RefCell<Graphics>>,
    images: Vec<ElementRef<Image>>,
}

impl Component for Sample {
    type Input = ();
    type Message = f64;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            scale: 1.0,
            graphics: Rc::new(RefCell::new(Graphics::default())),
            images: (0..TILES).map(|_| ElementRef::new()).collect(),
        }
    }

    fn update(&mut self, scale: f64, _context: &ComponentContext<Self>) {
        self.scale = scale;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas Shared Device");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));

        let image = self.images[0].clone();
        let sender = context.sender();
        context.use_effect_guard("image-scale", (), move || {
            image.observe_rasterization_scale(move |scale| {
                sender.send(scale);
            })
        });

        let graphics = Rc::clone(&self.graphics);
        let images = self.images.clone();
        let scale = self.scale;
        context.use_effect("draw", scale, move || {
            if let Err(error) = rebuild(&graphics, &images, scale as f32) {
                eprintln!("failed to build surfaces: {error}");
            }
            None
        });

        let rows = self
            .images
            .chunks(COLS)
            .enumerate()
            .map(|row| {
                let (row_index, row) = row;
                KeyedView::new(
                    row_index,
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(8.0)
                        .children((View::keyed_fragment(row.iter().enumerate().map(
                            |(column, image)| {
                                KeyedView::new(
                                    column,
                                    Image::new()
                                        .element_ref(image)
                                        .width(TILE as f64)
                                        .height(TILE as f64),
                                )
                            },
                        )),)),
                )
            })
            .collect::<Vec<_>>();

        StackPanel::new()
            .spacing(12.0)
            .margin(Thickness::uniform(16.0))
            .children((
                format!("{TILES} on-demand surfaces - all sharing one GpuDevice:"),
                StackPanel::new()
                    .spacing(8.0)
                    .children((View::keyed_fragment(rows),)),
            ))
    }
}

fn rebuild(graphics: &RefCell<Graphics>, images: &[ElementRef<Image>], scale: f32) -> Result<()> {
    let mut graphics = graphics.borrow_mut();
    if graphics
        .surfaces
        .first()
        .is_some_and(|surface| surface.scale() != scale)
    {
        graphics.surfaces.clear();
    }

    for attempt in 0..2 {
        if graphics.device.is_none() {
            graphics.device = Some(GpuDevice::new_or_warp()?);
        }
        if graphics.surfaces.is_empty() {
            let mut surfaces = Vec::with_capacity(TILES);
            let mut lost = false;
            for (index, image) in images.iter().enumerate() {
                let surface =
                    CanvasImageSource::new(graphics.device.as_ref().unwrap(), TILE, TILE, scale)?;
                if !surface.draw(background(index), |session| draw_tile(session, index))? {
                    lost = true;
                    break;
                }
                let _ = surface.attach(image);
                surfaces.push(surface);
            }
            if lost {
                for image in images {
                    let _ = image.request_set_native_source(None, |_| {});
                }
                graphics.device = None;
                if attempt == 0 {
                    continue;
                }
                return Err(windows_core::Error::from_hresult(windows_core::HRESULT(
                    0x80004005_u32 as _,
                )));
            }
            graphics.surfaces = surfaces;
        }
        return Ok(());
    }
    unreachable!()
}

fn background(i: usize) -> ColorF {
    let t = i as f32 / TILES as f32;
    ColorF::new(0.12 + 0.10 * t, 0.14, 0.30 - 0.12 * t, 1.0)
}

fn draw_tile(session: &DrawingSession, i: usize) -> Result<()> {
    let center = Vector2::new(TILE / 2.0, TILE / 2.0);
    let radius = TILE * 0.28;
    let brush = session.create_solid_brush(ColorF::WHITE)?;

    match i % 4 {
        0 => session.fill_ellipse(&CanvasEllipse::circle(center, radius), &brush),
        1 => session.fill_rect(
            &CanvasRect::new(
                center.x - radius,
                center.y - radius,
                center.x + radius,
                center.y + radius,
            ),
            &brush,
        ),
        2 => session.draw_ellipse(&CanvasEllipse::circle(center, radius), &brush, 8.0),
        _ => {
            let arm = radius;
            let thick = radius * 0.34;
            session.fill_rect(
                &CanvasRect::new(
                    center.x - arm,
                    center.y - thick,
                    center.x + arm,
                    center.y + thick,
                ),
                &brush,
            );
            session.fill_rect(
                &CanvasRect::new(
                    center.x - thick,
                    center.y - arm,
                    center.x + thick,
                    center.y + arm,
                ),
                &brush,
            );
        }
    }

    let format = TextFormat::new("Segoe UI", 16.0)?.with_alignment(TextAlignment::Center);
    session.draw_text(
        &format!("{i}"),
        &format,
        &CanvasRect::new(0.0, TILE - 28.0, TILE, TILE),
        &brush,
    );
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

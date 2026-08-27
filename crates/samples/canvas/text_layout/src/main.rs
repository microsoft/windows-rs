#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows_canvas::Brush;
use windows_canvas::*;
use windows_reactor::{
    App, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};

const TEXT: &str = "TextLayout shapes this paragraph and measures it, then draws it without \
re-shaping. Resize the window and the text reflows to fit the box, redrawing only when the size \
changes, not every frame.";

const MARGIN: f32 = 40.0;

struct Resources {
    layout: TextLayout,
    metrics: TextMetrics,
    label: TextLayout,
    outline: Brush,
    accent: Brush,
    white: Brush,
}

struct Sample {
    resources: Rc<RefCell<Option<Resources>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            resources: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Text Layout");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let resources = Rc::clone(&self.resources);
        canvas(move |ctx| {
            ctx.clear(ColorF::from_rgb8(16, 20, 28));

            if ctx.device_changed() {
                let format =
                    TextFormat::new("Segoe UI", 28.0)?.with_word_wrapping(WordWrapping::Wrap);
                let layout = TextLayout::new(
                    TEXT,
                    &format,
                    ctx.width - 2.0 * MARGIN,
                    ctx.height - 2.0 * MARGIN,
                )?;
                let metrics = layout.metrics();

                let readout = format!(
                    "{} lines  -  {:.0} x {:.0} px",
                    metrics.line_count, metrics.width, metrics.height
                );
                let label_format = TextFormat::new("Consolas", 16.0)?;
                let label =
                    TextLayout::new(&readout, &label_format, ctx.width - 2.0 * MARGIN, MARGIN)?;

                *resources.borrow_mut() = Some(Resources {
                    layout,
                    metrics,
                    label,
                    outline: ctx.create_solid_brush(ColorF::from_rgb8(60, 70, 90))?,
                    accent: ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?,
                    white: ctx.create_solid_brush(ColorF::WHITE)?,
                });
            }

            let res = resources.borrow();
            let res = res.as_ref().unwrap();

            let box_rect = Rect::new(MARGIN, MARGIN, ctx.width - MARGIN, ctx.height - MARGIN);
            ctx.draw_rect(&box_rect, &res.outline, 1.0);
            ctx.draw_rect(
                &res.metrics.bounds().offset(MARGIN, MARGIN),
                &res.accent,
                1.5,
            );
            ctx.draw_text_layout(Vector2::new(MARGIN, MARGIN), &res.layout, &res.white);
            ctx.draw_text_layout(Vector2::new(MARGIN, 8.0), &res.label, &res.accent);
            Ok(())
        })
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

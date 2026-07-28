//! Shaping a paragraph once with `TextLayout` and redrawing it on demand.
//!
//! `TextFormat` and `TextLayout` are device-independent, so the paragraph is
//! shaped once and re-shaped only when the window resizes - never per frame. The
//! shaped layout, its measurements, and the brushes live in a `use_ref` and are
//! rebuilt only when `device_changed` reports a resize or device loss. Compare
//! with `draw_text`, which re-shapes its string on every call.

#![windows_subsystem = "windows"]

use windows_canvas::Brush;
use windows_canvas::*;
use windows_reactor::*;

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

fn app(cx: &mut RenderCx) -> Element {
    let res = cx.use_ref::<Option<Resources>>(None);

    canvas(move |ctx| {
        ctx.clear(ColorF::from_rgb8(16, 20, 28));

        if ctx.device_changed() {
            let format = TextFormat::new("Segoe UI", 28.0)?.with_word_wrapping(WordWrapping::Wrap);
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
            let label = TextLayout::new(&readout, &label_format, ctx.width - 2.0 * MARGIN, MARGIN)?;

            *res.borrow_mut() = Some(Resources {
                layout,
                metrics,
                label,
                outline: ctx.create_solid_brush(ColorF::from_rgb8(60, 70, 90))?,
                accent: ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?,
                white: ctx.create_solid_brush(ColorF::WHITE)?,
            });
        }

        let res = res.borrow();
        let res = res.as_ref().unwrap();

        // The box the text fills, the measured bounds of the inked text, the
        // paragraph, and a line-count readout.
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
    .into()
}

fn main() -> Result<()> {
    App::new()
        .title("Text Layout")
        .backdrop(Backdrop::Mica)
        .render(app)
}

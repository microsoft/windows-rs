#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{TextFormat, TextLayout};

const TEXT: &str = "Each line has a baseline and a height, and each cluster has bounds. A \
bigger word makes its line taller. Resize the window to reflow the text.";

const MARGIN: f32 = 32.0;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let frame = ctx.create_solid_brush(ColorF::from_rgb8(60, 70, 90))?;
    let cluster = ctx.create_solid_brush(ColorF::new(0.3, 0.8, 0.4, 0.5))?;
    let baseline = ctx.create_solid_brush(ColorF::new(1.0, 0.6, 0.2, 1.0))?;
    let bounds = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    let origin = Vector2::new(MARGIN, MARGIN);
    let format = TextFormat::new("Segoe UI", 28.0)?;
    let mut layout = TextLayout::new(
        TEXT,
        &format,
        (ctx.width - 2.0 * MARGIN).min(640.0),
        ctx.height - 3.0 * MARGIN,
    )?;
    let bigger = TEXT.find("bigger").unwrap();
    layout.set_font_size(bigger..bigger + "bigger".len(), 48.0);

    // The layout box, and the bounds of the text inside it.
    let metrics = layout.metrics();
    let (width, height) = layout.max_size();
    ctx.draw_rect(&Rect::from_xywh(MARGIN, MARGIN, width, height), &frame, 1.0);
    ctx.draw_rect(&metrics.bounds().offset(MARGIN, MARGIN), &bounds, 1.5);

    // Lines stack from the top of the text, each with its baseline below its top.
    let mut top = metrics.top;

    for line in layout.line_metrics() {
        let y = MARGIN + top + line.baseline;
        ctx.draw_line(
            Vector2::new(MARGIN, y),
            Vector2::new(MARGIN + width, y),
            &baseline,
            1.0,
        );
        top += line.height;
    }

    let clusters = layout.cluster_metrics();

    for metrics in &clusters {
        for rect in layout.range_bounds(metrics.text_range.clone()) {
            ctx.draw_rect(&rect.offset(MARGIN, MARGIN), &cluster, 1.0);
        }
    }

    ctx.draw_text_layout(origin, &layout, &white);

    let summary = format!(
        "Lines: {}, clusters: {}, text: {:.1} x {:.1}",
        metrics.line_count,
        clusters.len(),
        metrics.width,
        metrics.height
    );
    let format = TextFormat::new("Consolas", 15.0)?;
    let label = TextLayout::new(&summary, &format, width, MARGIN)?;
    ctx.draw_text_layout(Vector2::new(MARGIN, ctx.height - MARGIN), &label, &bounds);
    Ok(())
}

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Text metrics");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        Border::new().content(canvas(draw)).into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

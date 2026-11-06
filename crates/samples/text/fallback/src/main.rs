#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{FontCollection, FontFallback, TextFormat, TextLayout};

const TEXT: &str = "Latin, \u{395}\u{3bb}\u{3bb}\u{3b7}\u{3bd}\u{3b9}\u{3ba}\u{3ac}, \
\u{5e2}\u{5d1}\u{5e8}\u{5d9}\u{5ea}, \u{627}\u{644}\u{639}\u{631}\u{628}\u{64a}\u{629}, \
\u{4e2d}\u{6587}, \u{65e5}\u{672c}\u{8a9e}, \u{d55c}\u{ad6d}\u{c5b4}, \u{2605} \u{2713} \u{266b}, \
\u{1f600}";

const MARGIN: f32 = 24.0;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let dim = ctx.create_solid_brush(ColorF::from_rgb8(140, 150, 170))?;
    let accent = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    let label_format = TextFormat::new("Segoe UI", 14.0)?;
    let width = (ctx.width - 2.0 * MARGIN).max(0.0);

    // Consolas covers only a few scripts. The system fallback picks fonts for the rest, and a
    // custom fallback tries its own families first, here a monochrome emoji and a different
    // Chinese font.
    let system = FontCollection::system();
    let custom = FontFallback::new(&system, &["Segoe UI Symbol", "SimSun"]);
    let rows = [
        ("System fallback", TextFormat::new("Consolas", 26.0)?),
        (
            "Segoe UI Symbol and SimSun first",
            TextFormat::new("Consolas", 26.0)?.with_fallback(&custom),
        ),
    ];

    let mut y = MARGIN;

    for (label, format) in &rows {
        let layout = TextLayout::new(label, &label_format, width, 24.0)?;
        ctx.draw_text_layout(Vector2::new(MARGIN, y), &layout, &dim);
        y += 24.0;

        let layout = TextLayout::new(TEXT, format, width, ctx.height)?;
        ctx.draw_text_layout(Vector2::new(MARGIN, y), &layout, &white);
        y += layout.metrics().height + 8.0;

        // The family of each glyph run shows which font drew which text.
        let mut fonts: Vec<(String, Vec<&str>)> = Vec::new();

        for run in layout.glyph_runs() {
            let family = run.font_face.family_name();
            let text = TEXT[run.text_range].trim_matches([' ', ',']);

            if text.is_empty() {
                continue;
            }

            match fonts.iter_mut().find(|(name, _)| *name == family) {
                Some((_, texts)) => texts.push(text),
                None => fonts.push((family, vec![text])),
            }
        }

        let summary = fonts
            .iter()
            .map(|(family, texts)| format!("{family}: {}", texts.join(" ")))
            .collect::<Vec<_>>()
            .join("\n");
        let layout = TextLayout::new(&summary, &label_format, width, ctx.height)?;
        ctx.draw_text_layout(Vector2::new(MARGIN, y), &layout, &accent);
        y += layout.metrics().height + 32.0;
    }

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
        context.window_title("Font fallback");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        Border::new().content(canvas(draw)).into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

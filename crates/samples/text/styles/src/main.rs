#![windows_subsystem = "windows"]

use std::ops::Range;
use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{FontFeature, FontStyle, FontWeight, TextFormat, TextLayout};

const MARGIN: f32 = 24.0;

// Text with the range of each styled part.
#[derive(Default)]
struct Builder {
    text: String,
}

impl Builder {
    fn add(&mut self, text: &str) -> Range<usize> {
        let start = self.text.len();
        self.text.push_str(text);
        start..self.text.len()
    }
}

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;

    let mut text = Builder::default();
    let bold = text.add("Bold");
    text.add(", ");
    let italic = text.add("italic");
    text.add(", ");
    let larger = text.add("larger");
    text.add(", ");
    let underline = text.add("underlined");
    text.add(", ");
    let strikethrough = text.add("struck out");
    text.add(", and ");
    let family = text.add("Consolas");
    text.add(" in one layout.\n\nLigatures: office affinity, without: ");
    let ligatures = text.add("office affinity");
    text.add("\nSmall caps: ");
    let small_caps = text.add("Small Caps");
    text.add("\nOldstyle figures: ");
    let oldstyle = text.add("0123456789");
    text.add(", lining: 0123456789\nFractions: ");
    let fractions = text.add("1/2 3/4");
    text.add(", without: 1/2 3/4");

    // Each setter styles a UTF-8 byte range of the text.
    let format = TextFormat::new("Calibri", 26.0);
    let mut layout = TextLayout::new(
        &text.text,
        &format,
        (ctx.width - 2.0 * MARGIN).max(0.0),
        (ctx.height - 2.0 * MARGIN).max(0.0),
    );
    layout.set_font_weight(bold, FontWeight::BOLD);
    layout.set_font_style(italic, FontStyle::Italic);
    layout.set_font_size(larger, 38.0);
    layout.set_underline(underline, true);
    layout.set_strikethrough(strikethrough, true);
    layout.set_font_family(family, "Consolas");
    layout.set_font_features(ligatures, &[FontFeature::new(b"liga", 0)]);
    layout.set_font_features(small_caps, &[FontFeature::new(b"smcp", 1)]);
    layout.set_font_features(oldstyle, &[FontFeature::new(b"onum", 1)]);
    layout.set_font_features(fractions, &[FontFeature::new(b"frac", 1)]);

    ctx.draw_text_layout(Vector2::new(MARGIN, MARGIN), &layout, &white);
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
        context.window_title("Text styles");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        Border::new().content(canvas(draw)).into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

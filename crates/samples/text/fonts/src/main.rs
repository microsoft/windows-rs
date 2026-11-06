#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{FontCollection, FontStyle, FontWeight, TextFormat, TextLayout, WordWrapping};

const MARGIN: f32 = 24.0;
const COLUMN: f32 = 260.0;
const ROW: f32 = 30.0;
const HEADER: f32 = 120.0;

struct Sample {
    loaded: FontCollection,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        // Any font data works, such as a font embedded with `include_bytes!`.
        let path = std::env::var("WINDIR").unwrap() + r"\Fonts\Gabriola.ttf";
        let data = std::fs::read(path).unwrap();

        Self {
            loaded: FontCollection::from_data([data.as_slice()]).unwrap(),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Fonts");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let loaded = self.loaded.clone();
        Border::new()
            .content(canvas(move |ctx| draw(ctx, &loaded)))
            .into()
    }
}

fn draw(ctx: &DrawContext, loaded: &FontCollection) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let accent = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
    let system = FontCollection::system();
    let families = system.family_names();

    // A format draws with the loaded collection instead of the system's fonts.
    let format = TextFormat::new("Gabriola", 44.0).with_collection(loaded);
    let layout = TextLayout::new("Gabriola, loaded from data", &format, ctx.width, HEADER);
    ctx.draw_text_layout(Vector2::new(MARGIN, MARGIN - 12.0), &layout, &accent);

    let face = loaded
        .find("Gabriola", FontWeight::NORMAL, FontStyle::Normal)
        .unwrap();
    let metrics = face.metrics();
    let summary = format!(
        "Ascent {}, descent {}, {} units per em. {} families installed:",
        metrics.ascent,
        metrics.descent,
        metrics.design_units_per_em,
        families.len()
    );
    let format = TextFormat::new("Segoe UI", 15.0);
    let layout = TextLayout::new(&summary, &format, ctx.width, ROW);
    ctx.draw_text_layout(Vector2::new(MARGIN, MARGIN + 64.0), &layout, &white);

    // Each installed family, drawn in its own font, in columns that fill the window.
    let mut origin = Vector2::new(MARGIN, HEADER);

    for family in &families {
        if origin.y + ROW > ctx.height - MARGIN {
            origin = Vector2::new(origin.x + COLUMN, HEADER);
        }

        if origin.x + COLUMN > ctx.width {
            break;
        }

        let format = TextFormat::new(family, 18.0).with_word_wrapping(WordWrapping::NoWrap);
        let layout = TextLayout::new(family, &format, COLUMN, ROW);
        ctx.draw_text_layout(origin, &layout, &white);
        origin.y += ROW;
    }

    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

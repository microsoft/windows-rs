#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{GlyphImage, TextFormat, TextLayout};

const TEXT: &str = "Glyph runs: caf\u{e9}, \u{5e9}\u{5dc}\u{5d5}\u{5dd}, and \u{1f600}, \
with an underline.";

const MARGIN: f32 = 24.0;
const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

// Premultiplied BGRA pixels that glyph coverage is blended into.
struct Target {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Target {
    // Blends `color` wherever `image` has coverage, with the glyph's pen at `x`, `y`.
    fn draw(&mut self, image: &GlyphImage, x: i32, y: i32, color: [f32; 4]) {
        for row in 0..image.height as i32 {
            for column in 0..image.width as i32 {
                let coverage = image.data[(row * image.width as i32 + column) as usize];
                self.blend(
                    x + image.left + column,
                    y + image.top + row,
                    color,
                    coverage,
                );
            }
        }
    }

    fn fill(&mut self, bounds: Rect, color: [f32; 4]) {
        let top = bounds.top.round() as i32;
        let bottom = (bounds.bottom.round() as i32).max(top + 1);

        for y in top..bottom {
            for x in bounds.left.round() as i32..bounds.right.round() as i32 {
                self.blend(x, y, color, 255);
            }
        }
    }

    fn blend(&mut self, x: i32, y: i32, color: [f32; 4], coverage: u8) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }

        let alpha = color[3] * f32::from(coverage) / 255.0;
        let index = (y as usize * self.width + x as usize) * 4;
        let pixel = &mut self.pixels[index..index + 4];

        for (channel, source) in [color[2], color[1], color[0], 1.0].into_iter().enumerate() {
            let target = f32::from(pixel[channel]) / 255.0;
            pixel[channel] = ((source * alpha + target * (1.0 - alpha)) * 255.0).round() as u8;
        }
    }
}

// Draws the layout by hand from its glyph runs, at `scale` physical pixels per DIP. A real
// renderer caches the images by face, glyph, size, and subpixel offset, usually in an atlas.
fn rasterize(layout: &TextLayout, scale: f32) -> Target {
    let metrics = layout.metrics();
    let mut target = Target {
        width: (metrics.layout_width * scale).ceil() as usize,
        height: (metrics.bounds().bottom * scale).ceil() as usize,
        pixels: Vec::new(),
    };
    target.pixels = vec![0; target.width * target.height * 4];

    for run in layout.glyph_runs() {
        let face = &run.font_face;
        let size = run.font_size * scale;

        for glyph in &run.glyphs {
            // Keep the fraction of the pen's horizontal position, and snap the baseline to whole
            // pixels.
            let x = glyph.position.x * scale;
            let offset = Vector2::new(x - x.floor(), 0.0);
            let (x, y) = (x.floor() as i32, (glyph.position.y * scale).round() as i32);

            // A color glyph, such as an emoji, draws as layers of glyphs in its own colors.
            let layers = match face.color_layers(glyph.id) {
                Some(layers) => layers
                    .iter()
                    .map(|layer| (layer.glyph, layer.color))
                    .collect(),
                None => vec![(glyph.id, None)],
            };

            for (id, color) in layers {
                if let Some(image) = face.rasterize(id, size, offset) {
                    target.draw(&image, x, y, color.unwrap_or(FOREGROUND));
                }
            }
        }
    }

    for decoration in layout.decorations() {
        let bounds = decoration.bounds;
        let bounds = Rect::new(
            bounds.left * scale,
            bounds.top * scale,
            bounds.right * scale,
            bounds.bottom * scale,
        );
        target.fill(bounds, FOREGROUND);
    }

    target
}

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::from_rgb8(16, 20, 28));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let dim = ctx.create_solid_brush(ColorF::from_rgb8(140, 150, 170))?;
    let label_format = TextFormat::new("Segoe UI", 14.0)?;

    let format = TextFormat::new("Segoe UI", 32.0)?;
    let width = (ctx.width - 2.0 * MARGIN).max(0.0);
    let mut layout = TextLayout::new(TEXT, &format, width, ctx.height)?;
    let underline = TEXT.find("underline").unwrap();
    layout.set_underline(underline..underline + "underline".len(), true);
    let bottom = layout.metrics().bounds().bottom;

    let label = TextLayout::new("Direct2D", &label_format, ctx.width, MARGIN)?;
    ctx.draw_text_layout(Vector2::new(MARGIN, MARGIN), &label, &dim);
    ctx.draw_text_layout(Vector2::new(MARGIN, 2.0 * MARGIN), &layout, &white);

    let y = 3.0 * MARGIN + bottom;
    let label = TextLayout::new("Glyph runs, by hand", &label_format, ctx.width, MARGIN)?;
    ctx.draw_text_layout(Vector2::new(MARGIN, y), &label, &dim);

    // Align the bitmap to whole physical pixels so it maps one to one.
    let scale = ctx.scale();
    let target = rasterize(&layout, scale);
    let x = (MARGIN * scale).round() / scale;
    let y = ((y + MARGIN) * scale).round() / scale;
    let bitmap = ctx.create_bitmap(&target.pixels, target.width as u32, target.height as u32)?;
    let dest = Rect::from_xywh(
        x,
        y,
        target.width as f32 / scale,
        target.height as f32 / scale,
    );
    ctx.draw_bitmap(&bitmap, &dest, 1.0);
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
        context.window_title("Glyph runs");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        Border::new().content(canvas(draw)).into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

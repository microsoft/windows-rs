#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    App, Border, Component, ComponentContext, View, ViewContext, WindowBackdrop, WindowVisuals,
};
use windows_text::{GlyphImage, PixelGeometry, RenderingParams, TextFormat, TextLayout};

const TEXT: &str = "Glyph runs: caf\u{e9}, \u{5e9}\u{5dc}\u{5d5}\u{5dd}, and \u{1f600}, \
with an underline.";

const MARGIN: f32 = 24.0;
const FOREGROUND: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BACKGROUND: [u8; 3] = [16, 20, 28];

// Opaque BGRA pixels that glyph coverage is blended into. ClearType coverage can only be blended
// with an opaque background, so the target starts filled with the window's background color.
struct Target {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
    params: RenderingParams,
    gamma: [f32; 4],
}

impl Target {
    fn new(width: usize, height: usize) -> Self {
        let [red, green, blue] = BACKGROUND;
        let pixels = [blue, green, red, 255].repeat(width * height);
        let params = RenderingParams::system();

        Self {
            width,
            height,
            pixels,
            params,
            gamma: params.gamma_ratios(),
        }
    }

    // Blends `color` wherever `image` has coverage, with the glyph's pen at `x`, `y`. A ClearType
    // image has red, green, and blue coverage for each pixel; `bgr` swaps them for a monitor
    // whose stripes are in blue, green, red order.
    fn draw(&mut self, image: &GlyphImage, x: i32, y: i32, color: [f32; 4], bgr: bool) {
        let channels = image.data.len() / (image.width * image.height) as usize;

        for row in 0..image.height as i32 {
            for column in 0..image.width as i32 {
                let index = (row * image.width as i32 + column) as usize * channels;
                let mut coverage = [image.data[index]; 3];

                if channels == 3 {
                    coverage.copy_from_slice(&image.data[index..index + 3]);

                    if bgr {
                        coverage.reverse();
                    }
                }

                let coverage = self.correct(coverage, color, channels == 3);

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
                self.blend(x, y, color, [1.0; 3]);
            }
        }
    }

    // Adjusts coverage for `color` as DirectWrite does, as described for
    // `RenderingParams::gamma_ratios`, and returns it from 0.0 to 1.0.
    fn correct(&self, coverage: [u8; 3], color: [f32; 4], cleartype: bool) -> [f32; 3] {
        let coverage = coverage.map(|value| f32::from(value) / 255.0);
        let [red, green, blue, _] = color;

        let (contrast, level) = if cleartype {
            (self.params.enhanced_contrast, self.params.cleartype_level)
        } else {
            (self.params.grayscale_enhanced_contrast, 1.0)
        };

        let lightness = 0.30 * red + 0.59 * green + 0.11 * blue;
        let contrast = contrast * (4.0 * (0.75 - lightness)).clamp(0.0, 1.0);
        let average = coverage.iter().sum::<f32>() / 3.0;
        let gray = 0.25 * red + 0.5 * green + 0.25 * blue;
        let [g0, g1, g2, g3] = self.gamma;

        std::array::from_fn(|channel| {
            let mut alpha = level * coverage[channel] + (1.0 - level) * average;
            alpha = alpha * (contrast + 1.0) / (alpha * contrast + 1.0);
            let intensity = if cleartype { color[channel] } else { gray };
            alpha + alpha * (1.0 - alpha) * ((g0 * intensity + g1) * alpha + g2 * intensity + g3)
        })
    }

    // Blends each color channel with its own coverage from 0.0 to 1.0, given in red, green, blue
    // order.
    fn blend(&mut self, x: i32, y: i32, color: [f32; 4], coverage: [f32; 3]) {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }

        let index = (y as usize * self.width + x as usize) * 4;
        let pixel = &mut self.pixels[index..index + 3];

        for (channel, (source, coverage)) in [
            (color[2], coverage[2]),
            (color[1], coverage[1]),
            (color[0], coverage[0]),
        ]
        .into_iter()
        .enumerate()
        {
            let alpha = color[3] * coverage;
            let target = f32::from(pixel[channel]) / 255.0;
            pixel[channel] = ((source * alpha + target * (1.0 - alpha)) * 255.0).round() as u8;
        }
    }
}

// Draws the layout by hand from its glyph runs, at `scale` physical pixels per DIP, with
// ClearType for the given pixel geometry or in grayscale for `None`. A real renderer caches the
// images by face, glyph, size, and subpixel offset, usually in an atlas.
fn rasterize(layout: &TextLayout, scale: f32, cleartype: Option<PixelGeometry>) -> Target {
    let metrics = layout.metrics();
    let mut target = Target::new(
        (metrics.layout_width * scale).ceil() as usize,
        (metrics.bounds().bottom * scale).ceil() as usize,
    );
    let bgr = cleartype == Some(PixelGeometry::Bgr);

    for run in layout.glyph_runs() {
        let face = &run.font_face;
        let size = run.font_size * scale;

        for glyph in &run.glyphs {
            // Keep the fraction of the pen's horizontal position, and snap the baseline to whole
            // pixels.
            let x = glyph.position.x * scale;
            let offset = Vector2::new(x - x.floor(), 0.0);
            let (x, y) = (x.floor() as i32, (glyph.position.y * scale).round() as i32);

            // A color glyph, such as an emoji, draws as grayscale layers of glyphs in its own
            // colors.
            if let Some(layers) = face.color_layers(glyph.id) {
                for layer in layers {
                    if let Some(image) = face.rasterize(layer.glyph, size, offset) {
                        let color = layer.color.unwrap_or(FOREGROUND);
                        target.draw(&image, x, y, color, false);
                    }
                }
            } else {
                let image = if cleartype.is_some() {
                    face.rasterize_cleartype(glyph.id, size, offset)
                } else {
                    face.rasterize(glyph.id, size, offset)
                };

                if let Some(image) = image {
                    target.draw(&image, x, y, FOREGROUND, bgr);
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
    let [red, green, blue] = BACKGROUND;
    ctx.clear(ColorF::from_rgb8(red, green, blue));
    let white = ctx.create_solid_brush(ColorF::WHITE)?;
    let dim = ctx.create_solid_brush(ColorF::from_rgb8(140, 150, 170))?;
    let label_format = TextFormat::new("Segoe UI", 14.0);

    let format = TextFormat::new("Segoe UI", 32.0);
    let width = (ctx.width - 2.0 * MARGIN).max(0.0);
    let mut layout = TextLayout::new(TEXT, &format, width, ctx.height);
    let underline = TEXT.find("underline").unwrap();
    layout.set_underline(underline..underline + "underline".len(), true);
    let bottom = layout.metrics().bounds().bottom;

    let label = TextLayout::new("Direct2D", &label_format, ctx.width, MARGIN);
    ctx.draw_text_layout(Vector2::new(MARGIN, MARGIN), &label, &dim);
    ctx.draw_text_layout(Vector2::new(MARGIN, 2.0 * MARGIN), &layout, &white);

    // ClearType needs a monitor with known stripes, so it is shown only for one.
    let cleartype = match RenderingParams::system().pixel_geometry {
        PixelGeometry::Flat => None,
        geometry => Some(("Glyph runs by hand, ClearType", Some(geometry))),
    };
    let sections = [Some(("Glyph runs by hand, grayscale", None)), cleartype];
    let mut y = 3.0 * MARGIN + bottom;

    for (text, cleartype) in sections.into_iter().flatten() {
        let label = TextLayout::new(text, &label_format, ctx.width, MARGIN);
        ctx.draw_text_layout(Vector2::new(MARGIN, y), &label, &dim);

        // Align the bitmap to whole physical pixels so it maps one to one.
        let scale = ctx.scale();
        let target = rasterize(&layout, scale, cleartype);
        let x = (MARGIN * scale).round() / scale;
        let top = ((y + MARGIN) * scale).round() / scale;
        let bitmap =
            ctx.create_bitmap(&target.pixels, target.width as u32, target.height as u32)?;
        let dest = Rect::from_xywh(
            x,
            top,
            target.width as f32 / scale,
            target.height as f32 / scale,
        );
        ctx.draw_bitmap(&bitmap, &dest, 1.0);
        y += 2.0 * MARGIN + bottom;
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
        context.window_title("Glyph runs");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        Border::new().content(canvas(draw)).into()
    }
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

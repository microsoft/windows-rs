#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use windows_canvas::Rect as CanvasRect;
use windows_canvas::*;
use windows_reactor::*;

const BARS: usize = 12;
const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 320.0;

struct Sample {
    seed: u32,
    invalidator: Invalidator,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            seed: 1,
            invalidator: Invalidator::new(),
        }
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.seed = self.seed.wrapping_add(1);
        self.invalidator.invalidate();
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Canvas Chart");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let seed = self.seed;
        StackPanel::new()
            .spacing(12.0)
            .margin(Thickness::uniform(16.0))
            .children((
                TextBlock::new().text("On-demand canvas redraws only when the data changes:"),
                Border::new()
                    .width(WIDTH)
                    .height(HEIGHT)
                    .content(canvas_invalidated(&self.invalidator, move |context| {
                        draw_chart(context, seed)
                    })),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .on_click(context.message(()))
                            .content(TextBlock::new().text("New data")),
                        TextBlock::new().text(format!("revision {}", self.seed)),
                    )),
            ))
    }
}

fn draw_chart(context: &DrawContext, seed: u32) -> Result<()> {
    context.clear(ColorF::new(0.10, 0.12, 0.16, 1.0));
    let padding = 24.0;
    let inner_width = (context.width - padding * 2.0).max(1.0);
    let inner_height = (context.height - padding * 2.0).max(1.0);
    let gap = 8.0;
    let bar_width = ((inner_width - gap * (BARS as f32 - 1.0)) / BARS as f32).max(1.0);
    let baseline = padding + inner_height;

    for index in 0..BARS {
        let value = bar_value(seed, index);
        let bar_height = inner_height * value;
        let left = padding + index as f32 * (bar_width + gap);
        let rect = CanvasRect::new(left, baseline - bar_height, left + bar_width, baseline);
        let hue = index as f32 / BARS as f32;
        let brush = context.create_solid_brush(ColorF::new(
            0.30 + 0.60 * (hue * TAU).cos().abs(),
            0.35 + 0.55 * value,
            0.75,
            1.0,
        ))?;
        context.fill_rect(&rect, &brush);
    }
    Ok(())
}

fn bar_value(seed: u32, index: usize) -> f32 {
    let mut value = seed
        .wrapping_mul(2_654_435_761)
        .wrapping_add((index as u32).wrapping_mul(40_503));
    value ^= value >> 13;
    value = value.wrapping_mul(1_274_126_177);
    value ^= value >> 16;
    0.15 + 0.85 * (value % 1000) as f32 / 1000.0
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

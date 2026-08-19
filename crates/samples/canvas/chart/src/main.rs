#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use windows_canvas::*;
use windows_reactor::{
    Application, CanvasDrawContext, Element, RenderCx, StackPanel, Thickness, Window,
    WindowBackdrop, button, component, hstack, run_reactor_winui_app,
    swap_chain_canvas_invalidated, text_block,
};

const BARS: usize = 12;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 320.0;

fn app(cx: &mut RenderCx<'_>) -> Element {
    let seed = cx.use_state(|| 1_u32);
    let current_seed = seed.value();
    let invalidator = cx.use_canvas_invalidator();
    let invalidate = invalidator.clone();
    cx.use_effect(current_seed, move || {
        invalidate.invalidate();
    });

    StackPanel::new([
        text_block("On-demand swap chain - redraws only when the data changes:"),
        swap_chain_canvas_invalidated(&invalidator, move |ctx| draw_chart(ctx, current_seed))
            .width(WIDTH as f64)
            .height(HEIGHT as f64)
            .build(),
        hstack(
            8.0,
            [
                button("New data", move || {
                    seed.update(|value| *value = value.wrapping_add(1));
                }),
                text_block(format!("revision {current_seed}")),
            ],
        ),
    ])
    .spacing(12.0)
    .margin(Thickness::uniform(16.0))
    .build()
}

fn draw_chart(ctx: &CanvasDrawContext<'_>, seed: u32) -> Result<()> {
    ctx.clear(ColorF::new(0.10, 0.12, 0.16, 1.0));

    let pad = 24.0;
    let inner_w = (ctx.width - pad * 2.0).max(1.0);
    let inner_h = (ctx.height - pad * 2.0).max(1.0);
    let gap = 8.0;
    let bar_w = ((inner_w - gap * (BARS as f32 - 1.0)) / BARS as f32).max(1.0);
    let baseline = pad + inner_h;

    for i in 0..BARS {
        let value = bar_value(seed, i);
        let bar_h = inner_h * value;
        let left = pad + i as f32 * (bar_w + gap);
        let rect = Rect::new(left, baseline - bar_h, left + bar_w, baseline);
        let hue = i as f32 / BARS as f32;
        let brush = ctx.create_solid_brush(ColorF::new(
            0.30 + 0.60 * (hue * TAU).cos().abs(),
            0.35 + 0.55 * value,
            0.75,
            1.0,
        ))?;
        ctx.fill_rect(&rect, &brush);
    }
    Ok(())
}

fn bar_value(seed: u32, index: usize) -> f32 {
    let mut x = seed
        .wrapping_mul(2_654_435_761)
        .wrapping_add((index as u32).wrapping_mul(40_503));
    x ^= x >> 13;
    x = x.wrapping_mul(1_274_126_177);
    x ^= x >> 16;
    0.15 + 0.85 * (x % 1000) as f32 / 1000.0
}

fn main() -> Result<()> {
    let root = component(|cx| {
        let open = cx.use_state(|| true);
        let windows = if open.value() {
            vec![
                Window::new("Canvas Chart", component(app), move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .client_size(704.0, 456.0)
                .build()
                .key(0),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}

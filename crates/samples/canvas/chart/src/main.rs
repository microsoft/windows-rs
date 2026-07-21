//! On-demand swap-chain drawing with `windows-canvas` hosted in a reactor UI.
//!
//! A [`CanvasSwapChain`] presents through a composition swap chain — low-latency,
//! like [`animated_canvas`] — but only when its data changes, like
//! [`CanvasImageSource`]. Here the "New data" button changes the bar chart; each
//! change redraws exactly one frame. While the data is idle nothing is drawn and
//! no render loop runs, so an idle chart costs no GPU work.
//!
//! Compare with the continuous per-vsync loop in `crates/samples/canvas/clock`
//! and the `SurfaceImageSource` on-demand path in
//! `crates/samples/canvas/image_source`.

#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use windows_canvas::*;
use windows_reactor::*;

/// Number of bars in the chart.
const BARS: usize = 12;

/// Displayed surface size, in device-independent pixels.
const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 320.0;

fn app(cx: &mut RenderCx) -> Element {
    let (seed, set_seed) = cx.use_state(1_u32);
    let (size, set_size) = cx.use_state((WIDTH, HEIGHT));
    let (scale, set_scale) = cx.use_state(1.0_f32);
    let device = cx.use_ref::<Option<GpuDevice>>(None);
    let host = cx.use_ref::<Option<CanvasSwapChain>>(None);
    let scale_sub = cx.use_ref::<Option<windows_core::EventRevoker>>(None);

    // One process-wide device, shared by every surface (here, just this chart —
    // but the same device could back a wall of charts). Created lazily so the
    // whole tree can render before any GPU work happens.
    if device.borrow().is_none() {
        match GpuDevice::new_or_warp() {
            Ok(d) => device.set(Some(d)),
            Err(e) => eprintln!("failed to create device: {e}"),
        }
    }

    // Redraw only when the data (`seed`), the panel size, or the DPI scale
    // changes. On an idle frame this effect does not run, so the swap chain
    // presents nothing.
    {
        let host = host.clone();
        cx.use_effect((seed, size, scale), move || {
            if let Some(chain) = host.borrow().as_ref() {
                chain.set_scale(scale);
                chain.resize(size.0, size.1);
                let _ = chain.draw(|ctx| draw_chart(ctx, seed));
            }
        });
    }

    let panel = swap_chain_panel()
        .on_mounted(move |panel| {
            let s = panel.composition_scale().map_or(1.0, |(x, _)| x);
            set_scale.call(s);

            let device = device.borrow();
            let Some(device) = device.as_ref() else {
                return;
            };
            // The panel's native control now exists, so a swap chain can be
            // attached. Draw the first frame immediately.
            match CanvasSwapChain::with_device(&panel, device, WIDTH, HEIGHT, s) {
                Ok(chain) => {
                    let _ = chain.draw(|ctx| draw_chart(ctx, seed));
                    host.set(Some(chain));
                }
                Err(e) => return eprintln!("failed to create swap chain: {e}"),
            }

            // Keep the chart crisp when the window moves to a monitor with a
            // different scale factor.
            let set_scale = set_scale.clone();
            if let Ok(revoker) = panel.on_composition_scale_changed(move |x, _| set_scale.call(x)) {
                scale_sub.set(Some(revoker));
            }
        })
        .on_resize(move |w, h| set_size.call((w as f32, h as f32)))
        .width(WIDTH as f64)
        .height(HEIGHT as f64);

    let controls = hstack((
        button("New data").on_click(move || set_seed.call(seed.wrapping_add(1))),
        text_block(format!("revision {seed}")),
    ))
    .spacing(8.0);

    vstack((
        text_block("On-demand swap chain \u{2014} redraws only when the data changes:"),
        panel,
        controls,
    ))
    .spacing(12.0)
    .margin(Thickness::uniform(16.0))
    .into()
}

/// Draws a bar chart in surface-local device-independent pixels.
fn draw_chart(ctx: &DrawContext, seed: u32) {
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
        let brush = ctx
            .create_solid_brush(ColorF::new(
                0.30 + 0.60 * (hue * TAU).cos().abs(),
                0.35 + 0.55 * value,
                0.75,
                1.0,
            ))
            .unwrap();
        ctx.fill_rect(&rect, &brush);
    }
}

/// Cheap deterministic hash mapping `(seed, index)` to a bar height in `0.15..1.0`.
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
    App::new()
        .title("Canvas Chart")
        .backdrop(Backdrop::Mica)
        .render(app)
}

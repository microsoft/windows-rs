#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use windows_canvas::{ColorF, Ellipse, Vector2};
use windows_reactor::{
    Application, TextBlock, Window, button, component, run_reactor_winui_app, stack_panel,
    swap_chain_canvas_invalidated, text_block,
};

fn main() -> windows_core::Result<()> {
    let frames = Arc::new(AtomicU32::new(0));
    let root = component(move |cx| {
        let count = cx.use_state(|| 5u32);
        let current_count = count.value();
        let refresh = cx.use_state(|| false);
        let current_refresh = refresh.value();
        let invalidator = cx.use_canvas_invalidator();
        let draw_frames = Arc::clone(&frames);
        let invalidate = invalidator.clone();

        let content = stack_panel([
            swap_chain_canvas_invalidated(&invalidator, move |ctx| {
                draw_frames.fetch_add(1, Ordering::Relaxed);
                ctx.clear(ColorF::new(0.08, 0.12, 0.2, 1.0));
                let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
                for index in 0..current_count {
                    ctx.fill_ellipse(
                        &Ellipse::circle(Vector2::new(30.0 + index as f32 * 35.0, 80.0), 12.0),
                        &brush,
                    );
                }
                Ok(())
            })
            .width(320.0)
            .height(180.0)
            .automation_name("Matched drawing surface")
            .build(),
            text_block(format!("Circle count: {current_count}")),
            TextBlock::new(format!("Canvas frames: {}", frames.load(Ordering::Relaxed)))
                .automation_id("canvas-frame-status")
                .build(),
            button("Invalidate canvas", move || {
                count.update(|value| *value += 1);
                invalidate.invalidate();
            }),
            button("Refresh canvas status", move || {
                refresh.set(!current_refresh);
            }),
        ]);
        Application::new([
            Window::new("windows-reactor matched canvas", content, || {})
                .client_size(420.0, 360.0)
                .build()
                .key(0),
        ])
        .build()
    });
    run_reactor_winui_app(root)
}

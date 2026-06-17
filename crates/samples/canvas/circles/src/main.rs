//! Animated circles rendered with `windows-canvas` inside a reactor UI.
//!
//! Compare with `crates/samples/reactor/minimal/examples/direct2d.rs` —
//! that sample requires ~250 lines of raw D2D/DXGI setup. This one
//! uses `animated_canvas()` which handles everything automatically.
//!
//! State is managed entirely through reactor hooks (`use_state` and
//! `use_ref`) — no `thread_local!` needed. See `docs/reactor-state.md`.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(5_u32);
    let frame = cx.use_ref(0_u64);
    let circle_count = cx.use_ref(5_u32);
    circle_count.set(count);

    let add = {
        let s = set_count.clone();
        move || s.call(count + 1)
    };

    let remove = {
        let s = set_count;
        move || {
            if count > 0 {
                s.call(count - 1);
            }
        }
    };

    let margin = 16.0;

    grid((
        Element::from(
            animated_canvas({
                move |ctx| {
                    *frame.borrow_mut() += 1;
                    let t = *frame.borrow() as f32 * 0.02;
                    let count = *circle_count.borrow();
                    let cx = ctx.width / 2.0;
                    let cy = ctx.height / 2.0;
                    let orbit = cx.min(cy) * 0.5;

                    ctx.clear(ColorF::TRANSPARENT);

                    // Create a brush once per frame (cheap — reuse via set_color).
                    let Ok(brush) = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE) else {
                        return;
                    };

                    for i in 0..count {
                        let phase = t + i as f32 * 1.2;
                        let x = cx + phase.cos() * orbit;
                        let y = cy + phase.sin() * (orbit * 0.7);
                        let radius = 20.0 + (phase * 0.7).sin().abs() * 30.0;

                        brush.set_color(ColorF::new(
                            0.3 + (phase * 0.3).sin().abs() * 0.7,
                            0.4 + (phase * 0.5).cos().abs() * 0.5,
                            0.8,
                            0.85,
                        ));

                        ctx.fill_ellipse(&Ellipse::circle(Vector2::new(x, y), radius), &brush);
                    }

                    // Draw a label showing the count.
                    let Ok(format) = TextFormat::with_weight("Segoe UI", 20.0, FontWeight::BOLD)
                        .map(|f| f.with_alignment(TextAlignment::Center))
                    else {
                        return;
                    };

                    let label = format!("{count} circles");
                    brush.set_color(ColorF::WHITE);
                    let rect = Rect::new(0.0, ctx.height - 40.0, ctx.width, ctx.height);
                    ctx.draw_text(&label, &format, &rect, &brush);
                }
            })
            .margin(Thickness {
                left: margin,
                top: margin,
                right: margin,
                bottom: 0.0,
            }),
        )
        .grid_row(0),
        Element::from(
            hstack((
                button("Add circle").on_click(add),
                button("Remove circle").on_click(remove),
            ))
            .spacing(8.0)
            .margin(Thickness::uniform(margin)),
        )
        .grid_row(1),
    ))
    .rows([GridLength::STAR, GridLength::Auto])
    .into()
}

fn main() -> Result<()> {
    App::new()
        .title("Canvas + Reactor")
        .backdrop(Backdrop::Mica)
        .render(app)
}

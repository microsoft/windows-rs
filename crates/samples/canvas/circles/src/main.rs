#![windows_subsystem = "windows"]

use std::cell::Cell;
use std::rc::Rc;

use windows_canvas::*;
use windows_reactor::{
    Application, Button, CanvasDrawContext, Grid, GridChild, GridLength, Orientation, StackPanel,
    Thickness, Window, WindowBackdrop, animated_canvas, component, run_reactor_winui_app,
};

fn main() -> Result<()> {
    let frame = Rc::new(Cell::new(0u64));
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let count = cx.use_state(|| 5_u32);
        let current_count = count.value();
        let draw_frame = Rc::clone(&frame);
        let add_count = count.clone();
        let margin = 16.0;

        let content = Grid::new([
            GridChild::new(
                animated_canvas(move |ctx: &CanvasDrawContext<'_>| {
                    let frame = draw_frame.get().wrapping_add(1);
                    draw_frame.set(frame);
                    let t = frame as f32 * 0.02;
                    let cx = ctx.width / 2.0;
                    let cy = ctx.height / 2.0;
                    let orbit = cx.min(cy) * 0.5;

                    ctx.clear(ColorF::TRANSPARENT);
                    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

                    for i in 0..current_count {
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

                    let format = TextFormat::with_weight("Segoe UI", 20.0, FontWeight::BOLD)?
                        .with_alignment(TextAlignment::Center);

                    let label = format!("{current_count} circles");
                    brush.set_color(ColorF::WHITE);
                    let rect = Rect::new(0.0, ctx.height - 40.0, ctx.width, ctx.height);
                    ctx.draw_text(&label, &format, &rect, &brush);

                    Ok(())
                })
                .margin(Thickness {
                    left: margin,
                    top: margin,
                    right: margin,
                    bottom: 0.0,
                })
                .build(),
            )
            .row(0),
            GridChild::new(
                StackPanel::new([
                    Button::new("Add circle")
                        .on_click(move || {
                            add_count.set(current_count + 1);
                        })
                        .build(),
                    Button::new("Remove circle")
                        .on_click(move || {
                            if current_count > 0 {
                                count.set(current_count - 1);
                            }
                        })
                        .build(),
                ])
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .margin(Thickness::uniform(margin))
                .build(),
            )
            .row(1),
        ])
        .rows([GridLength::STAR, GridLength::Auto])
        .build();

        let windows = if open.value() {
            vec![
                Window::new("Canvas + Reactor", content, move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .client_size(800.0, 600.0)
                .build(),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}

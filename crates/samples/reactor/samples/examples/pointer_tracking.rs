#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, Element, PointerEvent, RenderCx, TextBlock, Thickness, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let position = cx.use_state(|| None::<(f32, f32)>);
    let inside = cx.use_state(|| false);

    let label = match (inside.value(), position.value()) {
        (true, Some((x, y))) => format!("Tracking at ({x:.0}, {y:.0})"),
        (true, None) => "Pointer entered".to_string(),
        (false, _) => "Move the pointer into the box".to_string(),
    };

    let enter_position = position.clone();
    let enter_inside = inside.clone();
    let move_position = position.clone();
    let exit_position = position;
    let exit_inside = inside;
    let fill = if enter_inside.value() {
        Color::rgb(40, 160, 90)
    } else {
        Color::rgb(40, 120, 200)
    };

    vstack(
        12.0,
        [
            TextBlock::new(label)
                .font_size(20.0)
                .automation_id("pointer-label")
                .build(),
            Border::new(
                TextBlock::new("Move the pointer over me")
                    .foreground(Color::rgb(255, 255, 255))
                    .build(),
            )
            .background(fill)
            .padding(Thickness::uniform(40.0))
            .width(360.0)
            .height(240.0)
            .on_pointer_entered(move |event: PointerEvent| {
                enter_inside.set(true);
                enter_position.set(Some((event.x, event.y)));
            })
            .on_pointer_moved(move |event: PointerEvent| {
                move_position.set(Some((event.x, event.y)));
            })
            .on_pointer_exited(move |_event: PointerEvent| {
                exit_inside.set(false);
                exit_position.set(None);
            })
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Pointer Tracking", app)
}

#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, Element, Orientation, RenderCx, StackPanel, TextBlock, Thickness, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let width = cx.use_state(|| 260.0_f64);
    let current_width = width.value();
    let drag_start = cx.use_ref(|| None::<(f64, f64)>);
    let pressed_start = drag_start.clone();
    let moved_start = drag_start.clone();
    let released_start = drag_start.clone();
    let lost_start = drag_start.clone();

    let handle = Border::new(
        TextBlock::new("Drag")
            .foreground(Color::rgb(255, 255, 255))
            .build(),
    )
    .background(Color::rgb(90, 90, 100))
    .width(44.0)
    .on_pointer_pressed(move |event| {
        if event.is_left_button_pressed && event.capture_succeeded {
            pressed_start.set(Some((f64::from(event.window_x), current_width)));
        }
    })
    .on_pointer_moved(move |event| {
        if !event.is_left_button_pressed {
            moved_start.set(None);
            return;
        }
        if let Some((start_x, start_width)) = moved_start.get().flatten() {
            width.set((start_width + f64::from(event.window_x) - start_x).clamp(140.0, 520.0));
        }
    })
    .on_pointer_released(move |_| {
        released_start.set(None);
    })
    .on_pointer_capture_lost(move |_| {
        lost_start.set(None);
    })
    .on_pointer_canceled(move |_| {
        drag_start.set(None);
    })
    .capture_pointer_on_press()
    .build();

    vstack(
        12.0,
        [
            TextBlock::new(format!("Left pane width: {current_width:.0} DIPs"))
                .automation_id("resize-status")
                .build(),
            StackPanel::new([
                Border::new(
                    TextBlock::new("Resizable pane")
                        .padding(Thickness::uniform(16.0))
                        .build(),
                )
                .background(Color::rgb(35, 90, 150))
                .width(current_width)
                .build(),
                handle,
                Border::new(
                    TextBlock::new("The handle moves, but window_x remains stable.").build(),
                )
                .padding(Thickness::uniform(16.0))
                .build(),
            ])
            .orientation(Orientation::Horizontal)
            .height(240.0)
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Pointer Resize", app)
}

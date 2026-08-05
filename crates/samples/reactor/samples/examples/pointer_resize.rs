#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (width, set_width) = cx.use_state(260.0_f64);
    let width_ref = cx.use_ref(width);
    width_ref.set(width);
    let drag_start = cx.use_ref(None::<(f64, f64)>);

    let on_pressed = cx.use_callback((), {
        let drag_start = drag_start.clone();
        move |info: PointerEventInfo| {
            if info.is_left_button_pressed && info.capture_succeeded {
                drag_start.set(Some((info.window_x, width_ref.get_cloned())));
            }
        }
    });
    let on_moved = cx.use_callback((), {
        let drag_start = drag_start.clone();
        move |info: PointerEventInfo| {
            if !info.is_left_button_pressed {
                drag_start.set(None);
                return;
            }
            if let Some((start_x, start_width)) = drag_start.get_cloned() {
                set_width.call((start_width + info.window_x - start_x).clamp(140.0, 520.0));
            }
        }
    });
    let on_released = cx.use_callback((), {
        let drag_start = drag_start.clone();
        move |_: PointerEventInfo| drag_start.set(None)
    });
    let on_capture_ended = cx.use_callback((), move |()| drag_start.set(None));

    vstack((
        TitleBar::new("windows_reactor - pointer resize"),
        text_block(format!("Left pane width: {width:.0} DIPs")),
        hstack((
            border(text_block("Resizable pane").padding(Thickness::uniform(16.0)))
                .width(width)
                .background(Color::rgb(35, 90, 150)),
            border(text_block("Drag").foreground(Color::rgb(255, 255, 255)))
                .width(44.0)
                .background(Color::rgb(90, 90, 100))
                .on_pointer_pressed(on_pressed)
                .on_pointer_moved(on_moved)
                .on_pointer_released(on_released)
                .on_pointer_capture_lost(on_capture_ended.clone())
                .on_pointer_canceled(on_capture_ended)
                .capture_pointer_on_press(),
            border(text_block("The handle moves, but window_x remains stable."))
                .padding(Thickness::uniform(16.0)),
        ))
        .height(240.0),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Pointer Resize", app)
}

#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (count, set_count) = cx.use_state(0_i32);

    let dec = decrement_handler(set_count.clone(), count);
    let inc = increment_handler(set_count.clone(), count);
    let reset = reset_handler(set_count);

    let reset_for_accel = reset.clone();
    let reset_accelerator =
        KeyboardAccelerator::new(KeyboardKey::R, KeyModifiers::CONTROL, reset_for_accel);

    vstack((
        TitleBar::new("windows_reactor — counter").subtitle("Phase 1 demo"),
        text_block(format!("Count: {count}"))
            .bold()
            .font_size(28.0)
            .heading_level(HeadingLevel::Level1)
            .automation_id("count-label"),
        hstack((
            button("-")
                .on_click(dec)
                .automation_name("Decrement")
                .automation_id("decrement-button"),
            button("+")
                .on_click(inc)
                .automation_name("Increment")
                .automation_id("increment-button"),
            button("reset (Ctrl+R)")
                .on_click(reset)
                .automation_id("reset-button")
                .keyboard_accelerator(reset_accelerator),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0)
}

fn decrement_handler(set: SetState<i32>, current: i32) -> impl Fn() + 'static {
    move || set.call(current - 1)
}

fn increment_handler(set: SetState<i32>, current: i32) -> impl Fn() + 'static {
    move || set.call(current + 1)
}

fn reset_handler(set: SetState<i32>) -> impl Fn() + Clone + 'static {
    move || set.call(0)
}

fn main() -> Result<()> {
    App::new().title("windows_reactor — counter").render(app)
}

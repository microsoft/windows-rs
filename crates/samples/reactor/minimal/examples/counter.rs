#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    let dec = {
        let s = set_count.clone();
        move || s.call(count - 1)
    };
    let inc = {
        let s = set_count.clone();
        move || s.call(count + 1)
    };
    let reset = {
        let s = set_count.clone();
        move || s.call(0)
    };

    let reset_accel = {
        let s = set_count;
        KeyboardAccelerator::new(KeyboardKey::R, KeyModifiers::CONTROL, move || s.call(0))
    };

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
                .keyboard_accelerator(reset_accel),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("Counter", app)
}

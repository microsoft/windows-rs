#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("TypeRamp", || {
        let text = |value, size, weight| {
            TextBlock::new()
                .text(value)
                .font_size(size)
                .font_weight(weight)
        };
        StackPanel::new().spacing(8.0).children((
            text("Title - 28px Semibold", 28.0, FontWeight::SEMI_BOLD),
            text("Subtitle - 20px Semibold", 20.0, FontWeight::SEMI_BOLD),
            text("BodyLarge - 18px Normal", 18.0, FontWeight::NORMAL),
            text("BodyStrong - 14px Semibold", 14.0, FontWeight::SEMI_BOLD),
            text("Body - 14px Normal", 14.0, FontWeight::NORMAL),
            text("Caption - 12px Normal", 12.0, FontWeight::NORMAL),
            text("Custom weight", 14.0, FontWeight::new(325).unwrap()),
        ))
    })
    .unwrap();
}

#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Border", || {
        StackPanel::new().spacing(8.0).children((
            Border::new()
                .background(Color::rgb(60, 100, 180))
                .padding(Thickness::uniform(12.0))
                .content(
                    TextBlock::new()
                        .text("Boxed text")
                        .foreground(Color::rgb(255, 255, 255)),
                ),
            Border::new()
                .background(Color::rgb(80, 140, 90))
                .padding(Thickness::xy(16.0, 8.0))
                .margin(Thickness::uniform(4.0))
                .max_width(280.0)
                .content(
                    TextBlock::new()
                        .text("Margined + width-capped")
                        .font_weight(FontWeight::BOLD)
                        .foreground(Color::rgb(255, 255, 255)),
                ),
        ))
    })
    .unwrap();
}

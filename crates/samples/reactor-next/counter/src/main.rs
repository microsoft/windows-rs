use windows_reactor_next::*;

fn main() {
    let counter = StackPanel::new()
        .child("value", TextBlock::new().text("0"))
        .child(
            "increment",
            Button::new().content(TextBlock::new().text("+")),
        );

    println!("{counter:#?}");
}

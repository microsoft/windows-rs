#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (text, set_text) = cx.use_state(String::new());

    text_box(text)
        .multiline()
        .placeholder("Start typing…")
        .on_changed(set_text)
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .vertical_alignment(VerticalAlignment::Stretch)
        .into()
}

fn main() -> Result<()> {
    App::new().title("windows_reactor — notepad").render(app)
}

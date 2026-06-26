#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (pos, set_pos) = cx.use_state(None::<(f64, f64)>);

    let label = match pos {
        Some((x, y)) => format!("Pressed at ({x:.0}, {y:.0})"),
        None => "Click anywhere in the box".to_string(),
    };

    let on_press = move |info: PointerEventInfo| set_pos.call(Some((info.x, info.y)));

    vstack((
        TitleBar::new("windows_reactor — pointer position"),
        text_block(label)
            .font_size(20.0)
            .automation_id("pointer-label"),
        border(
            text_block("Click to read the pointer position").foreground(Color::rgb(255, 255, 255)),
        )
        .background(Color::rgb(40, 120, 200))
        .padding(Thickness::uniform(40.0))
        .width(360.0)
        .height(240.0)
        .on_pointer_pressed(on_press),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Pointer Position", app)
}

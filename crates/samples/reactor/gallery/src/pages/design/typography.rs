use crate::controls::page_content;
use windows_reactor::*;

pub fn typography_page(_: &(), _cx: &mut RenderCx) -> Element {
    let samples: Vec<Element> = vec![
        type_sample("Caption", 12.0, false),
        type_sample("Body", 14.0, false),
        type_sample("Body Strong", 14.0, true),
        type_sample("Subtitle", 20.0, true),
        type_sample("Title", 28.0, true),
        type_sample("Title Large", 40.0, true),
        type_sample("Display", 68.0, true),
    ];

    let ramp = vstack(samples).spacing(12.0).into();

    page_content(
        "Typography",
        "The WinUI 3 type ramp provides a set of named text styles for consistent hierarchy.",
        vec![ramp],
    )
    .into()
}

fn type_sample(name: &str, size: f64, bold: bool) -> Element {
    let mut tb = text_block(format!("{name} — {size}pt")).font_size(size);
    if bold {
        tb = tb.bold();
    }
    tb.into()
}

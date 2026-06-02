//! Minimal sample for rich tooltip content.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let rich_panel = vstack((
        text_block("Action: Save").bold(),
        text_block("Writes the current document to disk."),
    ))
    .spacing(4.0);

    vstack((
        button("Save").tooltip_with(Tooltip::rich(rich_panel)),
        button("Open").tooltip("Opens a document"),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("tooltip_rich sample").render(app)
}

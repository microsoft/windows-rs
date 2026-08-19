#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, DropFormats, DropOperation, DropTarget, Element, HorizontalAlignment, RenderCx,
    TextBlock, TextWrapping, Thickness, VerticalAlignment,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let dropped = cx.use_state(|| "Drop files or some text here".to_string());
    let current = dropped.value();
    let target = DropTarget::new(
        DropOperation::Copy,
        DropFormats::TEXT | DropFormats::STORAGE_ITEMS,
    );

    Border::new(
        TextBlock::new(current)
            .text_wrapping(TextWrapping::Wrap)
            .font_size(24.0)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .build(),
    )
    .background(Color::rgb(245, 245, 245))
    .padding(Thickness::uniform(20.0))
    .horizontal_alignment(HorizontalAlignment::Stretch)
    .vertical_alignment(VerticalAlignment::Stretch)
    .margin(Thickness::uniform(40.0))
    .on_drop(target, move |result| {
        let label = match result {
            Ok(event) if !event.storage_items.is_empty() => {
                if event.storage_items.len() == 1 {
                    event.storage_items[0].path.clone()
                } else {
                    let names = event
                        .storage_items
                        .iter()
                        .map(|item| item.name.as_str())
                        .collect::<Vec<_>>();
                    format!("{} files dropped: {}", names.len(), names.join(", "))
                }
            }
            Ok(event) => event.text.unwrap_or_else(|| "Unsupported drop".to_string()),
            Err(error) => format!("Drop failed: {error}"),
        };
        dropped.set(label);
    })
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Drag and Drop", app)
}

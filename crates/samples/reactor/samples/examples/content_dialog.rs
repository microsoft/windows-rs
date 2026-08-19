#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, ContentDialog, ContentDialogResult, Element, RenderCx, StackPanel, TextBlock,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| false);
    let result = cx.use_state(|| None::<ContentDialogResult>);
    let current_open = open.value();
    let current_result = result.value();
    let show = open.clone();
    let close = open;

    let label = match current_result {
        None => "No choice yet.",
        Some(ContentDialogResult::Primary) => "You picked: Delete",
        Some(ContentDialogResult::Secondary) => "You picked: Archive",
        Some(ContentDialogResult::None) => "You picked: Cancel",
    };

    StackPanel::new([
        TextBlock::new(label).automation_id("dialog-result").build(),
        Button::new("Open dialog")
            .on_click(move || {
                show.set(true);
            })
            .automation_id("open-dialog")
            .build(),
        ContentDialog::new(
            "Delete this item?",
            TextBlock::new("This action cannot be undone.").build(),
        )
        .primary_button("Delete")
        .secondary_button("Archive")
        .close_button("Cancel")
        .open(current_open)
        .on_closed(move |value| {
            result.set(Some(value));
            close.set(false);
        })
        .build(),
    ])
    .spacing(8.0)
    .max_width(320.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ContentDialog", app)
}

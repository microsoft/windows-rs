//! Minimal sample for the `ContentDialog` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (open, set_open) = cx.use_state(false);
    let (result, set_result) = cx.use_state::<Option<ContentDialogResult>>(None);

    let close = set_open.clone();
    let record = set_result;
    let on_closed = move |r: ContentDialogResult| {
        record.call(Some(r));
        close.call(false);
    };

    let show = move || set_open.call(true);

    let label = match result {
        None => "No choice yet.".to_string(),
        Some(ContentDialogResult::Primary) => "You picked: Delete".to_string(),
        Some(ContentDialogResult::Secondary) => "You picked: Archive".to_string(),
        Some(ContentDialogResult::None) => "You picked: Cancel".to_string(),
    };

    vstack((
        text_block(label),
        button("Open dialog").on_click(show),
        ContentDialog::new("Delete this item?")
            .content("This action cannot be undone.")
            .primary_button_text("Delete")
            .secondary_button_text("Archive")
            .close_button_text("Cancel")
            .is_open(open)
            .on_closed(on_closed),
    ))
    .spacing(8.0)
    .max_width(320.0)
}

fn main() -> Result<()> {
    App::new().title("ContentDialog").render(app)
}

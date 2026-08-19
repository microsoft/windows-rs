#![windows_subsystem = "windows"]

use windows_reactor::{
    Application, ApplicationResource, Button, CollectionSelection, Color, ContentDialog,
    ContentDialogResult, Element, ListBox, RenderCx, StackPanel, TextBlock, TextBox, Thickness,
    Window,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let main_open = cx.use_state(|| true);
    let secondary_open = cx.use_state(|| false);
    let dialog_open = cx.use_state(|| false);
    let text = cx.use_state(|| "Initial value".to_string());
    let selected = cx.use_state(|| None::<u64>);
    let dialog_result = cx.use_state(|| ContentDialogResult::None);

    let current_text = text.value();
    let current_selected = selected.value();
    let current_dialog_open = dialog_open.value();
    let current_dialog_result = dialog_result.value();
    let mut windows = Vec::new();

    if main_open.value() {
        let show_secondary = secondary_open.clone();
        let show_dialog = dialog_open.clone();
        let close_dialog = dialog_open;
        let update_dialog_result = dialog_result;
        let close_main = main_open;

        let content = StackPanel::new([
            TextBlock::new("Reactor public acceptance application")
                .automation_name("Acceptance heading")
                .build(),
            TextBox::new(current_text.clone(), move |value| {
                text.set(value);
            })
            .header("Controlled input")
            .automation_name("Acceptance text input")
            .build(),
            TextBlock::new(format!("Current text: {current_text}"))
                .automation_name("Acceptance text status")
                .build(),
            ListBox::new(
                [(10, "Alpha"), (20, "Beta"), (30, "Gamma")],
                move |selection| {
                    selected.set(selection.as_slice().first().copied());
                },
            )
            .selection(CollectionSelection::new(current_selected))
            .automation_name("Acceptance collection")
            .build(),
            TextBlock::new(format!("Selected key: {current_selected:?}"))
                .automation_name("Acceptance selection status")
                .build(),
            Button::new("Open secondary window")
                .on_click(move || {
                    show_secondary.set(true);
                })
                .automation_name("Open acceptance secondary window")
                .build(),
            Button::new("Open dialog")
                .on_click(move || {
                    show_dialog.set(true);
                })
                .automation_name("Open acceptance dialog")
                .build(),
            TextBlock::new(format!("Dialog result: {current_dialog_result:?}"))
                .automation_name("Acceptance dialog status")
                .build(),
            ContentDialog::new(
                "Acceptance dialog",
                TextBlock::new("The overlay is owned by the main window.").build(),
            )
            .primary_button("Accept")
            .close_button("Cancel")
            .open(current_dialog_open)
            .on_closed(move |result| {
                update_dialog_result.set(result);
                close_dialog.set(false);
            })
            .build(),
        ])
        .spacing(10.0)
        .padding(Thickness::uniform(20.0))
        .build();

        windows.push(
            Window::new("Reactor acceptance", content, move || {
                close_main.set(false);
            })
            .client_size(480.0, 600.0)
            .build()
            .key(1),
        );
    }

    if secondary_open.value() {
        let close_secondary = secondary_open.clone();
        let close_from_button = secondary_open;
        let content = StackPanel::new([
            TextBlock::new("Secondary window")
                .automation_name("Acceptance secondary heading")
                .build(),
            Button::new("Close secondary window")
                .on_click(move || {
                    close_from_button.set(false);
                })
                .automation_name("Close acceptance secondary window")
                .build(),
        ])
        .spacing(10.0)
        .padding(Thickness::uniform(20.0))
        .build();

        windows.push(
            Window::new("Reactor acceptance secondary", content, move || {
                close_secondary.set(false);
            })
            .client_size(360.0, 180.0)
            .build()
            .key(2),
        );
    }

    Application::new(windows)
        .resources([
            (
                "AcceptanceAccent",
                ApplicationResource::from(Color::rgb(0, 120, 212)),
            ),
            (
                "AcceptanceSpacing",
                ApplicationResource::from(Thickness::uniform(8.0)),
            ),
        ])
        .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}

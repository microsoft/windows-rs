#![windows_subsystem = "windows"]

use windows_reactor::{
    Application, Border, Button, CheckBox, CollectionSelection, Color, CommandBar, CommandBarItem,
    ContentDialog, ContentDialogResult, Image, ImageSource, ListBox, TextBlock, TextBox, Thickness,
    VirtualItemKeys, VirtualList, Window, border, button, component, hstack, run_reactor_winui_app,
    stack_panel, text_block,
};

fn item_name(key: u64) -> &'static str {
    match key {
        10 => "Alpha",
        20 => "Beta",
        30 => "Gamma",
        _ => unreachable!(),
    }
}

fn main() -> windows_core::Result<()> {
    let root = component(|cx| {
        let open = cx.use_state(|| true);
        let secondary_open = cx.use_state(|| false);
        let secondary_count = cx.use_state(|| 0u32);
        let last_command = cx.use_state(|| "none".to_string());
        let dialog_open = cx.use_state(|| false);
        let dialog_result = cx.use_state(|| "none".to_string());
        let state = cx.use_state(|| 5_000usize);
        let count = state.value();
        let update = state;
        let keys = cx.use_state(|| VirtualItemKeys::new([10, 20, 30]));
        let current_keys = keys.value();
        let order = current_keys
            .as_slice()
            .iter()
            .map(|key| item_name(*key))
            .collect::<Vec<_>>()
            .join(", ");
        let row_keys = current_keys.clone();
        let input = cx.use_state(|| "initial".to_string());
        let current_input = input.value();
        let checked = cx.use_state(|| false);
        let current_checked = checked.value();
        let selected = cx.use_state(|| None::<u64>);
        let current_selected = selected.value();
        let selection = match current_selected {
            Some(10) => "Alpha choice",
            Some(20) => "Beta choice",
            Some(30) => "Gamma choice",
            _ => "none",
        };
        let current_secondary_count = secondary_count.value();
        let current_command = last_command.value();
        let current_dialog_open = dialog_open.value();
        let current_dialog_result = dialog_result.value();
        let show_secondary = secondary_open.clone();
        let increment_secondary_count = secondary_count;
        let controls = stack_panel([
            TextBox::new(current_input.clone(), move |value| {
                input.set(value);
            })
            .automation_name("Matched text input")
            .build(),
            TextBlock::new(format!("Input: {current_input}")).build(),
            CheckBox::new("Matched toggle", current_checked, move |value| {
                checked.set(value);
            })
            .build(),
            TextBlock::new(format!("Toggle: {current_checked}")).build(),
            ListBox::new(
                [
                    (10, "Alpha choice"),
                    (20, "Beta choice"),
                    (30, "Gamma choice"),
                ],
                move |selection| {
                    selected.set(selection.as_slice().first().copied());
                },
            )
            .selection(CollectionSelection::new(current_selected))
            .height(80.0)
            .build(),
            TextBlock::new(format!("Selection: {selection}")).build(),
        ]);
        let show_dialog = dialog_open.clone();
        let close_dialog = dialog_open;
        let actions = stack_panel([
            CommandBar::new([CommandBarItem::button(10, "Matched command", move || {
                last_command.set("Matched command".to_string());
            })])
            .build(),
            TextBlock::new(format!("Command: {current_command}")).build(),
            Button::new("Open matched dialog")
                .on_click(move || {
                    show_dialog.set(true);
                })
                .build(),
            ContentDialog::new(
                "Matched dialog",
                TextBlock::new("Matched dialog content").build(),
            )
            .primary_button("Accept matched")
            .close_button("Cancel matched")
            .open(current_dialog_open)
            .on_closed(move |value| {
                let result = match value {
                    ContentDialogResult::Primary => "primary",
                    ContentDialogResult::Secondary => "secondary",
                    ContentDialogResult::None => "none",
                };
                dialog_result.set(result.to_string());
                close_dialog.set(false);
            })
            .build(),
            TextBlock::new(format!("Dialog: {current_dialog_result}")).build(),
            Image::new(ImageSource::bitmap(format!(
                "file:///{}/../../../tests/libs/canvas/test.png",
                env!("CARGO_MANIFEST_DIR").replace('\\', "/")
            )))
            .width(120.0)
            .height(60.0)
            .automation_name("Matched image")
            .build(),
        ]);
        let content = stack_panel([
            border(text_block(format!("Declarative rows: {count}"))),
            button("Toggle row count", move || {
                update.update(|value| {
                    *value = if *value == 5_000 { 10_000 } else { 5_000 };
                });
            }),
            VirtualList::new(count, 200.0, |index| {
                TextBlock::new(format!("Declarative row {index}"))
                    .height(match index % 3 {
                        0 => 24.0,
                        1 => 40.0,
                        _ => 72.0,
                    })
                    .build()
            })
            .build(),
            TextBlock::new(format!("Order: {order}")).build(),
            Button::new("Rotate rows")
                .on_click(move || {
                    keys.update(|keys| {
                        let mut values = keys.as_slice().to_vec();
                        values.rotate_left(1);
                        *keys = VirtualItemKeys::new(values);
                    });
                })
                .build(),
            VirtualList::new(current_keys.len(), 100.0, move |index| {
                let key = row_keys.as_slice()[index];
                component(move |cx| {
                    let clicks = cx.use_state(|| 0u32);
                    let count = clicks.value();
                    let name = item_name(key);
                    hstack(
                        8.0,
                        [
                            TextBlock::new(format!("{name}: {count}")).build(),
                            Button::new(format!("Increment {name}"))
                                .on_click(move || {
                                    clicks.update(|value| *value += 1);
                                })
                                .build(),
                        ],
                    )
                })
            })
            .item_keys(current_keys)
            .build(),
            Border::new(
                TextBlock::new("Matched styled content")
                    .foreground(Color::rgb(255, 255, 255))
                    .build(),
            )
            .background(Color::rgb(60, 100, 180))
            .padding(Thickness::uniform(12.0))
            .width(240.0)
            .height(50.0)
            .automation_name("Matched styled panel")
            .help_text("Styled panel help")
            .build(),
            controls,
            Button::new("Open matched secondary")
                .on_click(move || {
                    increment_secondary_count.update(|value| *value += 1);
                    show_secondary.set(true);
                })
                .build(),
            TextBlock::new(format!(
                "Secondary windows opened: {current_secondary_count}"
            ))
            .build(),
            actions,
        ]);
        let mut windows = Vec::new();
        if open.value() {
            windows.push(
                Window::new("windows-reactor matched workload", content, move || {
                    open.set(false);
                })
                .client_size(600.0, 1000.0)
                .build()
                .key(0),
            );
        }
        if secondary_open.value() {
            let close = secondary_open;
            let secondary_content = component(|cx| {
                let count = cx.use_state(|| 0u32);
                let current = count.value();
                stack_panel([
                    text_block(format!("Secondary count: {current}")),
                    button("Increment secondary", move || {
                        count.update(|value| *value += 1);
                    }),
                ])
            });
            windows.push(
                Window::new(
                    "windows-reactor matched secondary",
                    secondary_content,
                    move || {
                        close.set(false);
                    },
                )
                .client_size(360.0, 180.0)
                .build()
                .key(1),
            );
        }
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}

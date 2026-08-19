#![windows_subsystem = "console"]

use windows_canvas::ColorF;
use windows_reactor::{
    Application, AutoSuggestBox, BreadcrumbBar, Button, CalendarDatePicker, CollectionSelection,
    Color, ColorPicker, ComboBox, CommandBar, CommandBarDefaultLabelPosition, CommandBarItem,
    ContentDialog, ContentDialogResult, DatePicker, DateTime, DropDownButton, DropFormats,
    DropOperation, DropTarget, Element, Icon, IconSymbol, Image, ImageSource, KeyboardAccelerator,
    ListBox, ListBoxItem, ListBoxItems, NumberBox, PasswordBox, RadioButton, RadioButtons,
    RatingControl, RenderCx, SelectionMode, SelectorBar, SelectorBarItem, SelectorItem,
    SelectorItems, Slider, StackPanel, State, TeachingTip, TextBlock, TextBox, VirtualGrid,
    VirtualKey, VirtualKeyModifiers, VirtualList, Window, animated_canvas, border, button,
    canvas_image_invalidated, check_box, component, fragment, run_reactor_winui,
    run_reactor_winui_app, stack_panel, swap_chain_canvas_invalidated, text_block,
};

trait StateTestExt<T> {
    fn get(&self) -> Option<T>;
}

impl<T: Clone + 'static> StateTestExt<T> for State<T> {
    fn get(&self) -> Option<T> {
        self.try_value()
    }
}

fn multi_window(cx: &mut RenderCx) -> Element {
    let first_open = cx.use_state(|| true);
    let second_open = cx.use_state(|| true);
    let first_close_requests = cx.use_state(|| 0usize);
    let second_count = cx.use_state(|| 0usize);
    let first_dialog_open = cx.use_state(|| false);
    let second_dialog_open = cx.use_state(|| false);
    let mut windows = Vec::new();

    let owned_windows = if second_open.get().unwrap() {
        let close = second_open;
        let count = second_count;
        let current_count = count.get().unwrap();
        let dialog_open = second_dialog_open;
        let current_dialog_open = dialog_open.get().unwrap();
        let open_dialog = dialog_open.clone();
        let close_dialog = dialog_open;
        vec![
            Window::new(
                "windows-reactor secondary window",
                stack_panel([
                    button("Increment second window", move || {
                        count.set(current_count + 1);
                    }),
                    text_block(format!("Second window count: {current_count}")),
                    button("Open second window dialog", move || {
                        open_dialog.set(true);
                    }),
                    ContentDialog::new(
                        "Second window dialog",
                        text_block("Second window dialog content"),
                    )
                    .close_button("Close second dialog")
                    .open(current_dialog_open)
                    .on_closed(move |_| {
                        close_dialog.set(false);
                    })
                    .build(),
                ]),
                move || {
                    close.set(false);
                },
            )
            .build()
            .key(2),
        ]
    } else {
        Vec::new()
    };

    if first_open.get().unwrap() {
        let close = first_open;
        let requests = first_close_requests;
        let current_requests = requests.get().unwrap();
        let dialog_open = first_dialog_open;
        let current_dialog_open = dialog_open.get().unwrap();
        let open_dialog = dialog_open.clone();
        let close_dialog = dialog_open;
        windows.push(
            Window::new(
                "windows-reactor native self-test",
                stack_panel([
                    text_block("First window content"),
                    text_block(format!("First close requests: {current_requests}")),
                    button("Open first window dialog", move || {
                        open_dialog.set(true);
                    }),
                    ContentDialog::new(
                        "First window dialog",
                        text_block("First window dialog content"),
                    )
                    .close_button("Close first dialog")
                    .open(current_dialog_open)
                    .on_closed(move |_| {
                        close_dialog.set(false);
                    })
                    .build(),
                ]),
                move || {
                    let next = current_requests + 1;
                    requests.set(next);
                    if next >= 2 {
                        close.set(false);
                    }
                },
            )
            .owned_windows(owned_windows)
            .build()
            .key(1),
        );
    }

    Application::new(windows).build()
}

fn interactive(cx: &mut RenderCx) -> Element {
    let count = cx.use_state(|| 5_000usize);
    let current = count.get().unwrap();
    let update = count.clone();
    let clear_rows = count;
    let invoked = cx.use_state(|| None::<u64>);
    let current_invoked = invoked.get().unwrap();
    let update_invoked = invoked;
    let order = cx.use_state(|| vec![1_u64, 2, 3]);
    let current_order = order.get().unwrap();
    let reorder = order;
    let keyed = stack_panel(current_order.into_iter().map(|key| {
        let reorder = reorder.clone();
        button(format!("Item {key}"), move || {
            if key == 2 {
                reorder.set(vec![3, 1, 2]);
            }
        })
        .key(key)
    }));
    let text = cx.use_state(|| "initial".to_string());
    let current_text = text.get().unwrap();
    let update_text = text.clone();
    let text_events = cx.use_state(|| 0usize);
    let current_text_events = text_events.get().unwrap();
    let update_text_events = text_events;
    let password = cx.use_state(String::new);
    let current_password = password.get().unwrap();
    let update_password = password;
    let password_events = cx.use_state(|| 0usize);
    let current_password_events = password_events.get().unwrap();
    let update_password_events = password_events;
    let slider = cx.use_state(|| 25.0);
    let current_slider = slider.get().unwrap();
    let update_slider = slider;
    let slider_events = cx.use_state(|| 0usize);
    let current_slider_events = slider_events.get().unwrap();
    let update_slider_events = slider_events;
    let number = cx.use_state(|| Some(10.0));
    let current_number = number.get().unwrap();
    let update_number = number.clone();
    let number_events = cx.use_state(|| 0usize);
    let current_number_events = number_events.get().unwrap();
    let update_number_events = number_events;
    let rating = cx.use_state(|| Some(3.0));
    let current_rating = rating.get().unwrap();
    let update_rating = rating.clone();
    let rating_events = cx.use_state(|| 0usize);
    let current_rating_events = rating_events.get().unwrap();
    let update_rating_events = rating_events;
    let color = cx.use_state(|| Color::rgb(255, 0, 0));
    let current_color = color.get().unwrap();
    let update_color = color.clone();
    let color_events = cx.use_state(|| 0usize);
    let current_color_events = color_events.get().unwrap();
    let update_color_events = color_events;
    let date = cx.use_state(|| Some(DateTime::from_unix_secs(1_705_276_800)));
    let current_date = date.get().unwrap();
    let update_date = date.clone();
    let date_events = cx.use_state(|| 0usize);
    let current_date_events = date_events.get().unwrap();
    let update_date_events = date_events;
    let calendar_date = cx.use_state(|| Some(DateTime::from_unix_secs(1_705_276_800)));
    let current_calendar_date = calendar_date.get().unwrap();
    let update_calendar_date = calendar_date.clone();
    let calendar_date_events = cx.use_state(|| 0usize);
    let current_calendar_date_events = calendar_date_events.get().unwrap();
    let update_calendar_date_events = calendar_date_events;
    let checked = cx.use_state(|| false);
    let current_checked = checked.get().unwrap();
    let update_checked = checked.clone();
    let toggle_events = cx.use_state(|| 0usize);
    let current_toggle_events = toggle_events.get().unwrap();
    let update_toggle_events = toggle_events;
    let radio_selection = cx.use_state(|| 0usize);
    let current_radio_selection = radio_selection.get().unwrap();
    let select_first_radio = radio_selection.clone();
    let select_second_radio = radio_selection;
    let programmatic_text = text;
    let programmatic_checked = checked;
    let programmatic_number = number;
    let programmatic_rating = rating;
    let programmatic_color = color;
    let programmatic_date = date;
    let programmatic_calendar_date = calendar_date;
    let accelerator_count = cx.use_state(|| 0usize);
    let current_accelerator_count = accelerator_count.get().unwrap();
    let update_accelerator_count = accelerator_count;
    let pointer_pressed = cx.use_state(|| 0usize);
    let current_pointer_pressed = pointer_pressed.get().unwrap();
    let update_pointer_pressed = pointer_pressed;
    let pointer_moved = cx.use_state(|| false);
    let current_pointer_moved = pointer_moved.get().unwrap();
    let update_pointer_moved = pointer_moved;
    let pointer_released = cx.use_state(|| 0usize);
    let current_pointer_released = pointer_released.get().unwrap();
    let update_pointer_released = pointer_released;
    let pointer_capture_lost = cx.use_state(|| 0usize);
    let current_pointer_capture_lost = pointer_capture_lost.get().unwrap();
    let update_pointer_capture_lost = pointer_capture_lost;
    let capture_succeeded = cx.use_state(|| false);
    let current_capture_succeeded = capture_succeeded.get().unwrap();
    let update_capture_succeeded = capture_succeeded;
    let dropped_text = cx.use_state(|| "none".to_string());
    let current_dropped_text = dropped_text.get().unwrap();
    let update_dropped_text = dropped_text;
    StackPanel::new([
        TextBlock::new("Drop target")
            .width(160.0)
            .height(48.0)
            .on_drop(
                DropTarget::new(DropOperation::Copy, DropFormats::TEXT),
                move |result| {
                    update_dropped_text.set(match result {
                        Ok(event) => event.text.unwrap_or_else(|| "missing text".to_string()),
                        Err(error) => format!("error: {error}"),
                    });
                },
            )
            .build(),
        text_block(format!("Dropped text: {current_dropped_text}")),
        TextBlock::new("Pointer target")
            .width(160.0)
            .height(48.0)
            .on_pointer_pressed(move |event| {
                update_capture_succeeded.set(event.capture_succeeded);
                update_pointer_pressed.set(update_pointer_pressed.get().unwrap() + 1);
            })
            .on_pointer_moved(move |_| {
                update_pointer_moved.set(true);
            })
            .on_pointer_released(move |_| {
                update_pointer_released.set(update_pointer_released.get().unwrap() + 1);
            })
            .on_pointer_capture_lost(move |_| {
                update_pointer_capture_lost.set(update_pointer_capture_lost.get().unwrap() + 1);
            })
            .capture_pointer_on_press()
            .build(),
        text_block(format!(
            "Pointer pressed: {current_pointer_pressed} capture: {current_capture_succeeded}"
        )),
        text_block(format!("Pointer moved: {current_pointer_moved}")),
        text_block(format!("Pointer released: {current_pointer_released}")),
        text_block(format!(
            "Pointer capture lost: {current_pointer_capture_lost}"
        )),
        RatingControl::new(current_rating, move |value| {
            update_rating.set(value);
            update_rating_events.set(current_rating_events + 1);
        })
        .placeholder(4.0)
        .caption("Rating")
        .automation_name("Native rating")
        .build(),
        text_block(format!(
            "Rating value: {}",
            current_rating.map_or_else(|| "empty".to_string(), |value| value.to_string())
        )),
        text_block(format!("Rating events: {current_rating_events}")),
        ColorPicker::new(current_color, move |value| {
            update_color.set(value);
            update_color_events.set(current_color_events + 1);
        })
        .alpha_enabled(false)
        .color_slider_visible(false)
        .color_channel_text_input_visible(false)
        .automation_name("Native color picker")
        .build(),
        text_block(format!(
            "Color value: #{:02X}{:02X}{:02X}",
            current_color.r, current_color.g, current_color.b
        )),
        text_block(format!("Color events: {current_color_events}")),
        DatePicker::new(current_date, move |value| {
            update_date.set(value);
            update_date_events.set(current_date_events + 1);
        })
        .automation_name("Native date picker")
        .build(),
        text_block(format!(
            "DatePicker value: {}",
            current_date.map_or_else(
                || "empty".to_string(),
                |value| value.unix_secs().to_string()
            )
        )),
        text_block(format!("DatePicker events: {current_date_events}")),
        CalendarDatePicker::new(current_calendar_date, move |value| {
            update_calendar_date.set(value);
            update_calendar_date_events.set(current_calendar_date_events + 1);
        })
        .header("Calendar date")
        .placeholder_text("Choose a calendar date")
        .today_highlighted(true)
        .automation_name("Native calendar date picker")
        .build(),
        text_block(format!(
            "CalendarDatePicker value: {}",
            current_calendar_date.map_or_else(
                || "empty".to_string(),
                |value| value.unix_secs().to_string()
            )
        )),
        text_block(format!(
            "CalendarDatePicker events: {current_calendar_date_events}"
        )),
        fragment([
            border(text_block(format!("Rows: {current}"))),
            Button::new("Toggle row count")
                .on_click(move || {
                    update.set(if update.get().unwrap() == 5_000 {
                        10_000
                    } else {
                        5_000
                    });
                })
                .automation_name("Rows toggle")
                .help_text("Changes the virtual row count")
                .build(),
            Button::new("Clear rows")
                .on_click(move || {
                    clear_rows.set(0);
                })
                .automation_name("Rows clear")
                .help_text("Shows the empty list state")
                .build(),
        ]),
        keyed,
        text_block(format!(
            "Invoked row key: {}",
            current_invoked.map_or_else(|| "none".to_string(), |key| key.to_string())
        )),
        VirtualList::new(current, 640.0, |index| {
            TextBlock::new(format!("Row {index}"))
                .height(match index % 3 {
                    0 => 24.0,
                    1 => 40.0,
                    _ => 72.0,
                })
                .build()
        })
        .empty_state(text_block("No rows available"))
        .automation_name("Virtual rows")
        .help_text("Scrollable virtual row results")
        .on_item_invoked(move |key| {
            update_invoked.set(Some(key));
        })
        .build(),
        text_block(format!("Text value: {current_text}")),
        TextBox::new(current_text, move |value| {
            update_text.set(value);
            update_text_events.set(current_text_events + 1);
        })
        .automation_name("Text input")
        .build(),
        text_block(format!("Text events: {current_text_events}")),
        PasswordBox::new(current_password.clone(), move |value| {
            update_password.set(value);
            update_password_events.set(current_password_events + 1);
        })
        .automation_name("Password input")
        .build(),
        text_block(format!("Password length: {}", current_password.len())),
        text_block(format!("Password events: {current_password_events}")),
        Slider::new(current_slider, move |value| {
            update_slider.set(value);
            update_slider_events.set(current_slider_events + 1);
        })
        .automation_name("Native slider")
        .width(240.0)
        .build(),
        text_block(format!("Slider value: {current_slider}")),
        text_block(format!("Slider events: {current_slider_events}")),
        NumberBox::new(current_number, move |value| {
            update_number.set(value);
            update_number_events.set(current_number_events + 1);
        })
        .range(0.0, 100.0)
        .automation_name("Native number box")
        .width(240.0)
        .build(),
        text_block(format!(
            "NumberBox value: {}",
            current_number.map_or_else(|| "empty".to_string(), |value| value.to_string())
        )),
        text_block(format!("NumberBox events: {current_number_events}")),
        text_block(format!("Checked value: {current_checked}")),
        check_box("Native checkbox", current_checked, move |value| {
            update_checked.set(value);
            update_toggle_events.set(current_toggle_events + 1);
        }),
        text_block(format!("Toggle events: {current_toggle_events}")),
        RadioButton::new(
            "Native radio one",
            current_radio_selection == 0,
            move |checked| {
                if checked {
                    select_first_radio.set(0);
                }
            },
        )
        .group_name("native-radio-group")
        .build(),
        TextBlock::new("Radio separator").build(),
        RadioButton::new(
            "Native radio two",
            current_radio_selection == 1,
            move |checked| {
                if checked {
                    select_second_radio.set(1);
                }
            },
        )
        .group_name("native-radio-group")
        .build(),
        text_block(format!("Radio selection: {current_radio_selection}")),
        button("Programmatic controls", move || {
            programmatic_text.set("programmatic".to_string());
            programmatic_checked.set(true);
            programmatic_number.set(None);
            programmatic_rating.set(None);
            programmatic_color.set(Color::rgb(0, 0, 255));
            programmatic_date.set(None);
            programmatic_calendar_date.set(None);
        }),
        text_block(format!("Ctrl+S invocations: {current_accelerator_count}")),
    ])
    .keyboard_accelerator(KeyboardAccelerator::new(
        VirtualKey::S,
        VirtualKeyModifiers::CONTROL,
        move || {
            update_accelerator_count.set(update_accelerator_count.get().unwrap() + 1);
        },
    ))
    .build()
}

fn failure(cx: &mut RenderCx) -> Element {
    let font_size = cx.use_state(|| 14.0);
    let current = font_size.get().unwrap();
    let update = font_size;
    stack_panel([
        button("Trigger native failure", move || {
            update.set(-1.0);
        }),
        TextBlock::new("Native failure target")
            .font_size(current)
            .build(),
    ])
}

fn grid(cx: &mut RenderCx) -> Element {
    let populated = cx.use_state(|| true);
    let is_populated = populated.get().unwrap();
    let toggle_populated = populated;
    let selection = cx.use_state(CollectionSelection::default);
    let current_selection = selection.get().unwrap();
    let update_selection = selection;
    let invoked = cx.use_state(|| None::<u64>);
    let current_invoked = invoked.get().unwrap();
    let update_invoked = invoked;
    let count = if is_populated { 4_000 } else { 0 };
    let selected_keys = current_selection
        .as_slice()
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");

    stack_panel([
        Button::new("Toggle tiles")
            .on_click(move || {
                toggle_populated.set(!toggle_populated.get().unwrap());
            })
            .automation_name("Tiles toggle")
            .build(),
        TextBlock::new(format!(
            "Selected tile keys: {}",
            if selected_keys.is_empty() {
                "none"
            } else {
                &selected_keys
            }
        ))
        .automation_name("Selected tile status")
        .help_text(format!(
            "Selected tile keys: {}",
            if selected_keys.is_empty() {
                "none"
            } else {
                &selected_keys
            }
        ))
        .build(),
        TextBlock::new(format!(
            "Invoked tile key: {}",
            current_invoked.map_or_else(|| "none".to_string(), |key| key.to_string())
        ))
        .automation_name("Invoked tile status")
        .help_text(format!(
            "Invoked tile key: {}",
            current_invoked.map_or_else(|| "none".to_string(), |key| key.to_string())
        ))
        .build(),
        VirtualGrid::new(count, 520.0, |index| {
            TextBlock::new(format!("Tile {index}"))
                .width(160.0)
                .height(100.0)
                .automation_name(format!("Tile {index}"))
                .build()
        })
        .selection(current_selection, move |value| {
            update_selection.set(value);
        })
        .empty_state(text_block("No tiles available"))
        .automation_name("Virtual tiles")
        .help_text("Scrollable virtual tile results")
        .on_item_invoked(move |key| {
            update_invoked.set(Some(key));
        })
        .build(),
    ])
}

fn list_box(cx: &mut RenderCx) -> Element {
    let items = cx.use_state(|| {
        ListBoxItems::new([
            ListBoxItem::new(10, "Choice"),
            ListBoxItem::new(20, "Choice"),
            ListBoxItem::new(30, "Other"),
        ])
    });
    let current_items = items.get().unwrap();
    let reorder_items = items.clone();
    let toggle_items = items;
    let selection = cx.use_state(CollectionSelection::default);
    let current_selection = selection.get().unwrap();
    let update_selection = selection;
    let status = format!(
        "Selected choice keys: {}",
        current_selection
            .as_slice()
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );

    stack_panel([
        Button::new("Reorder choices")
            .on_click(move || {
                reorder_items.set(ListBoxItems::new([
                    ListBoxItem::new(30, "Other"),
                    ListBoxItem::new(10, "Choice"),
                    ListBoxItem::new(20, "Choice"),
                ]));
            })
            .automation_name("Reorder choices")
            .build(),
        Button::new("Toggle key 20")
            .on_click(move || {
                let current = toggle_items.get().unwrap();
                if current.as_slice().iter().any(|item| item.key() == 20) {
                    toggle_items.set(ListBoxItems::new(
                        current
                            .as_slice()
                            .iter()
                            .filter(|item| item.key() != 20)
                            .cloned(),
                    ));
                } else {
                    let mut next = current.as_slice().to_vec();
                    next.push(ListBoxItem::new(20, "Choice"));
                    toggle_items.set(ListBoxItems::new(next));
                }
            })
            .automation_name("Toggle choice key 20")
            .build(),
        TextBlock::new(&status)
            .automation_name("Choice selection status")
            .help_text(status)
            .build(),
        ListBox::from_items(current_items, move |value| {
            update_selection.set(value);
        })
        .selection_mode(SelectionMode::Multiple)
        .selection(current_selection)
        .height(240.0)
        .automation_name("Keyed choices")
        .help_text("Select keyed choices")
        .build(),
    ])
}

fn combo_box(cx: &mut RenderCx) -> Element {
    let items = cx.use_state(|| {
        SelectorItems::new([
            SelectorItem::new(10, "Choice"),
            SelectorItem::new(20, "Choice"),
            SelectorItem::new(30, "Other"),
        ])
    });
    let current_items = items.get().unwrap();
    let reorder_items = items;
    let selection = cx.use_state(|| None::<u64>);
    let current_selection = selection.get().unwrap();
    let update_selection = selection;
    let status = format!(
        "Selected combo key: {}",
        current_selection
            .map(|key| key.to_string())
            .unwrap_or_default()
    );

    stack_panel([
        Button::new("Reorder combo choices")
            .on_click(move || {
                reorder_items.set(SelectorItems::new([
                    SelectorItem::new(30, "Other"),
                    SelectorItem::new(10, "Choice"),
                    SelectorItem::new(20, "Choice"),
                ]));
            })
            .automation_name("Reorder combo choices")
            .build(),
        TextBlock::new(&status)
            .automation_name("Combo selection status")
            .help_text(status)
            .build(),
        ComboBox::from_items(current_items, move |value| {
            update_selection.set(value);
        })
        .selected_key(current_selection)
        .automation_name("Keyed combo choices")
        .help_text("Select one keyed combo choice")
        .build(),
    ])
}

fn radio_buttons(cx: &mut RenderCx) -> Element {
    let items = cx.use_state(|| {
        SelectorItems::new([
            SelectorItem::new(10, "Choice"),
            SelectorItem::new(20, "Choice"),
            SelectorItem::new(30, "Other"),
        ])
    });
    let current_items = items.get().unwrap();
    let reorder_items = items.clone();
    let toggle_items = items;
    let selection = cx.use_state(|| None::<u64>);
    let current_selection = selection.get().unwrap();
    let update_selection = selection;
    let status = format!(
        "Selected radio key: {}",
        current_selection
            .map(|key| key.to_string())
            .unwrap_or_default()
    );

    stack_panel([
        Button::new("Reorder radio choices")
            .on_click(move || {
                reorder_items.set(SelectorItems::new([
                    SelectorItem::new(30, "Other"),
                    SelectorItem::new(10, "Choice"),
                    SelectorItem::new(20, "Choice"),
                ]));
            })
            .automation_name("Reorder radio choices")
            .build(),
        Button::new("Toggle radio key 20")
            .on_click(move || {
                let current = toggle_items.get().unwrap();
                if current.as_slice().iter().any(|item| item.key() == 20) {
                    toggle_items.set(SelectorItems::new(
                        current
                            .as_slice()
                            .iter()
                            .filter(|item| item.key() != 20)
                            .cloned(),
                    ));
                } else {
                    let mut next = current.as_slice().to_vec();
                    next.push(SelectorItem::new(20, "Choice"));
                    toggle_items.set(SelectorItems::new(next));
                }
            })
            .automation_name("Toggle radio key 20")
            .build(),
        TextBlock::new(&status)
            .automation_name("Radio selection status")
            .help_text(status)
            .build(),
        RadioButtons::from_items(current_items, move |value| {
            update_selection.set(value);
        })
        .selected_key(current_selection)
        .max_columns(2)
        .automation_name("Keyed radio choices")
        .help_text("Select one keyed radio choice")
        .build(),
    ])
}

fn selector_bar(cx: &mut RenderCx) -> Element {
    let reversed = cx.use_state(|| false);
    let current_reversed = reversed.get().unwrap();
    let reverse = reversed;
    let selected = cx.use_state(|| Some(10u64));
    let current_selected = selected.get().unwrap();
    let update_selected = selected;
    let events = cx.use_state(|| 0usize);
    let current_events = events.get().unwrap();
    let update_events = events;
    let status = format!(
        "Selected selector key: {}; events: {current_events}",
        current_selected
            .map(|key| key.to_string())
            .unwrap_or_default()
    );
    let mut items = vec![
        SelectorBarItem::new(10, "Recent"),
        SelectorBarItem::new(20, "Shared").icon(Icon::symbol(IconSymbol::PEOPLE)),
        SelectorBarItem::new(30, "Favorites").icon(Icon::symbol(IconSymbol::FAVORITE)),
    ];
    if current_reversed {
        items.reverse();
    }

    stack_panel([
        Button::new("Reverse selector items")
            .on_click(move || {
                reverse.set(!current_reversed);
            })
            .automation_name("Reverse selector items")
            .build(),
        TextBlock::new(&status)
            .automation_name("SelectorBar selection status")
            .help_text(status)
            .build(),
        SelectorBar::new(items, move |key| {
            update_selected.set(key);
            update_events.set(current_events + 1);
        })
        .selected_key(current_selected)
        .automation_name("Keyed selector bar")
        .help_text("Select one keyed selector item")
        .build(),
    ])
}

fn breadcrumb_bar(cx: &mut RenderCx) -> Element {
    let reversed = cx.use_state(|| false);
    let current_reversed = reversed.get().unwrap();
    let reverse = reversed;
    let clicked = cx.use_state(|| None::<u64>);
    let current_clicked = clicked.get().unwrap();
    let update_clicked = clicked;
    let events = cx.use_state(|| 0usize);
    let current_events = events.get().unwrap();
    let update_events = events;
    let status = format!(
        "Clicked breadcrumb key: {}; events: {current_events}",
        current_clicked
            .map(|key| key.to_string())
            .unwrap_or_default()
    );
    let mut items = vec![(10, "Home"), (20, "Documents"), (30, "Report")];
    if current_reversed {
        items.reverse();
    }

    stack_panel([
        Button::new("Reverse breadcrumb items")
            .on_click(move || {
                reverse.set(!current_reversed);
            })
            .automation_name("Reverse breadcrumb items")
            .build(),
        TextBlock::new(&status)
            .automation_name("BreadcrumbBar click status")
            .help_text(status)
            .build(),
        BreadcrumbBar::new(items)
            .on_item_clicked(move |key| {
                update_clicked.set(Some(key));
                update_events.set(current_events + 1);
            })
            .automation_name("Keyed breadcrumb bar")
            .help_text("Invoke one keyed breadcrumb item")
            .build(),
    ])
}

fn auto_suggest_box(cx: &mut RenderCx) -> Element {
    let text = cx.use_state(String::new);
    let current_text = text.get().unwrap();
    let update_text = text.clone();
    let choose_text = text;
    let chosen = cx.use_state(|| None::<u64>);
    let current_chosen = chosen.get().unwrap();
    let update_chosen = chosen;
    let submitted = cx.use_state(String::new);
    let current_submitted = submitted.get().unwrap();
    let update_submitted = submitted;
    let status = format!(
        "Text: {current_text}; chosen: {}; submitted: {current_submitted}",
        current_chosen
            .map(|key| key.to_string())
            .unwrap_or_default()
    );

    stack_panel([
        TextBlock::new(&status)
            .automation_name("AutoSuggestBox status")
            .help_text(status)
            .build(),
        AutoSuggestBox::new(current_text, move |value| {
            update_text.set(value);
        })
        .items([(10, "Apple"), (20, "Apricot"), (30, "Banana")])
        .placeholder_text("Search fruit")
        .header("Fruit")
        .on_query_submitted(move |value| {
            update_submitted.set(value);
        })
        .on_suggestion_chosen(move |key| {
            update_chosen.set(Some(key));
            choose_text.set(
                match key {
                    10 => "Apple",
                    20 => "Apricot",
                    30 => "Banana",
                    _ => unreachable!(),
                }
                .to_string(),
            );
        })
        .automation_name("Keyed auto suggest box")
        .help_text("Search keyed fruit suggestions")
        .build(),
    ])
}

fn teaching_tip(cx: &mut RenderCx) -> Element {
    let open = cx.use_state(|| true);
    let current_open = open.get().unwrap();
    let open_tip = open.clone();
    let close_tip = open.clone();
    let close_from_tip = open;
    let text_owner = cx.use_state(|| false);
    let current_text_owner = text_owner.get().unwrap();
    let replace_owner = text_owner;
    let present = cx.use_state(|| true);
    let current_present = present.get().unwrap();
    let remove_owner = present;
    let closed = cx.use_state(|| 0usize);
    let current_closed = closed.get().unwrap();
    let update_closed = closed;
    let actions = cx.use_state(|| 0usize);
    let current_actions = actions.get().unwrap();
    let update_actions = actions;
    let status = format!(
        "TeachingTip open: {current_open}; closed: {current_closed}; actions: {current_actions}"
    );

    let owner = if current_text_owner {
        TextBlock::new("Teaching tip text owner")
            .automation_name("Teaching tip owner")
            .build()
    } else {
        Button::new("Teaching tip button owner")
            .on_click(|| {})
            .automation_name("Teaching tip owner")
            .build()
    };
    let owner = if current_present {
        owner.teaching_tip(
            TeachingTip::new("Reactor teaching tip")
                .subtitle("Owner-bound overlay")
                .open(current_open)
                .action_button("Advance")
                .close_button("Close")
                .on_closed(move || {
                    update_closed.set(update_closed.get().unwrap() + 1);
                    close_from_tip.set(false);
                })
                .on_action_button_click(move || {
                    update_actions.set(update_actions.get().unwrap() + 1);
                }),
        )
    } else {
        TextBlock::new("Teaching tip owner removed").build()
    };

    stack_panel([
        TextBlock::new(&status)
            .automation_name("Teaching tip status")
            .help_text(status)
            .build(),
        Button::new("Open teaching tip")
            .on_click(move || {
                open_tip.set(true);
            })
            .automation_name("Open teaching tip")
            .build(),
        Button::new("Close teaching tip programmatically")
            .on_click(move || {
                close_tip.set(false);
            })
            .automation_name("Close teaching tip programmatically")
            .build(),
        Button::new("Replace teaching tip owner")
            .on_click(move || {
                replace_owner.set(!replace_owner.get().unwrap());
            })
            .automation_name("Replace teaching tip owner")
            .build(),
        Button::new("Remove teaching tip owner")
            .on_click(move || {
                remove_owner.set(false);
            })
            .automation_name("Remove teaching tip owner")
            .build(),
        owner,
    ])
}

fn flyout(cx: &mut RenderCx) -> Element {
    let version = cx.use_state(|| 0usize);
    let current_version = version.get().unwrap();
    let update_version = version;
    let present = cx.use_state(|| true);
    let current_present = present.get().unwrap();
    let remove_owner = present;
    let opened = cx.use_state(|| 0usize);
    let current_opened = opened.get().unwrap();
    let update_opened = opened;
    let closed = cx.use_state(|| 0usize);
    let current_closed = closed.get().unwrap();
    let update_closed = closed;
    let status = format!(
        "Flyout version: {current_version}; opened: {current_opened}; closed: {current_closed}"
    );

    let owner = if current_present {
        DropDownButton::new(
            "Open flyout",
            StackPanel::new([
                TextBlock::new(format!("Flyout content {current_version}")).build(),
                Button::new("Update flyout content")
                    .on_click(move || {
                        update_version.set(update_version.get().unwrap() + 1);
                    })
                    .automation_name("Update flyout content")
                    .build(),
            ])
            .build(),
        )
        .on_opened(move || {
            update_opened.set(update_opened.get().unwrap() + 1);
        })
        .on_closed(move || {
            update_closed.set(update_closed.get().unwrap() + 1);
        })
        .automation_name("Open flyout")
        .build()
    } else {
        TextBlock::new("Flyout owner removed").build()
    };

    stack_panel([
        TextBlock::new(&status)
            .automation_name("Flyout status")
            .help_text(status)
            .build(),
        Button::new("Remove flyout owner")
            .on_click(move || {
                remove_owner.set(false);
            })
            .automation_name("Remove flyout owner")
            .build(),
        owner,
    ])
}

fn content_dialog(cx: &mut RenderCx) -> Element {
    let open = cx.use_state(|| false);
    let current_open = open.get().unwrap();
    let show_dialog = open.clone();
    let close_dialog = open;
    let present = cx.use_state(|| true);
    let current_present = present.get().unwrap();
    let remove_dialog = present.clone();
    let restore_dialog = present;
    let version = cx.use_state(|| 0usize);
    let current_version = version.get().unwrap();
    let update_version = version;
    let result = cx.use_state(|| ContentDialogResult::None);
    let current_result = result.get().unwrap();
    let update_result = result;
    let status = format!("Dialog open: {current_open}; result: {current_result:?}");

    let dialog = if current_present {
        ContentDialog::new(
            "Reactor content dialog",
            StackPanel::new([
                TextBlock::new(format!("Dialog content {current_version}")).build(),
                Button::new("Update dialog content")
                    .on_click(move || {
                        update_version.set(update_version.get().unwrap() + 1);
                    })
                    .automation_name("Update dialog content")
                    .build(),
                Button::new("Remove open dialog")
                    .on_click(move || {
                        remove_dialog.set(false);
                    })
                    .automation_name("Remove open dialog")
                    .build(),
            ])
            .build(),
        )
        .primary_button("Primary")
        .secondary_button("Secondary")
        .close_button("Close")
        .open(current_open)
        .on_closed(move |value| {
            update_result.set(value);
            close_dialog.set(false);
        })
        .build()
    } else {
        TextBlock::new("Content dialog removed").build()
    };

    stack_panel([
        TextBlock::new(&status)
            .automation_name("Content dialog status")
            .help_text(status)
            .build(),
        Button::new("Open content dialog")
            .on_click(move || {
                show_dialog.set(true);
            })
            .automation_name("Open content dialog")
            .build(),
        Button::new("Restore content dialog")
            .on_click(move || {
                restore_dialog.set(true);
            })
            .automation_name("Restore content dialog")
            .build(),
        dialog,
    ])
}

fn command_bar(cx: &mut RenderCx) -> Element {
    let open_count = cx.use_state(|| 0usize);
    let current_open_count = open_count.get().unwrap();
    let open = open_count;
    let pinned = cx.use_state(|| false);
    let current_pinned = pinned.get().unwrap();
    let set_pinned = pinned;
    let secondary_count = cx.use_state(|| 0usize);
    let current_secondary_count = secondary_count.get().unwrap();
    let secondary = secondary_count;
    let reversed = cx.use_state(|| false);
    let current_reversed = reversed.get().unwrap();
    let reorder = reversed;
    let present = cx.use_state(|| true);
    let current_present = present.get().unwrap();
    let remove = present.clone();
    let restore = present;

    let open_item = CommandBarItem::button(10, "Open command", move || {
        open.set(current_open_count + 1);
    });
    let pin_item = CommandBarItem::toggle(20, "Pin command", current_pinned, move |value| {
        set_pinned.set(value);
    });
    let separator = CommandBarItem::separator(30);
    let reorder_item = CommandBarItem::button(40, "Reorder commands", move || {
        reorder.set(!current_reversed);
    });
    let primary = if current_reversed {
        vec![reorder_item, pin_item, separator, open_item]
    } else {
        vec![open_item, pin_item, separator, reorder_item]
    };
    let bar = if current_present {
        CommandBar::new(primary)
            .secondary_commands([
                CommandBarItem::button(50, "Secondary command", move || {
                    secondary.set(current_secondary_count + 1);
                }),
                CommandBarItem::button(60, "Remove command bar", move || {
                    remove.set(false);
                }),
            ])
            .default_label_position(CommandBarDefaultLabelPosition::Right)
            .automation_name("Reactor command bar")
            .build()
    } else {
        TextBlock::new("Command bar removed").build()
    };

    stack_panel([
        TextBlock::new(format!(
            "Open: {current_open_count}; pinned: {current_pinned}; secondary: \
             {current_secondary_count}; reversed: {current_reversed}; present: {current_present}"
        ))
        .automation_name("Command bar status")
        .help_text(format!(
            "Open: {current_open_count}; pinned: {current_pinned}; secondary: \
             {current_secondary_count}; reversed: {current_reversed}; present: {current_present}"
        ))
        .build(),
        Button::new("Restore command bar")
            .on_click(move || {
                restore.set(true);
            })
            .automation_name("Restore command bar")
            .build(),
        bar,
    ])
}

fn media(cx: &mut RenderCx) -> Element {
    let bitmap_loaded = cx.use_state(|| false);
    let svg_loaded = cx.use_state(|| false);
    let failure_reported = cx.use_state(|| false);
    let pending = cx.use_state(|| false);
    let bitmap_uri = "ms-appx:///reactor-native-media.png".to_string();
    let svg_uri = "ms-appx:///reactor-native-media.svg".to_string();
    let bitmap_source = ImageSource::bitmap(bitmap_uri.clone());
    let svg_source = ImageSource::svg(svg_uri);

    let bitmap_state = bitmap_loaded.clone();
    let bitmap = Image::new(bitmap_source.clone())
        .on_load(move |result| {
            bitmap_state.set(result.is_ok());
        })
        .width(64.0)
        .height(64.0)
        .automation_name("Bitmap media image")
        .build();
    let svg_state = svg_loaded.clone();
    let svg = Image::new(svg_source.clone())
        .on_load(move |result| {
            svg_state.set(result.is_ok());
        })
        .width(64.0)
        .height(64.0)
        .automation_name("SVG media image")
        .build();
    let failure_state = failure_reported.clone();
    let failed = Image::new(ImageSource::bitmap(
        "ms-appx:///missing-reactor-native-media.png",
    ))
    .on_load(move |result| {
        failure_state.set(result.is_err());
    })
    .width(1.0)
    .height(1.0)
    .build();
    let pending_image = if pending.get().unwrap() {
        Image::new(ImageSource::bitmap(
            "http://192.0.2.1:81/reactor-native-pending.png",
        ))
        .width(1.0)
        .height(1.0)
        .build()
    } else {
        TextBlock::new("Pending image not started").build()
    };
    let start_pending = pending;
    let status = format!(
        "bitmap: {}; svg: {}; failure: {}",
        if bitmap_loaded.get().unwrap() {
            "loaded"
        } else {
            "waiting"
        },
        if svg_loaded.get().unwrap() {
            "loaded"
        } else {
            "waiting"
        },
        if failure_reported.get().unwrap() {
            "reported"
        } else {
            "waiting"
        }
    );
    let commands = CommandBar::new([
        CommandBarItem::button(1, "Symbol icon", || {}).icon(Icon::symbol(IconSymbol::ACCEPT)),
        CommandBarItem::button(2, "Font icon", || {})
            .icon(Icon::font("\u{e8a5}", "Segoe Fluent Icons")),
        CommandBarItem::button(3, "Bitmap icon", || {}).icon(Icon::bitmap(bitmap_uri, false)),
        CommandBarItem::button(4, "Image icon", || {}).icon(Icon::image(svg_source)),
    ])
    .secondary_commands([
        CommandBarItem::button(5, "Secondary symbol icon", || {})
            .icon(Icon::symbol(IconSymbol::MORE)),
        CommandBarItem::button(6, "Secondary image icon", || {}).icon(Icon::image(bitmap_source)),
    ])
    .automation_name("Reactor media command bar")
    .build();

    stack_panel([
        TextBlock::new(status.clone())
            .automation_name("Media status")
            .help_text(status)
            .build(),
        bitmap,
        svg,
        failed,
        Button::new("Start pending image")
            .on_click(move || {
                start_pending.set(true);
            })
            .automation_name("Start pending image")
            .build(),
        pending_image,
        commands,
    ])
}

fn canvas(cx: &mut RenderCx) -> Element {
    let draws = cx.use_state(|| 0usize);
    let current = draws.get().unwrap();
    let record_draw = draws;
    let draw_counter = cx.use_ref(|| 0usize);
    let increment_draw = draw_counter;
    let last_width = cx.use_state(|| 0u32);
    let current_last_width = last_width.get().unwrap();
    let record_width = last_width;
    let devices = cx.use_state(|| 0usize);
    let current_devices = devices.get().unwrap();
    let record_device = devices;
    let device_counter = cx.use_ref(|| 0usize);
    let increment_device = device_counter;
    let fail_next = cx.use_ref(|| false);
    let fail_draw = fail_next.clone();
    let request_device_loss = fail_next;
    let width = cx.use_state(|| 240.0);
    let current_width = width.get().unwrap();
    let resize = width.clone();
    let zero = width.clone();
    let restore_size = width;
    let visible = cx.use_state(|| true);
    let current_visible = visible.get().unwrap();
    let remove = visible.clone();
    let restore_node = visible;
    let invalidator = cx.use_canvas_invalidator();
    let request_draw = invalidator.clone();
    let surface = if current_visible {
        swap_chain_canvas_invalidated(&invalidator, move |frame| {
            if fail_draw.get().unwrap() {
                fail_draw.set(false);
                return Err(windows_canvas::device_lost_error());
            }
            frame.clear(ColorF::DARK_SLATE_BLUE);
            if frame.device_changed() {
                let count = increment_device
                    .with_mut(|value| {
                        *value += 1;
                        *value
                    })
                    .unwrap();
                record_device.set(count);
            }
            record_width.set(frame.width.round() as u32);
            let count = increment_draw
                .with_mut(|value| {
                    *value += 1;
                    *value
                })
                .unwrap();
            record_draw.set(count);
            Ok(())
        })
        .width(current_width)
        .height(120.0)
        .automation_name("Demand canvas")
        .build()
    } else {
        TextBlock::new("Canvas removed").build()
    };
    let status =
        format!("draws: {current}; width: {current_last_width}; devices: {current_devices}");

    stack_panel([
        TextBlock::new(&status)
            .automation_name("Canvas status")
            .help_text(status)
            .build(),
        surface,
        Button::new("Invalidate canvas")
            .on_click(move || request_draw.invalidate())
            .build(),
        Button::new("Lose canvas device")
            .on_click(move || {
                request_device_loss.set(true);
                invalidator.invalidate();
            })
            .build(),
        Button::new("Resize canvas")
            .on_click(move || {
                resize.set(320.0);
            })
            .build(),
        Button::new("Zero canvas")
            .on_click(move || {
                zero.set(0.0);
            })
            .build(),
        Button::new("Restore canvas size")
            .on_click(move || {
                restore_size.set(240.0);
            })
            .build(),
        Button::new("Remove canvas")
            .on_click(move || {
                remove.set(false);
            })
            .build(),
        Button::new("Restore canvas node")
            .on_click(move || {
                restore_node.set(true);
            })
            .build(),
    ])
}

fn canvas_image(cx: &mut RenderCx) -> Element {
    let draws = cx.use_state(|| 0usize);
    let current_draws = draws.get().unwrap();
    let record_draws = draws;
    let devices = cx.use_state(|| 0usize);
    let current_devices = devices.get().unwrap();
    let record_devices = devices;
    let surfaces = cx.use_state(|| 0usize);
    let current_surfaces = surfaces.get().unwrap();
    let record_surfaces = surfaces;
    let last_width = cx.use_state(|| 0u32);
    let current_width_drawn = last_width.get().unwrap();
    let record_width = last_width;
    let fail_next = cx.use_ref(|| false);
    let fail_draw = fail_next.clone();
    let request_device_loss = fail_next;
    let width = cx.use_state(|| 240.0);
    let current_width = width.get().unwrap();
    let resize = width.clone();
    let zero = width.clone();
    let restore_size = width;
    let visible = cx.use_state(|| true);
    let current_visible = visible.get().unwrap();
    let remove = visible.clone();
    let restore_node = visible;
    let invalidator = cx.use_canvas_invalidator();
    let request_draw = invalidator.clone();
    let surface = if current_visible {
        canvas_image_invalidated(&invalidator, move |frame| {
            if fail_draw.get().unwrap() {
                fail_draw.set(false);
                return Err(windows_canvas::device_lost_error());
            }
            frame.clear(ColorF::DARK_SLATE_BLUE);
            if frame.device_changed() {
                record_devices.set(current_devices.wrapping_add(1));
            }
            if frame.surface_changed() {
                record_surfaces.set(current_surfaces.wrapping_add(1));
            }
            record_width.set(frame.width.round() as u32);
            record_draws.set(current_draws.wrapping_add(1));
            Ok(())
        })
        .width(current_width)
        .height(120.0)
        .automation_name("Canvas image")
        .build()
    } else {
        TextBlock::new("Canvas image removed").build()
    };
    let status = format!(
        "draws: {current_draws}; width: {current_width_drawn}; devices: {current_devices}; surfaces: {current_surfaces}"
    );

    stack_panel([
        TextBlock::new(&status)
            .automation_name("Canvas image status")
            .help_text(status)
            .build(),
        surface,
        Button::new("Invalidate canvas image")
            .on_click(move || request_draw.invalidate())
            .build(),
        Button::new("Lose canvas image device")
            .on_click(move || {
                request_device_loss.set(true);
                invalidator.invalidate();
            })
            .build(),
        Button::new("Resize canvas image")
            .on_click(move || {
                resize.set(320.0);
            })
            .build(),
        Button::new("Zero canvas image")
            .on_click(move || {
                zero.set(0.0);
            })
            .build(),
        Button::new("Restore canvas image size")
            .on_click(move || {
                restore_size.set(240.0);
            })
            .build(),
        Button::new("Remove canvas image")
            .on_click(move || {
                remove.set(false);
            })
            .build(),
        Button::new("Restore canvas image node")
            .on_click(move || {
                restore_node.set(true);
            })
            .build(),
    ])
}

fn animated_canvas_fixture(cx: &mut RenderCx) -> Element {
    let sample = cx.use_state(|| "ready".to_string());
    let current_sample = sample.get().unwrap();
    let publish_sample = sample;
    let draws = std::rc::Rc::new(std::cell::Cell::new(0usize));
    let record_draws = std::rc::Rc::clone(&draws);
    let visible = cx.use_state(|| true);
    let current_visible = visible.get().unwrap();
    let remove = visible;
    let surface = if current_visible {
        animated_canvas(move |frame| {
            frame.clear(ColorF::DARK_SLATE_BLUE);
            record_draws.set(record_draws.get().wrapping_add(1));
            Ok(())
        })
        .width(240.0)
        .height(120.0)
        .automation_name("Animated canvas")
        .build()
    } else {
        TextBlock::new("Animated canvas removed").build()
    };
    stack_panel([
        TextBlock::new(&current_sample)
            .automation_name("Animated canvas status")
            .help_text(current_sample)
            .build(),
        surface,
        Button::new("Publish animated canvas metrics")
            .on_click(move || {
                publish_sample.set(format!("draws: {}", draws.get()));
            })
            .build(),
        Button::new("Remove animated canvas")
            .on_click(move || {
                remove.set(false);
            })
            .build(),
    ])
}

fn main() -> windows_core::Result<()> {
    let failure_mode = std::env::args().any(|argument| argument == "--failure");
    let grid_mode = std::env::args().any(|argument| argument == "--grid");
    let list_box_mode = std::env::args().any(|argument| argument == "--list-box");
    let combo_box_mode = std::env::args().any(|argument| argument == "--combo-box");
    let radio_buttons_mode = std::env::args().any(|argument| argument == "--radio-buttons");
    let selector_bar_mode = std::env::args().any(|argument| argument == "--selector-bar");
    let breadcrumb_bar_mode = std::env::args().any(|argument| argument == "--breadcrumb-bar");
    let auto_suggest_box_mode = std::env::args().any(|argument| argument == "--auto-suggest-box");
    let teaching_tip_mode = std::env::args().any(|argument| argument == "--teaching-tip");
    let flyout_mode = std::env::args().any(|argument| argument == "--flyout");
    let content_dialog_mode = std::env::args().any(|argument| argument == "--content-dialog");
    let command_bar_mode = std::env::args().any(|argument| argument == "--command-bar");
    let media_mode = std::env::args().any(|argument| argument == "--media");
    let canvas_mode = std::env::args().any(|argument| argument == "--canvas");
    let canvas_image_mode = std::env::args().any(|argument| argument == "--canvas-image");
    let animated_canvas_mode = std::env::args().any(|argument| argument == "--animated-canvas");
    let multi_window_mode = std::env::args().any(|argument| argument == "--multi-window");
    if multi_window_mode {
        return run_reactor_winui_app(component(multi_window));
    }
    let root = if failure_mode {
        component(failure)
    } else if grid_mode {
        component(grid)
    } else if list_box_mode {
        component(list_box)
    } else if combo_box_mode {
        component(combo_box)
    } else if radio_buttons_mode {
        component(radio_buttons)
    } else if selector_bar_mode {
        component(selector_bar)
    } else if breadcrumb_bar_mode {
        component(breadcrumb_bar)
    } else if auto_suggest_box_mode {
        component(auto_suggest_box)
    } else if teaching_tip_mode {
        component(teaching_tip)
    } else if flyout_mode {
        component(flyout)
    } else if content_dialog_mode {
        component(content_dialog)
    } else if command_bar_mode {
        component(command_bar)
    } else if media_mode {
        component(media)
    } else if animated_canvas_mode {
        component(animated_canvas_fixture)
    } else if canvas_image_mode {
        component(canvas_image)
    } else if canvas_mode {
        component(canvas)
    } else {
        component(interactive)
    };
    run_reactor_winui("windows-reactor native self-test", root)
}

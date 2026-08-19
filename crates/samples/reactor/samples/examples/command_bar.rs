#![windows_subsystem = "windows"]

use windows_reactor::{
    CommandBar, CommandBarDefaultLabelPosition, CommandBarItem, Element, RenderCx, TextBlock,
    vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let status = cx.use_state(|| "(none)".to_string());
    let current = status.value();
    let pinned = cx.use_state(|| false);
    let current_pinned = pinned.value();
    let add = status.clone();
    let edit = status.clone();
    let pin_status = status.clone();
    let save = status.clone();
    let delete = status.clone();
    let select_all = status.clone();
    let share = status;

    let bar = CommandBar::new([
        CommandBarItem::button(10, "Add", move || {
            add.set("Add".to_string());
        }),
        CommandBarItem::button(20, "Edit", move || {
            edit.set("Edit".to_string());
        }),
        CommandBarItem::toggle(30, "Pin", current_pinned, move |value| {
            pinned.set(value);
            pin_status.set("Pin".to_string());
        }),
        CommandBarItem::separator(40),
        CommandBarItem::button(50, "Save", move || {
            save.set("Save".to_string());
        }),
        CommandBarItem::button(60, "Delete", move || {
            delete.set("Delete".to_string());
        }),
    ])
    .secondary_commands([
        CommandBarItem::button(70, "Select All", move || {
            select_all.set("Select All".to_string());
        }),
        CommandBarItem::button(80, "Share", move || {
            share.set("Share".to_string());
        }),
    ])
    .default_label_position(CommandBarDefaultLabelPosition::Right)
    .automation_name("Sample command bar")
    .build();

    vstack(
        12.0,
        [
            bar,
            TextBlock::new(format!("Last clicked: {current}; pinned: {current_pinned}"))
                .automation_id("command-status")
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("CommandBar", app)
}

#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, CommandBarFlyout, CommandBarItem, Element, Icon, IconSymbol, RenderCx, TextBlock,
    vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let last_action = cx.use_state(|| String::from("(none)"));
    let current = last_action.value();
    let cut = last_action.clone();
    let copy = last_action.clone();
    let paste = last_action.clone();
    let select_all = last_action.clone();
    let print = last_action;

    vstack(
        8.0,
        [
            Button::new("Show Commands")
                .command_bar_flyout(
                    CommandBarFlyout::new([
                        CommandBarItem::button(1, "Cut", move || {
                            cut.set(String::from("Cut"));
                        })
                        .icon(Icon::symbol(IconSymbol::EDIT)),
                        CommandBarItem::button(2, "Copy", move || {
                            copy.set(String::from("Copy"));
                        }),
                        CommandBarItem::button(3, "Paste", move || {
                            paste.set(String::from("Paste"));
                        }),
                    ])
                    .secondary_commands([
                        CommandBarItem::button(4, "Select All", move || {
                            select_all.set(String::from("Select All"));
                        }),
                        CommandBarItem::button(5, "Print", move || {
                            print.set(String::from("Print"));
                        }),
                    ]),
                )
                .build(),
            TextBlock::new(format!("Last action: {current}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("CommandBarFlyout", app)
}

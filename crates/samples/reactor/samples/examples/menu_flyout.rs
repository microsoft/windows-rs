#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, MenuFlyout, MenuItem, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let last_action = cx.use_state(|| String::from("(none)"));
    let current = last_action.value();

    let cut = last_action.clone();
    let copy = last_action.clone();
    let paste = last_action.clone();
    let small = last_action.clone();
    let medium = last_action.clone();
    let large = last_action;

    vstack(
        8.0,
        [
            Button::new("Open Menu")
                .menu_flyout(MenuFlyout::new(vec![
                    MenuItem::new(1, "Cut", move || {
                        cut.set(String::from("Cut"));
                    }),
                    MenuItem::new(2, "Copy", move || {
                        copy.set(String::from("Copy"));
                    }),
                    MenuItem::new(3, "Paste", move || {
                        paste.set(String::from("Paste"));
                    }),
                    MenuItem::separator(4),
                    MenuItem::submenu(
                        5,
                        "Font Size",
                        vec![
                            MenuItem::new(6, "Small", move || {
                                small.set(String::from("Small"));
                            }),
                            MenuItem::new(7, "Medium", move || {
                                medium.set(String::from("Medium"));
                            }),
                            MenuItem::new(8, "Large", move || {
                                large.set(String::from("Large"));
                            }),
                        ],
                    ),
                ]))
                .build(),
            TextBlock::new(format!("Last action: {current}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("MenuFlyout", app)
}

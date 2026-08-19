#![windows_subsystem = "windows"]

use windows_reactor::{
    DropDownButton, Element, MenuBar, MenuBarItem, MenuFlyout, MenuItem, RenderCx, TextBlock,
    vstack,
};

fn action(state: &windows_reactor::State<String>, value: &'static str) -> MenuItem {
    let state = state.clone();
    MenuItem::new(menu_key(value), value, move || {
        state.set(String::from(value));
    })
}

const fn menu_key(value: &str) -> u64 {
    let bytes = value.as_bytes();
    let mut hash = 1469598103934665603u64;
    let mut index = 0;
    while index < bytes.len() {
        hash ^= bytes[index] as u64;
        hash = hash.wrapping_mul(1099511628211);
        index += 1;
    }
    hash
}

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let last_click = cx.use_state(|| String::from("(none)"));
    let current = last_click.value();

    vstack(
        12.0,
        [
            MenuBar::new(vec![
                MenuBarItem::new(
                    1,
                    "File",
                    vec![
                        action(&last_click, "New"),
                        action(&last_click, "Open"),
                        MenuItem::separator(2),
                        MenuItem::submenu(
                            3,
                            "Recent",
                            vec![
                                action(&last_click, "doc1.txt"),
                                action(&last_click, "doc2.txt"),
                            ],
                        ),
                        MenuItem::separator(4),
                        action(&last_click, "Exit"),
                    ],
                ),
                MenuBarItem::new(
                    5,
                    "Edit",
                    vec![
                        action(&last_click, "Cut"),
                        action(&last_click, "Copy"),
                        action(&last_click, "Paste"),
                    ],
                ),
                MenuBarItem::new(6, "Help", vec![action(&last_click, "About")]),
            ])
            .build(),
            DropDownButton::with_menu(
                "Actions",
                MenuFlyout::new(vec![
                    action(&last_click, "Action A"),
                    action(&last_click, "Action B"),
                    MenuItem::separator(7),
                    MenuItem::submenu(
                        8,
                        "More",
                        vec![
                            action(&last_click, "Action C"),
                            action(&last_click, "Action D"),
                        ],
                    ),
                ]),
            )
            .build(),
            TextBlock::new(format!("Last clicked: {current}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("MenuBar", app)
}

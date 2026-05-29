use crate::controls::*;
use windows_reactor::*;

pub fn command_bar_flyout_page(_: &(), cx: &mut RenderCx) -> Element {
    let (last_action, set_last_action) = cx.use_state(String::from("(none)"));

    page_content(
        "CommandBarFlyout",
        "A flyout that provides quick access to common commands.",
        vec![
            sample_card(
                "Basic CommandBarFlyout",
                vstack((
                    button("Show Commands")
                        .command_bar_flyout(vec![
                            app_bar_button_icon("Cut", SymbolGlyph::Cut),
                            app_bar_button_icon("Copy", SymbolGlyph::Copy),
                            app_bar_button_icon("Paste", SymbolGlyph::Paste),
                        ])
                        .on_command_bar_flyout_click({
                            let set_last_action = set_last_action.clone();
                            move |label| set_last_action.call(label)
                        }),
                    text_block(format!("Last action: {last_action}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"button("Show Commands")
    .command_bar_flyout(vec![
        app_bar_button_icon("Cut", SymbolGlyph::Cut),
        app_bar_button_icon("Copy", SymbolGlyph::Copy),
        app_bar_button_icon("Paste", SymbolGlyph::Paste),
    ])
    .on_command_bar_flyout_click(|label| set_action.call(label))"#,
            ),
            sample_card(
                "CommandBarFlyout with Secondary Commands",
                button("More Options")
                    .command_bar_flyout(vec![app_bar_button_icon("Share", SymbolGlyph::Send)])
                    .command_bar_flyout_secondary(vec![
                        app_bar_button("Select All"),
                        app_bar_separator(),
                        app_bar_button("Print"),
                    ])
                    .on_command_bar_flyout_click({
                        let set_last_action = set_last_action;
                        move |label| set_last_action.call(label)
                    }),
                r#"button("More Options")
    .command_bar_flyout(vec![
        app_bar_button_icon("Share", SymbolGlyph::Send),
    ])
    .command_bar_flyout_secondary(vec![
        app_bar_button("Select All"),
        app_bar_separator(),
        app_bar_button("Print"),
    ])"#,
            ),
        ],
    )
    .into()
}

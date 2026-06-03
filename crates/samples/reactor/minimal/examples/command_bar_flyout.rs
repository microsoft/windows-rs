//! Sample for `CommandBarFlyout` on a `Button`.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (last_action, set_action) = cx.use_state("(none)".to_string());

    let on_cmd = move |label: String| set_action.call(label);

    vstack((
        button("Show Commands")
            .command_bar_flyout(vec![
                app_bar_button_icon("Cut", SymbolGlyph::Cut),
                app_bar_button_icon("Copy", SymbolGlyph::Copy),
                app_bar_button_icon("Paste", SymbolGlyph::Paste),
            ])
            .command_bar_flyout_secondary(vec![
                app_bar_button("Select All"),
                app_bar_button("Print"),
            ])
            .on_command_bar_flyout_click(on_cmd),
        text_block(format!("Last action: {last_action}")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("CommandBarFlyout", app)
}

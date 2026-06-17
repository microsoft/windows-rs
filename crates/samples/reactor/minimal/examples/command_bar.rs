//! Sample for the `CommandBar` widget.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (last_click, set_last_click) = cx.use_state(String::from("(none)"));

    vstack((
        command_bar(vec![
            app_bar_button_icon("Add", Symbol::Add),
            app_bar_button_icon("Edit", Symbol::Edit),
            app_bar_separator(),
            app_bar_button_icon("Save", Symbol::Save),
            app_bar_button_icon("Delete", Symbol::Delete),
        ])
        .secondary_commands(vec![app_bar_button("Select All"), app_bar_button("Share")])
        .on_click(move |label| set_last_click.call(label)),
        text_block(format!("Last clicked: {last_click}")),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("CommandBar", app)
}

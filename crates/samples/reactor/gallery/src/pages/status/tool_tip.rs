use crate::controls::*;
use windows_reactor::*;

pub fn tool_tip_page(_: &(), cx: &mut RenderCx) -> Element {
    let (hover_count, set_hover_count) = cx.use_state(0_i32);

    let rich_tooltip = Tooltip::rich(
        vstack((
            text_block("Interactive tooltip").bold(),
            text_block("Hover to inspect the tooltip, then click to log another visit."),
            text_block(format!("Hover count: {hover_count}")).opacity(0.6),
        ))
        .spacing(4.0),
    );

    page_content(
        "ToolTip",
        "A popup with helpful text on hover.",
        vec![
            sample_card(
                "Interactive ToolTip",
                vstack((
                    button("Hover target").tooltip_with(rich_tooltip).on_click({
                        let set_hover_count = set_hover_count;
                        move || set_hover_count.call(hover_count + 1)
                    }),
                    text_block(format!("Hover count: {hover_count}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"button("Hover target")
    .tooltip_with(Tooltip::rich(content))
    .on_click(...)"#,
            ),
            sample_card(
                "Button with ToolTip",
                button("Hover me").tooltip("This is a helpful tooltip"),
                r#"button("Hover me").tooltip("This is a helpful tooltip")"#,
            ),
            sample_card(
                "Multiple Controls with Tooltips",
                hstack((
                    button("Save").icon(Symbol::Save).tooltip("Save the file"),
                    button("Copy")
                        .icon(Symbol::Copy)
                        .tooltip("Copy to clipboard"),
                    button("Delete")
                        .icon(Symbol::Delete)
                        .tooltip("Delete selected item"),
                ))
                .spacing(8.0),
                r#"button("Save").icon(Symbol::Save).tooltip("Save the file")"#,
            ),
        ],
    )
}

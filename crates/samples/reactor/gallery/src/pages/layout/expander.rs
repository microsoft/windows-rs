use crate::controls::*;
use windows_reactor::*;

pub fn expander_page(_: &(), cx: &mut RenderCx) -> Element {
    let (is_expanded, set_expanded) = cx.use_state(true);

    page_content(
        "Expander",
        "Expands and collapses to show or hide content.",
        vec![
            sample_card(
                "Controlled Expander",
                vstack((
                    Expander::new(
                        vstack((
                            text_block("This content can be shown or hidden."),
                            text_block("It supports any child element.").opacity(0.6),
                        ))
                        .spacing(4.0),
                    )
                    .header("Click to expand/collapse")
                    .expanded(is_expanded)
                    .on_expanding(move |v| set_expanded.call(v)),
                    text_block(format!(
                        "State: {}",
                        if is_expanded { "expanded" } else { "collapsed" }
                    ))
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"Expander::new(content).header("...").expanded(state).on_expanding(handler)"#,
            ),
            sample_card(
                "Collapsed by Default",
                Expander::new(text_block("Hidden by default — click header to reveal."))
                    .header("More info")
                    .expanded(false),
                r#"Expander::new(content).header("More info").expanded(false)"#,
            ),
        ],
    )
}

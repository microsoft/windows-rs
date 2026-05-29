use crate::controls::*;
use windows_reactor::*;

pub fn split_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (open, set_open) = cx.use_state(true);

    page_content(
        "SplitView",
        "A collapsible pane and content area.",
        vec![sample_card(
            "Basic SplitView",
            vstack((
                ToggleSwitch::new(open)
                    .header("Pane open")
                    .on_changed(move |v: bool| set_open.call(v)),
                split_view(text_block("Main content area"))
                    .pane(text_block("Pane content"))
                    .is_pane_open(open),
            ))
            .spacing(8.0),
            r#"split_view(content).pane(pane).is_pane_open(open)"#,
        )],
    )
    .into()
}

use crate::controls::*;
use windows_reactor::*;

pub fn flip_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);
    let items = vec!["Welcome", "Features", "Getting Started", "Resources"];

    page_content(
        "FlipView",
        "Presents one item at a time with flipping navigation.",
        vec![sample_card(
            "Interactive FlipView",
            vstack((
                flip_view(items.clone(), |item, idx| {
                    border(
                        vstack((
                            text_block(*item).font_size(24.0).bold(),
                            text_block(format!("Slide {} of 4", idx + 1)).opacity(0.6),
                        ))
                        .spacing(8.0),
                    )
                    .padding(Thickness::uniform(24.0))
                    .corner_radius(8.0)
                })
                .on_selection_changed(set_selected)
                .height(200.0),
                text_block(format!("Current slide: {}", selected + 1)).opacity(0.6),
            ))
            .spacing(8.0),
            r#"flip_view(items, |item, idx| ...).on_selection_changed(set_selected)"#,
        )],
    )
}

use crate::controls::*;
use windows_reactor::*;

pub fn scroll_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(30_i32);

    let items: Vec<Element> = (1..=count)
        .map(|i| text_block(format!("Scrollable item {i}")).into())
        .collect();

    page_content(
        "ScrollView",
        "A scrollable container for overflowing content.",
        vec![sample_card(
            "Dynamic ScrollView",
            vstack((
                hstack((
                    button("More items").on_click({
                        let set = set_count.clone();
                        move || set.call(count + 10)
                    }),
                    button("Fewer items").on_click({
                        let set = set_count;
                        move || set.call((count - 10).max(5))
                    }),
                    text_block(format!("{count} items")).opacity(0.6),
                ))
                .spacing(8.0),
                scroll_view(vstack(items).spacing(4.0)).height(200.0),
            ))
            .spacing(12.0),
            r#"scroll_view(vstack(items).spacing(4.0)).height(200.0)"#,
        )],
    )
}

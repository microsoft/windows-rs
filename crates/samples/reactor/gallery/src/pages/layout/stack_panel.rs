use crate::controls::*;
use windows_reactor::*;

pub fn stack_panel_page(_: &(), cx: &mut RenderCx) -> Element {
    let (item_count, set_count) = cx.use_state(3_i32);

    let items: Vec<Element> = (1..=item_count)
        .map(|index| text_block(format!("Item {index}")).into())
        .collect();

    page_content(
        "StackPanel",
        "Arranges children in a single horizontal or vertical line.",
        vec![
            sample_card(
                "Dynamic Items",
                vstack((
                    hstack((
                        button("Add").icon(Symbol::Add).on_click({
                            let set_count = set_count.clone();
                            move || set_count.call(item_count + 1)
                        }),
                        button("Remove")
                            .icon(Symbol::Delete)
                            .enabled(item_count > 0)
                            .on_click({
                                let set_count = set_count;
                                move || set_count.call(item_count.saturating_sub(1))
                            }),
                        text_block(format!("{item_count} items")).opacity(0.6),
                    ))
                    .spacing(8.0),
                    vstack(items).spacing(4.0),
                ))
                .spacing(12.0),
                r#"vstack(items).spacing(4.0) // items generated from state"#,
            ),
            sample_card(
                "Vertical Stack",
                vstack((
                    text_block("Item 1"),
                    text_block("Item 2"),
                    text_block("Item 3"),
                ))
                .spacing(8.0),
                r#"vstack((text_block("1"), text_block("2"), text_block("3"))).spacing(8.0)"#,
            ),
            sample_card(
                "Horizontal Stack",
                hstack((button("A"), button("B"), button("C"))).spacing(8.0),
                r#"hstack((button("A"), button("B"), button("C"))).spacing(8.0)"#,
            ),
        ],
    )
}

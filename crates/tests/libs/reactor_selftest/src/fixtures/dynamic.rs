use windows_reactor::Element;
use windows_reactor::{ElementExt, button, text_block};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;
use windows_reactor::{hstack, vstack};

pub fn list_grow_shrink(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, set) = cx.use_state(3i32);
            let inc = {
                let set = set.clone();
                move || set.call(count + 1)
            };
            let dec = {
                let set = set;
                move || set.call((count - 1).max(0))
            };
            let items: Vec<Element> = (0..count)
                .map(|i| {
                    text_block(format!("Item #{i}"))
                        .with_key(format!("item-{i}"))
                        .into()
                })
                .collect();
            vstack((
                hstack((
                    text_block(format!("Items: {count}")),
                    button("Add").on_click(inc),
                    button("Remove").on_click(dec),
                ))
                .spacing(8.0),
                vstack(items).spacing(2.0),
            ))
            .spacing(8.0)
            .into()
        }));
        h.render().await;

        h.check(
            "DynamicList_GrowShrink_InitialItems",
            h.find_text("Items: 3").is_some()
                && h.find_text("Item #0").is_some()
                && h.find_text("Item #2").is_some(),
        );

        let _ = h.click_button("Add");
        h.render().await;
        let _ = h.click_button("Add");
        h.render().await;

        h.check(
            "DynamicList_GrowShrink_AfterAdd",
            h.find_text("Items: 5").is_some() && h.find_text("Item #4").is_some(),
        );

        let _ = h.click_button("Remove");
        h.render().await;
        let _ = h.click_button("Remove");
        h.render().await;
        let _ = h.click_button("Remove");
        h.render().await;

        h.check(
            "DynamicList_GrowShrink_AfterRemove",
            h.find_text("Items: 2").is_some() && h.find_text("Item #2").is_none(),
        );
    })
}

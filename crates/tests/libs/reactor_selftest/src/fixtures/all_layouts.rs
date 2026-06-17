use windows_reactor::{Element, GridLength};
use windows_reactor::{button, grid, text_block};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;
use windows_reactor::{hstack, vstack};

pub fn all_layouts_dynamic_child_count(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, set) = cx.use_state(2i32);
            let inc = {
                let set = set.clone();
                move || set.call(count + 1)
            };
            let dec = {
                let set = set;
                move || set.call((count - 1).max(0))
            };
            let stack_kids: Vec<Element> = (0..count)
                .map(|i| text_block(format!("S{i}")).into())
                .collect();
            let grid_kids: Vec<Element> = (0..count)
                .map(|i| {
                    let e: Element = text_block(format!("G{i}")).into();
                    e.grid_row(0).grid_column(i)
                })
                .collect();
            let cols: Vec<GridLength> = (0..count.max(1)).map(|_| GridLength::Star(1.0)).collect();
            let g: Element = grid(grid_kids)
                .rows([GridLength::Star(1.0)])
                .columns(cols)
                .into();
            vstack((
                hstack((button("Add").on_click(inc), button("Remove").on_click(dec))).spacing(8.0),
                vstack(stack_kids).spacing(2.0),
                g,
            ))
            .spacing(8.0)
            .into()
        }));
        h.render().await;

        h.check(
            "Reconciler_AllLayouts_Initial",
            h.find_text("S0").is_some()
                && h.find_text("S1").is_some()
                && h.find_text("G0").is_some()
                && h.find_text("G1").is_some(),
        );

        let _ = h.click_button("Add");
        h.render().await;
        let _ = h.click_button("Add");
        h.render().await;
        h.check(
            "Reconciler_AllLayouts_AfterGrow",
            h.find_text("S3").is_some() && h.find_text("G3").is_some(),
        );

        let _ = h.click_button("Remove");
        h.render().await;
        let _ = h.click_button("Remove");
        h.render().await;
        let _ = h.click_button("Remove");
        h.render().await;
        h.check(
            "Reconciler_AllLayouts_AfterShrink",
            h.find_text("S0").is_some()
                && h.find_text("S1").is_none()
                && h.find_text("G0").is_some()
                && h.find_text("G1").is_none(),
        );

        let _ = h.click_button("Remove");
        h.render().await;
        h.check(
            "Reconciler_AllLayouts_Empty",
            h.find_text("S0").is_none() && h.find_text("G0").is_none(),
        );

        let _ = h.click_button("Add");
        h.render().await;
        let _ = h.click_button("Add");
        h.render().await;
        h.check(
            "Reconciler_AllLayouts_GrowFromEmpty",
            h.find_text("S0").is_some()
                && h.find_text("S1").is_some()
                && h.find_text("G0").is_some()
                && h.find_text("G1").is_some(),
        );
    })
}

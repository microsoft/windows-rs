#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Button, Element, RenderCx, TextBlock, Thickness, component, hstack, memo_component,
    vstack,
};

fn counter() -> Element {
    component(|cx| {
        let count = cx.use_state(|| 0_u32);
        let current = count.value();
        hstack(
            12.0,
            [
                TextBlock::new(format!("Child count: {current}"))
                    .automation_id("memo-child-count")
                    .build(),
                Button::new("Increment child")
                    .on_click(move || {
                        count.update(|value| *value += 1);
                    })
                    .build(),
            ],
        )
    })
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let parent = cx.use_state(|| 0_u32);
    let parent_count = parent.value();

    vstack(
        12.0,
        [
            TextBlock::new(format!("Parent renders: {parent_count}")).build(),
            Button::new("Rerender parent")
                .on_click(move || {
                    parent.update(|value| *value += 1);
                })
                .build(),
            memo_component((), |_| {
                Border::new(counter())
                    .padding(Thickness::uniform(12.0))
                    .build()
            }),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("MemoWidgetDescendant", app)
}

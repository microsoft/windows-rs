#![windows_subsystem = "windows"]

use windows_reactor::{Element, Pivot, PivotItem, RenderCx, TextBlock, Thickness, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| 0i32);
    let current = selected.value();
    let update = selected;

    vstack(
        8.0,
        [
            Pivot::new(
                [
                    PivotItem::new(
                        1,
                        "First",
                        TextBlock::new("Pivot - first tab")
                            .padding(Thickness::uniform(12.0))
                            .build(),
                    ),
                    PivotItem::new(
                        2,
                        "Second",
                        TextBlock::new("Pivot - second tab")
                            .padding(Thickness::uniform(12.0))
                            .build(),
                    ),
                    PivotItem::new(
                        3,
                        "Third",
                        TextBlock::new("Pivot - third tab")
                            .padding(Thickness::uniform(12.0))
                            .build(),
                    ),
                ],
                move |index| {
                    update.set(index);
                },
            )
            .title("Demo")
            .selected_index(current)
            .build(),
            TextBlock::new(format!("selected_index = {current}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Pivot", app)
}

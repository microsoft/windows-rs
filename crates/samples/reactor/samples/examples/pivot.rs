//! Sample for the `Pivot` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0i32);

    vstack((
        Pivot::new([
            PivotItem::new(
                "First",
                text_block("Pivot — first tab").padding(Thickness::uniform(12.0)),
            ),
            PivotItem::new(
                "Second",
                text_block("Pivot — second tab").padding(Thickness::uniform(12.0)),
            ),
            PivotItem::new(
                "Third",
                text_block("Pivot — third tab").padding(Thickness::uniform(12.0)),
            ),
        ])
        .title("Demo")
        .selected_index(selected)
        .on_selection_changed(set_selected),
        text_block(format!("selected_index = {selected}")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Pivot", app)
}

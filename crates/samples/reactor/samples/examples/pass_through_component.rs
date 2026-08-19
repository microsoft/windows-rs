#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, RenderCx, StackPanel, TextBlock, Thickness, component, memo_component, vstack,
};

fn counter() -> Element {
    component(|cx| {
        let count = cx.use_state(|| 0_u32);
        let current = count.value();
        vstack(
            8.0,
            [
                TextBlock::new(format!("count = {current}"))
                    .font_size(20.0)
                    .build(),
                Button::new("Increment")
                    .on_click(move || {
                        count.update(|value| *value += 1);
                    })
                    .build(),
            ],
        )
    })
}

fn app(_cx: &mut RenderCx<'_>) -> Element {
    StackPanel::new([
        TextBlock::new("The memoized wrapper returns the stateful component directly.").build(),
        TextBlock::new("Clicking Increment must continue to update the count.").build(),
        memo_component((), |_| counter()),
    ])
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("PassThroughComponent", app)
}

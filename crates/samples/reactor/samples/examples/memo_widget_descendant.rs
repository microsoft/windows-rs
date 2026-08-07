#![windows_subsystem = "windows"]

use windows_reactor::*;

fn counter(_props: &(), cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u32);

    hstack((
        text_block(format!("count = {count}")).font_size(20.0),
        button("Increment").on_click(move || set_count.call(count + 1)),
    ))
    .spacing(12.0)
    .into()
}

fn memoized_frame(_props: &(), _cx: &mut RenderCx) -> Element {
    border(component(counter, ()))
        .padding(Thickness::uniform(12.0))
        .into()
}

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("A dirty child must update through a memoized component with a widget root."),
        text_block("Click Increment. The count must advance on every click."),
        memo(memoized_frame, ()),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("MemoWidgetDescendant", app)
}

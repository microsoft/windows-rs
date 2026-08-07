#![windows_subsystem = "windows"]

use windows_reactor::*;

fn counter(_props: &(), cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u32);

    vstack((
        text_block(format!("count = {count}")).font_size(20.0),
        button("Increment").on_click(move || set_count.call(count + 1)),
    ))
    .spacing(8.0)
    .into()
}

fn pass_through(_props: &(), _cx: &mut RenderCx) -> Element {
    component(counter, ())
}

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("The memoized wrapper returns the stateful component directly."),
        text_block("Clicking Increment must continue to update the count."),
        memo(pass_through, ()),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("PassThroughComponent", app)
}

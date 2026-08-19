use windows_reactor::{Element, RenderCx};

pub fn run(
    title: impl Into<String>,
    render: for<'a> fn(&mut RenderCx<'a>) -> Element,
) -> windows_core::Result<()> {
    reactor_samples::run(title, render)
}

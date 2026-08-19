use windows_canvas::*;
use windows_reactor::{
    Application, CanvasDrawContext, Element, RenderCx, Window, WindowBackdrop, animated_canvas,
    component, run_reactor_winui_app, swap_chain_canvas,
};

pub fn run(
    title: &'static str,
    draw: for<'a> fn(&CanvasDrawContext<'a>) -> Result<()>,
) -> Result<()> {
    run_renderer(title, move |_| swap_chain_canvas(draw).build())
}

pub fn run_animated(
    title: &'static str,
    draw: for<'a> fn(&CanvasDrawContext<'a>) -> Result<()>,
) -> Result<()> {
    run_renderer(title, move |_| animated_canvas(draw).build())
}

pub fn run_component(
    title: &'static str,
    render: for<'a> fn(&mut RenderCx<'a>) -> Element,
) -> Result<()> {
    run_renderer(title, render)
}

fn run_renderer(
    title: &'static str,
    render: impl for<'a> Fn(&mut RenderCx<'a>) -> Element + 'static,
) -> Result<()> {
    windows_reactor::bootstrap()?;
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let content = render(cx);
        let windows = if open.value() {
            vec![
                Window::new(title, content, move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .build(),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}

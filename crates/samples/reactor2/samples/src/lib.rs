use windows_reactor2::{Application, Element, RenderCx, Window, component, run_reactor_winui_app};

pub fn run_with_window(
    title: impl Into<String>,
    configure: impl Fn(Window) -> Window + 'static,
    render: for<'a> fn(&mut RenderCx<'a>) -> Element,
) -> windows_core::Result<()> {
    windows_reactor2::bootstrap()?;
    let title = title.into();
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let windows = if open.value() {
            let content = component(render);
            vec![
                configure(Window::new(title.clone(), content, move || {
                    open.set(false);
                }))
                .build()
                .key(0),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });

    run_reactor_winui_app(root)
}

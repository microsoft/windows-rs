use super::*;

pub(in crate::winui) fn run_reactor_winui_async_fixture<F>(
    title: &str,
    root: Element,
    fixture: F,
) -> WindowsResult<()>
where
    F: FnOnce(&mut Reactor<WinUiRuntime>, Rc<dyn Fn(WindowsResult<()>)>) -> WindowsResult<()>
        + 'static,
{
    run_configured_async_fixture(title, root, |_| Ok(()), fixture)
}

#[cfg(feature = "canvas")]
pub(in crate::winui) fn run_reactor_winui_configured_async_fixture<C, F>(
    title: &str,
    root: Element,
    configure: C,
    fixture: F,
) -> WindowsResult<()>
where
    C: FnOnce(&mut Reactor<WinUiRuntime>) -> WindowsResult<()> + 'static,
    F: FnOnce(&mut Reactor<WinUiRuntime>, Rc<dyn Fn(WindowsResult<()>)>) -> WindowsResult<()>
        + 'static,
{
    run_configured_async_fixture(title, root, configure, fixture)
}

fn run_configured_async_fixture<C, F>(
    title: &str,
    root: Element,
    configure: C,
    fixture: F,
) -> WindowsResult<()>
where
    C: FnOnce(&mut Reactor<WinUiRuntime>) -> WindowsResult<()> + 'static,
    F: FnOnce(&mut Reactor<WinUiRuntime>, Rc<dyn Fn(WindowsResult<()>)>) -> WindowsResult<()>
        + 'static,
{
    let completion_error = Rc::new(RefCell::new(None));
    let callback_error = Rc::clone(&completion_error);
    run_reactor_winui_core(
        single_window_application(title, root, false),
        configure,
        move |reactor| {
            let finish_error = Rc::clone(&callback_error);
            let finish = Rc::new(move |result: WindowsResult<()>| {
                if let Err(error) = result {
                    finish_error.borrow_mut().get_or_insert(error);
                }
                terminate_host();
            });
            fixture(reactor, finish)
        },
    )?;
    if let Some(error) = completion_error.borrow_mut().take() {
        return Err(error);
    }
    Ok(())
}

pub(crate) fn run_reactor_winui_performance<F>(
    title: &str,
    root: Element,
    fullscreen: bool,
    on_render: F,
) -> WindowsResult<()>
where
    F: Fn(&crate::performance::RenderMetrics) + 'static,
{
    run_reactor_winui_core(
        single_window_application(title, root, fullscreen),
        move |reactor| {
            reactor.set_render_complete(on_render);
            Ok(())
        },
        |_| Ok(()),
    )
}

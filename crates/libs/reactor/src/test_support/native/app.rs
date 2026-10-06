use super::*;

impl AppContext {
    pub fn component_services(&self) -> Arc<dyn ComponentServices> {
        self.services.clone()
    }
}

fn with_primary_component_window<T>(
    operation: impl FnOnce(&ComponentWindowState) -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    APP_COMPONENT_APPLICATION.with(|application| {
        let application = application.borrow();
        let application = application.as_ref().ok_or("application is not running")?;
        let state = application.state.borrow();
        let window = state.windows.values().next().ok_or("window is not open")?;
        let window = window.state.borrow();
        let window = window.as_ref().ok_or("window is closed")?;
        operation(window)
    })
}

pub fn bring_live_virtual_index(index: usize) -> Result<(), &'static str> {
    with_primary_component_window(|window| {
        let collection = window
            .host
            .runtime()
            .graph()
            .objects()
            .find(|object| {
                window.host.runtime().graph().kind(*object) == Some(ObjectType::ItemsRepeater)
            })
            .ok_or("virtual collection is not mounted")?;
        window
            .host
            .runtime()
            .adapter()
            .realize_virtual_item(collection, index)
            .map_err(|_| "virtual index could not be realized")
    })
}

pub fn live_virtual_shell_counts() -> Result<(usize, usize), &'static str> {
    with_primary_component_window(|window| {
        Ok(window.host.runtime().adapter().virtual_shell_counts())
    })
}

pub struct LiveTickSubscription {
    _tick: windows_core::EventRevoker,
    timer: DispatcherQueueTimer,
}

impl Drop for LiveTickSubscription {
    fn drop(&mut self) {
        _ = self.timer.Stop();
    }
}

pub fn subscribe_live_tick(
    tick_callback: impl Fn() + 'static,
) -> windows_core::Result<LiveTickSubscription> {
    subscribe_live_interval(Duration::from_millis(16), tick_callback)
}

pub fn subscribe_live_interval(
    interval: Duration,
    tick_callback: impl Fn() + 'static,
) -> windows_core::Result<LiveTickSubscription> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let timer = dispatcher.CreateTimer()?;
    timer.SetInterval(TimeSpan::try_from(interval).unwrap())?;
    timer.SetIsRepeating(true)?;
    let tick = timer.Tick(move |_, _| {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&tick_callback)).is_err() {
            std::process::abort();
        }
    })?;
    timer.Start()?;
    Ok(LiveTickSubscription { _tick: tick, timer })
}

#[must_use = "dropping the subscription stops rendering notifications"]
pub struct LiveRenderingSubscription {
    _rendering: windows_core::EventRevoker,
}

pub fn subscribe_live_rendering(
    rendering_callback: impl Fn() + 'static,
) -> windows_core::Result<LiveRenderingSubscription> {
    let rendering = CompositionTarget::Rendering(move |_, _| {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&rendering_callback)).is_err() {
            std::process::abort();
        }
    })?;
    Ok(LiveRenderingSubscription {
        _rendering: rendering,
    })
}

pub fn schedule_live_test_exit(success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let handler = DispatcherQueueHandler::new(move || {
        std::process::exit(i32::from(!success));
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

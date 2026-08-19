use super::*;
use crate::tests::support::*;

mod access;
mod application;
mod calendar_view_access;
mod calendar_view_properties;
#[cfg(feature = "canvas")]
mod canvas;
mod collection_mutation;
mod collection_selection;
mod command_properties;
mod container_properties;
mod content_access;
mod content_properties;
mod control_chrome_access;
mod control_chrome_properties;
mod control_properties;
mod faults;
mod framework_access;
mod framework_layout;
mod hooks;
mod input;
mod lifecycle;
mod media_access;
mod media_properties;
mod navigation_access;
mod navigation_properties;
mod performance;
mod progress_access;
mod progress_properties;
mod rich_access;
mod rich_properties;
mod shape_access;
mod shape_properties;
mod single_selection;
mod status_access;
mod status_properties;
mod stress_performance;
mod time_picker_access;
mod time_picker_properties;
mod tooltip_access;
mod tooltip_properties;
mod typography_access;
mod typography_properties;
mod value_controls;
mod window_lifecycle;
mod window_properties;

use crate::app::Reactor;
use access::RuntimeProbe;

fn run_app_fixture(
    root: Element,
    fixture: impl FnOnce(&mut Reactor<WinUiRuntime>) -> windows_core::Result<()> + 'static,
) -> windows_core::Result<()> {
    host::run_reactor_winui_core(root, |_| Ok(()), fixture)
}

struct TestTimer {
    timer: bindings::DispatcherQueueTimer,
    _tick_revoker: windows_core::EventRevoker,
}

impl TestTimer {
    fn one_shot(delay: Duration, callback: impl Fn() + 'static) -> windows_core::Result<Self> {
        Self::new(delay, false, callback)
    }

    fn repeating(delay: Duration, callback: impl Fn() + 'static) -> windows_core::Result<Self> {
        Self::new(delay, true, callback)
    }

    fn new(
        delay: Duration,
        repeating: bool,
        callback: impl Fn() + 'static,
    ) -> windows_core::Result<Self> {
        let queue = bindings::DispatcherQueue::GetForCurrentThread()?;
        let timer = queue.CreateTimer()?;
        timer.SetInterval(TimeSpan {
            duration: i64::try_from(delay.as_nanos() / 100).unwrap(),
        })?;
        timer.SetIsRepeating(repeating)?;
        let tick_revoker = timer.Tick(move |_sender, _args| callback())?;
        timer.Start()?;
        Ok(Self {
            timer,
            _tick_revoker: tick_revoker,
        })
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        _ = self.timer.Stop();
    }
}

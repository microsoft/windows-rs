use std::time::Duration;

use windows_core::{Error, HRESULT, Result};
use windows_time::TimeSpan;

use crate::Element;
use crate::bindings::{CompositionTarget, DispatcherQueue, DispatcherQueueTimer};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderMetrics {
    pub tree_build_ms: f64,
    pub reconcile_ms: f64,
    pub effects_ms: f64,
    pub elements_diffed: u64,
    pub elements_skipped: u64,
    pub elements_created: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HostOptions {
    pub fullscreen: bool,
}

pub fn run_host<F>(title: &str, root: Element, options: HostOptions, on_render: F) -> Result<()>
where
    F: Fn(&RenderMetrics) + 'static,
{
    crate::winui::run_reactor_winui_performance(title, root, options.fullscreen, on_render)
}

pub fn request_exit() -> Result<()> {
    crate::winui::terminate_host();
    Ok(())
}

/// A dispatcher-queue timer that stops and revokes its callback when dropped.
pub struct DispatcherTimer {
    timer: DispatcherQueueTimer,
    _tick_revoker: windows_core::EventRevoker,
}

impl DispatcherTimer {
    pub fn repeating<F>(interval: Duration, callback: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(interval, true, callback)
    }

    pub fn one_shot<F>(delay: Duration, callback: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(delay, false, callback)
    }

    fn build<F>(interval: Duration, repeating: bool, callback: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        let queue = DispatcherQueue::GetForCurrentThread()?;
        let timer = queue.CreateTimer()?;
        timer.SetInterval(duration_to_timespan(interval)?)?;
        timer.SetIsRepeating(repeating)?;
        let tick_revoker = timer.Tick(move |_sender, _args| callback())?;
        timer.Start()?;
        Ok(Self {
            timer,
            _tick_revoker: tick_revoker,
        })
    }

    pub fn start(&self) -> Result<()> {
        self.timer.Start()
    }

    pub fn stop(&self) -> Result<()> {
        self.timer.Stop()
    }
}

impl Drop for DispatcherTimer {
    fn drop(&mut self) {
        _ = self.timer.Stop();
    }
}

/// A composition-frame subscription that revokes its callback when dropped.
pub struct RenderingSubscription {
    _revoker: windows_core::EventRevoker,
}

impl RenderingSubscription {
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        let revoker = CompositionTarget::Rendering(move |_sender, _args| callback())?;
        Ok(Self { _revoker: revoker })
    }
}

fn duration_to_timespan(value: Duration) -> Result<TimeSpan> {
    let ticks = value.as_nanos() / 100;
    let duration = i64::try_from(ticks).map_err(|_| {
        Error::new(
            HRESULT(0x80070057_u32 as i32),
            "timer duration exceeds the WinRT TimeSpan range",
        )
    })?;
    Ok(TimeSpan { duration })
}

mod tests {
    use super::*;

    #[test]
    fn duration_conversion_uses_100_nanosecond_ticks() {
        assert_eq!(
            duration_to_timespan(Duration::from_millis(33))
                .unwrap()
                .duration,
            330_000
        );
    }

    #[test]
    fn duration_conversion_rejects_overflow() {
        assert!(duration_to_timespan(Duration::MAX).is_err());
    }
}

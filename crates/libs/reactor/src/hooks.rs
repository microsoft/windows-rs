use std::time::Duration;

use super::*;
use bindings::*;

/// RAII timer wrapper; stops and unhooks on drop.
pub struct DispatcherTimer {
    timer: DispatcherQueueTimer,
    _tick_revoker: windows_core::EventRevoker,
}

impl DispatcherTimer {
    pub fn new<F>(interval: Duration, f: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(interval, true, f)
    }

    pub fn new_one_shot<F>(after: Duration, f: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::build(after, false, f)
    }

    fn build<F>(interval: Duration, repeating: bool, f: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        let queue = DispatcherQueue::GetForCurrentThread()?;
        let timer = queue.CreateTimer()?;
        timer.SetInterval(duration_to_timespan(interval))?;
        timer.SetIsRepeating(repeating)?;

        let tick_revoker = timer.Tick(move |_, _| {
            fault::catch("timer", &f);
        })?;
        timer.Start()?;
        Ok(Self {
            timer,
            _tick_revoker: tick_revoker,
        })
    }

    pub fn stop(&self) -> Result<()> {
        self.timer.Stop()
    }

    pub fn start(&self) -> Result<()> {
        self.timer.Start()
    }
}

impl Drop for DispatcherTimer {
    fn drop(&mut self) {
        let _ = self.timer.Stop();
    }
}

/// RAII handle for a `CompositionTarget::Rendering` subscription; detaches on drop.
pub struct Rendering {
    _revoker: windows_core::EventRevoker,
}

/// Subscribe `f` to `CompositionTarget::Rendering` for the current thread.
pub fn on_rendering<F>(f: F) -> Result<Rendering>
where
    F: Fn() + 'static,
{
    let revoker = CompositionTarget::Rendering(move |_, _| {
        fault::catch("rendering", &f);
    })?;
    Ok(Rendering { _revoker: revoker })
}

fn duration_to_timespan(d: Duration) -> TimeSpan {
    TimeSpan::try_from(d).unwrap_or(TimeSpan::MAX)
}

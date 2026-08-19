use super::*;

pub(in crate::winui) fn timer_count(runtime: &WinUiRuntime) -> usize {
    runtime.timers.len()
}

pub(in crate::winui) fn timer_ticks(runtime: &WinUiRuntime) -> Rc<Cell<usize>> {
    Rc::clone(&runtime.timer_ticks)
}

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Default)]
struct State {
    phase: &'static str,
    samples: BTreeMap<(&'static str, &'static str), (usize, Duration)>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

pub(super) struct Run(String);

pub(super) fn begin(label: String) -> Run {
    if std::env::var_os("WINDOWS_CLANG_PROFILE").is_some() {
        STATE.set(Some(State {
            phase: "cleanup",
            ..Default::default()
        }));
    }
    Run(label)
}

impl Drop for Run {
    fn drop(&mut self) {
        if let Some(state) = STATE.take() {
            let mut report = String::new();
            for ((phase, operation), (count, elapsed)) in state.samples {
                report.push_str(&format!(
                    "clang-profile\t{}\t{phase}\t{operation}\t{count}\t{:.6}\n",
                    self.0,
                    elapsed.as_secs_f64(),
                ));
            }
            eprint!("{report}");
        }
    }
}

pub(super) struct Phase {
    previous: &'static str,
    started: Instant,
}

pub(super) fn phase(name: &'static str) -> Phase {
    let previous = STATE.with_borrow_mut(|state| {
        state
            .as_mut()
            .map(|state| std::mem::replace(&mut state.phase, name))
            .unwrap_or_default()
    });
    Phase {
        previous,
        started: Instant::now(),
    }
}

impl Drop for Phase {
    fn drop(&mut self) {
        record("wall", self.started.elapsed());
        STATE.with_borrow_mut(|state| {
            if let Some(state) = state {
                state.phase = self.previous;
            }
        });
    }
}

pub(super) fn record(operation: &'static str, elapsed: Duration) {
    STATE.with_borrow_mut(|state| {
        if let Some(state) = state {
            let sample = state.samples.entry((state.phase, operation)).or_default();
            sample.0 += 1;
            sample.1 += elapsed;
        }
    });
}

pub(super) fn count(operation: &'static str, count: usize) {
    STATE.with_borrow_mut(|state| {
        if let Some(state) = state {
            state.samples.entry((state.phase, operation)).or_default().0 += count;
        }
    });
}

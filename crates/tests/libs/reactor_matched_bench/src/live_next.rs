mod live_support;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use live_support::{FrameAction, LiveTracker};
use test_reactor_matched_bench::{Model, Operation};
use windows_core::Result;
use windows_reactor_next::*;

struct MatchedLive {
    model: Model,
    _rendering: LiveRendering,
    tracker: Rc<RefCell<LiveTracker>>,
}

enum Message {
    Apply(Operation, usize),
}

impl Component for MatchedLive {
    type Message = Message;
    type Props = usize;

    fn create(samples: &usize, context: &mut ComponentContext<Self>) -> Self {
        let tracker = Rc::new(RefCell::new(LiveTracker::new(*samples)));
        let sender = context.sender();
        let rendering_tracker = Rc::clone(&tracker);
        let done = Arc::new(AtomicBool::new(false));
        let rendering_done = Arc::clone(&done);
        let window_prepared = Cell::new(false);
        let rendering = subscribe_live_rendering(move || {
            if !window_prepared.replace(true) {
                live_support::maximize_active_window();
                return;
            }
            let action = rendering_tracker.borrow_mut().on_frame();
            match action {
                FrameAction::Apply {
                    iteration,
                    operation,
                    start_measurement,
                } => {
                    if start_measurement {
                        clear_live_performance_times();
                        rendering_tracker.borrow_mut().begin_measurement();
                    }
                    sender.send(Message::Apply(operation, iteration));
                }
                FrameAction::Finish => {
                    if rendering_done.swap(true, Ordering::AcqRel) {
                        return;
                    }
                    let (mut dispatch, mut native) = take_live_performance_times();
                    let report = rendering_tracker.borrow().report(
                        "windows-reactor-next",
                        &mut dispatch,
                        &mut native,
                    );
                    live_support::write_report("next", &report);
                    schedule_live_test_exit(true).unwrap();
                }
                FrameAction::Settle => {}
            }
        })
        .unwrap();

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(30));
            if !done.swap(true, Ordering::AcqRel) {
                eprintln!("matched live next benchmark timed out");
                std::process::exit(2);
            }
        });

        Self {
            model: Model::default(),
            _rendering: rendering,
            tracker,
        }
    }

    fn update(&mut self, message: Message, _context: &mut ComponentContext<Self>) {
        match message {
            Message::Apply(operation, iteration) => self.model.apply(operation, iteration),
        }
    }

    fn view(&self, _props: &usize, _context: &mut ViewContext<Self>) -> View {
        std::hint::black_box(&self.tracker);
        test_reactor_matched_bench::next::view(&self.model)
    }
}

fn main() -> Result<()> {
    bootstrap()?;
    App::run_component::<MatchedLive>(live_support::samples())
}

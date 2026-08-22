mod live_support;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use live_support::{FrameAction, LiveTracker};
use test_reactor_matched_bench::Model;
use windows_core::Result;
use windows_reactor::*;

struct Driver {
    model: Model,
    tracker: LiveTracker,
}

struct MatchedLive;

static SAMPLES: OnceLock<usize> = OnceLock::new();

impl Component for MatchedLive {
    fn render(&self, _props: &(), context: &mut RenderCx) -> Element {
        let (generation, set_generation) = context.use_state(0usize);
        let driver_ref = context.use_ref::<Option<Rc<RefCell<Driver>>>>(None);
        if driver_ref.borrow().is_none() {
            driver_ref.set(Some(Rc::new(RefCell::new(Driver {
                model: Model::default(),
                tracker: LiveTracker::new(*SAMPLES.get().unwrap()),
            }))));
        }
        let driver = driver_ref.borrow().clone().unwrap();

        let rendering_ref = context.use_ref::<Option<Rendering>>(None);
        let done_ref = context.use_ref::<Option<Arc<AtomicBool>>>(None);
        if rendering_ref.borrow().is_none() {
            let done = Arc::new(AtomicBool::new(false));
            done_ref.set(Some(Arc::clone(&done)));
            let rendering_driver = Rc::clone(&driver);
            let rendering_done = Arc::clone(&done);
            let window_prepared = Cell::new(false);
            let rendering = on_rendering(move || {
                if !window_prepared.replace(true) {
                    live_support::maximize_active_window();
                    return;
                }
                let action = rendering_driver.borrow_mut().tracker.on_frame();
                match action {
                    FrameAction::Apply {
                        iteration,
                        operation,
                        start_measurement,
                    } => {
                        if start_measurement {
                            rendering_driver.borrow_mut().tracker.begin_measurement();
                        }
                        rendering_driver
                            .borrow_mut()
                            .model
                            .apply(operation, iteration);
                        set_generation.call(iteration + 1);
                    }
                    FrameAction::Finish => {
                        if rendering_done.swap(true, Ordering::AcqRel) {
                            return;
                        }
                        let report = rendering_driver.borrow().tracker.report(
                            "windows-reactor",
                            &mut [],
                            &mut [],
                        );
                        live_support::write_report("incumbent", &report);
                        std::process::exit(0);
                    }
                    FrameAction::Settle => {}
                }
            })
            .unwrap();
            rendering_ref.set(Some(rendering));

            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(30));
                if !done.swap(true, Ordering::AcqRel) {
                    eprintln!("matched live incumbent benchmark timed out");
                    std::process::exit(2);
                }
            });
        }

        let phases_ref = context.use_ref(false);
        if !*phases_ref.borrow() {
            phases_ref.set(true);
            let phases_driver = Rc::clone(&driver);
            with_active_host(|host| {
                host.set_render_complete(move |info: &RenderCompleteInfo| {
                    phases_driver.borrow_mut().tracker.record_incumbent_phases(
                        info.tree_build_ms,
                        info.reconcile_ms,
                        info.effects_ms,
                    );
                });
            });
        }

        std::hint::black_box(generation);
        test_reactor_matched_bench::incumbent::view(&driver.borrow().model)
    }
}

fn main() -> Result<()> {
    bootstrap()?;
    SAMPLES.set(live_support::samples()).unwrap();
    App::new()
        .title("windows-reactor matched live")
        .run(|| MatchedLive)
}

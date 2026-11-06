#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use tracelogging as tlg;
use windows_core::Result;
use windows_reactor::*;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

const APP_NAME: &str = "blank_windows_reactor";
const STARTUP_KEYWORD: u64 = 0x0000_4000_0000_0000;

tlg::define_provider!(
    STARTUP_PROVIDER,
    "BenchmarkSyntheticApps",
    id("fd80d616-e92b-4b2b-9bed-131ada36a8fd"),
);

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct ProviderRegistration;

impl Drop for ProviderRegistration {
    fn drop(&mut self) {
        _ = STARTUP_PROVIDER.unregister();
    }
}

fn register_provider() -> std::io::Result<ProviderRegistration> {
    // SAFETY: This executable keeps the registration alive until process termination.
    let error = unsafe { STARTUP_PROVIDER.register() };
    if error == 0 {
        Ok(ProviderRegistration)
    } else {
        Err(std::io::Error::from_raw_os_error(error as i32))
    }
}

fn event_payload() -> (u64, u32) {
    (SEQUENCE.fetch_add(1, Ordering::Relaxed), std::process::id())
}

fn trace_xaml_app_loaded() {
    let (sequence, process_id) = event_payload();
    let _ = tlg::write_event!(
        STARTUP_PROVIDER,
        "XamlAppLoaded",
        level(Informational),
        keyword(STARTUP_KEYWORD),
        id_version(2, 0),
        str8("AppName", APP_NAME),
        u64("Seq", &sequence),
        u32("Pid", &process_id),
    );
}

fn trace_window_loaded() {
    let (sequence, process_id) = event_payload();
    let _ = tlg::write_event!(
        STARTUP_PROVIDER,
        "WindowLoaded",
        level(Informational),
        keyword(STARTUP_KEYWORD),
        id_version(3, 0),
        str8("AppName", APP_NAME),
        u64("Seq", &sequence),
        u32("Pid", &process_id),
    );
}

fn trace_first_render() {
    let (sequence, process_id) = event_payload();
    let _ = tlg::write_event!(
        STARTUP_PROVIDER,
        "FirstRender",
        level(Informational),
        keyword(STARTUP_KEYWORD),
        id_version(4, 0),
        str8("AppName", APP_NAME),
        u64("Seq", &sequence),
        u32("Pid", &process_id),
    );
}

fn trace_first_idle() {
    let (sequence, process_id) = event_payload();
    let _ = tlg::write_event!(
        STARTUP_PROVIDER,
        "FirstIdle",
        level(Informational),
        keyword(STARTUP_KEYWORD),
        id_version(5, 0),
        str8("AppName", APP_NAME),
        u64("Seq", &sequence),
        u32("Pid", &process_id),
    );
}

fn trace_win_main_entry() {
    let (sequence, process_id) = event_payload();
    let _ = tlg::write_event!(
        STARTUP_PROVIDER,
        "wWinMainEntry",
        level(Informational),
        keyword(STARTUP_KEYWORD),
        id_version(1, 0),
        str8("AppName", APP_NAME),
        u64("Seq", &sequence),
        u32("Pid", &process_id),
    );
}

fn trace_process_stop() {
    let (sequence, process_id) = event_payload();
    let _ = tlg::write_event!(
        STARTUP_PROVIDER,
        "ProcessStop",
        level(Informational),
        keyword(STARTUP_KEYWORD),
        id_version(6, 0),
        str8("AppName", APP_NAME),
        u64("Seq", &sequence),
        u32("Pid", &process_id),
    );
}

struct Sample {
    rendering: Rc<RefCell<Option<windows_core::EventRevoker>>>,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        trace_xaml_app_loaded();
        Self {
            rendering: Rc::new(RefCell::new(None)),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("BlankWindowsReactor");
        context.window_visuals(WindowVisuals::new().client_size(1000.0, 1000.0));
        let rendering = Rc::clone(&self.rendering);
        let completion = context.completion();
        context.use_effect("startup-tracing", (), move || {
            trace_window_loaded();
            let first_rendered = AtomicBool::new(false);
            let revoker = bindings::CompositionTarget::Rendering(move |_, _| {
                if first_rendered.swap(true, Ordering::Relaxed) {
                    return;
                }
                trace_first_render();
                let completion = completion.clone();
                let idle = bindings::DispatcherQueueHandler::new(move || {
                    trace_first_idle();
                    _ = completion.clone().complete(());
                });
                let dispatcher = bindings::DispatcherQueue::GetForCurrentThread().unwrap();
                assert!(
                    dispatcher
                        .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Low, &idle)
                        .unwrap(),
                    "failed to enqueue the FirstIdle marker"
                );
            })
            .unwrap();
            *rendering.borrow_mut() = Some(revoker);
            Some(Box::new(move || {
                rendering.borrow_mut().take();
            }))
        });
        Border::new()
            .padding(Thickness::uniform(12.0))
            .content(
                TextBlock::new()
                    .text("Blank Windows Reactor")
                    .font_size(14.0),
            )
            .into()
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.rendering.borrow_mut().take();
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _provider = register_provider()?;
    trace_win_main_entry();
    let result: Result<()> = App::run_component::<Sample>(());
    trace_process_stop();
    result?;
    Ok(())
}

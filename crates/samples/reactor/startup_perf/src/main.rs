#![windows_subsystem = "windows"]

use std::sync::atomic::{AtomicU64, Ordering};

use windows_reactor::*;
use windows_tracing::{GUID, Level, Registration, Result, define_provider, write_event};

const APP_NAME: &str = "blank_windows_reactor";
const STARTUP_KEYWORD: u64 = 0x0000_4000_0000_0000;

define_provider!(
    STARTUP_PROVIDER,
    "BenchmarkSyntheticApps",
    id(GUID::from_u128(0xfd80d616_e92b_4b2b_9bed_131ada36a8fd))
);

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn register_provider() -> Result<Registration> {
    // SAFETY: This executable keeps the registration alive until process termination.
    unsafe { STARTUP_PROVIDER.register() }
}

fn event_payload() -> (u64, u32) {
    (SEQUENCE.fetch_add(1, Ordering::Relaxed), std::process::id())
}

fn trace_win_main_entry() {
    let (sequence, process_id) = event_payload();
    let _ = write_event!(
        STARTUP_PROVIDER,
        "wWinMainEntry",
        level(Level::INFORMATIONAL),
        keyword(STARTUP_KEYWORD),
        id_version(1, 0),
        str("AppName", APP_NAME),
        u64("Seq", sequence),
        u32("Pid", process_id),
    );
}

fn trace_xaml_app_loaded() {
    let (sequence, process_id) = event_payload();
    let _ = write_event!(
        STARTUP_PROVIDER,
        "XamlAppLoaded",
        level(Level::INFORMATIONAL),
        keyword(STARTUP_KEYWORD),
        id_version(2, 0),
        str("AppName", APP_NAME),
        u64("Seq", sequence),
        u32("Pid", process_id),
    );
}

fn trace_window_loaded() {
    let (sequence, process_id) = event_payload();
    let _ = write_event!(
        STARTUP_PROVIDER,
        "WindowLoaded",
        level(Level::INFORMATIONAL),
        keyword(STARTUP_KEYWORD),
        id_version(3, 0),
        str("AppName", APP_NAME),
        u64("Seq", sequence),
        u32("Pid", process_id),
    );
}

fn trace_first_render() {
    let (sequence, process_id) = event_payload();
    let _ = write_event!(
        STARTUP_PROVIDER,
        "FirstRender",
        level(Level::INFORMATIONAL),
        keyword(STARTUP_KEYWORD),
        id_version(4, 0),
        str("AppName", APP_NAME),
        u64("Seq", sequence),
        u32("Pid", process_id),
    );
}

fn trace_first_idle() {
    let (sequence, process_id) = event_payload();
    let _ = write_event!(
        STARTUP_PROVIDER,
        "FirstIdle",
        level(Level::INFORMATIONAL),
        keyword(STARTUP_KEYWORD),
        id_version(5, 0),
        str("AppName", APP_NAME),
        u64("Seq", sequence),
        u32("Pid", process_id),
    );
}

fn trace_process_stop() {
    let (sequence, process_id) = event_payload();
    let _ = write_event!(
        STARTUP_PROVIDER,
        "ProcessStop",
        level(Level::INFORMATIONAL),
        keyword(STARTUP_KEYWORD),
        id_version(6, 0),
        str("AppName", APP_NAME),
        u64("Seq", sequence),
        u32("Pid", process_id),
    );
}

fn app(cx: &mut RenderCx) -> Element {
    let xaml_app_loaded = cx.use_ref(false);
    if !*xaml_app_loaded.borrow() {
        xaml_app_loaded.set(true);
        trace_xaml_app_loaded();
    }

    let first_rendered = cx.use_ref(false);
    let rendering = cx.use_ref::<Option<Rendering>>(None);
    cx.use_effect((), {
        let rendering_for_callback = rendering.clone();
        move || {
            trace_window_loaded();

            rendering.set(Some(
                on_rendering(move || {
                    if first_rendered.replace(true) {
                        return;
                    }

                    trace_first_render();

                    let dispatcher = WinUIDispatcher::for_current_thread().unwrap();
                    let rendering = rendering_for_callback.clone();
                    assert!(
                        dispatcher.enqueue(
                            DispatcherQueuePriority::Low,
                            Box::new(move || {
                                trace_first_idle();
                                rendering.set(None);
                            }),
                        ),
                        "failed to enqueue the FirstIdle marker"
                    );
                })
                .unwrap(),
            ));
        }
    });

    text_block("Blank Windows Reactor")
        .font_size(14.0)
        .padding(12.0)
        .into()
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _provider = register_provider()?;
    trace_win_main_entry();

    let result = bootstrap().and_then(|()| {
        App::new()
            .title("BlankWindowsReactor")
            .inner_size(1000.0, 1000.0)
            .on_exit(trace_process_stop)
            .render(app)
    });

    if result.is_err() {
        trace_process_stop();
    }
    result?;
    Ok(())
}

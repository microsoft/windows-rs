//! Sample for `App::on_fault`.
//!
//! Reactor callbacks run behind WinUI `extern "system"` delegates that cannot
//! unwind, so a panic in an event handler would abort the process. `App::on_fault`
//! installs one hook that catches panics from every reactor callback and turns
//! them into a *controlled* fault instead of an abort.
//!
//! `panic!` is thus decoupled from fail-fast: the default policy is
//! log-and-continue. When a fault is genuinely unrecoverable, the handler
//! escalates it with `std::process::exit` (uncatchable), giving per-fault control
//! over which failures are fatal.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0usize);

    vstack((
        TitleBar::new("windows_reactor — on_fault"),
        text_block(format!("Handler invocations: {clicks}")),
        text_block(
            "Both buttons panic inside their click handler. on_fault catches both: it \
             logs and continues for the recoverable one, and escalates the fatal one \
             to process exit. Watch stderr.",
        )
        .font_size(13.0),
        button("Recoverable panic (logged, app continues)").on_click(move || {
            set_clicks.call(clicks + 1);
            panic!("recoverable: a bug in this handler");
        }),
        button("Fatal panic (handler escalates to exit)").on_click(|| {
            panic!("fatal: unrecoverable invariant violated");
        }),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;

    App::new()
        .title("on_fault")
        .inner_size(560.0, 300.0)
        .on_fault(|fault: &Fault| {
            eprintln!("on_fault: {} fault: {}", fault.context, fault.message);
            // process::exit is not caught by the fault boundary, so it truly ends
            // the process — the escape hatch for a fault we deem unrecoverable.
            if fault.message.starts_with("fatal:") {
                eprintln!("on_fault: escalating to process exit");
                std::process::exit(1);
            }
        })
        .render(app)
}

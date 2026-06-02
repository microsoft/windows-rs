#![windows_subsystem = "windows"]

//! Demonstrates panic hook and crash-log behavior.
//!
//! Run with: `cargo run -p examples --example diagnostics_demo`

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (status, set_status) = cx.use_state(String::from("Ready"));

    vstack((
        text_block("Diagnostics Demo").font_size(28.0).bold(),
        text_block("Demonstrates the panic hook and crash-log behavior.").opacity(0.7),
        text_block(format!("Status: {status}")),
        button("Trigger panic (process will exit)")
            .on_click(|| panic!("Deliberate panic — diagnostics demo")),
        button("Increment counter").on_click({
            let set = set_status;
            move || {
                set.call("Button clicked — app is responsive".into());
            }
        }),
        text_block("After the panic, check:").bold(),
        text_block("  • stderr for the full backtrace"),
        text_block("  • %TEMP%/windows-reactor-crash-{pid}.log"),
        text_block("  • Exit code 101 (echo %ERRORLEVEL%)"),
    ))
    .spacing(8.0)
    .padding(Thickness::uniform(24.0))
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Diagnostics Demo").render(app)
}

#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor_next::*;

fn main() -> Result<()> {
    bootstrap()?;
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(30));
        eprintln!("windows-reactor-next self-test timed out");
        std::process::exit(1);
    });

    App::run_windows([
        primary as fn(&mut Hooks) -> Element,
        secondary as fn(&mut Hooks) -> Element,
    ])?;
    Err(Error::new(
        HRESULT(0x80004005_u32 as _),
        "windows-reactor-next self-test returned before its completion marker",
    ))
}

fn primary(hooks: &mut Hooks) -> Element {
    hooks.use_effect((), move || {
        let passed = live_resources_installed().unwrap_or(false);
        if let Err(error) = schedule_live_controlled_repair_test(passed) {
            eprintln!("could not start live backend fixture: {error}");
            std::process::exit(1);
        }
        Some(Box::new(mark_live_test_cleanup as fn()))
    });
    TextBox::new()
        .text("fixed")
        .on_text_changed(record_live_primary_event)
        .into()
}

fn secondary(_hooks: &mut Hooks) -> Element {
    TextBox::new()
        .text("second")
        .on_text_changed(record_live_secondary_event)
        .into()
}

#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor_next::*;

fn main() -> Result<()> {
    bootstrap()?;

    App::run(move |hooks| {
        hooks.use_effect((), move || {
            let passed = live_resources_installed().unwrap_or(false);
            _ = schedule_live_test_exit(passed);
            None
        });
        TextBlock::new()
            .text("windows-reactor-next self-test")
            .into()
    })
}

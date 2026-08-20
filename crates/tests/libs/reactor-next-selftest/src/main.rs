#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor_next::*;

fn main() -> Result<()> {
    bootstrap()?;

    App::run(move |hooks| {
        hooks.use_effect((), move || {
            let passed = live_resources_installed().unwrap_or(false);
            _ = schedule_live_controlled_repair_test(passed);
            None
        });
        TextBox::new().text("fixed").into()
    })
}

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

    App::run(move |hooks| {
        hooks.use_effect((), move || {
            let passed = live_resources_installed().unwrap_or(false);
            if let Err(error) = schedule_live_controlled_repair_test(passed) {
                eprintln!("could not start live backend fixture: {error}");
                std::process::exit(1);
            }
            Some(Box::new(mark_live_test_cleanup as fn()))
        });
        TextBox::new().text("fixed").into()
    })
}

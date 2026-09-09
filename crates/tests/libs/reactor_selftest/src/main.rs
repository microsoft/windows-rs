#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor::*;

mod fixtures;
mod runner;

fn main() -> Result<()> {
    let mut arguments = std::env::args().skip(1);
    let mut filter = None;
    while let Some(argument) = arguments.next() {
        if argument == "--filter" {
            filter = arguments.next();
        } else if let Some(value) = argument.strip_prefix("--filter=") {
            filter = Some(value.to_string());
        }
    }
    let selected = match runner::select_fixtures(filter.as_deref()) {
        Ok(selected) => selected,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    std::thread::spawn(|| {
        std::thread::sleep(runner::SUITE_TIMEOUT);
        eprintln!("windows-reactor self-test timed out");
        std::process::exit(1);
    });

    App::run_component::<runner::FixtureRunner>(selected)
}

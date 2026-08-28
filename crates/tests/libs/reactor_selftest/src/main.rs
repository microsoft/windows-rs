#![windows_subsystem = "console"]

use windows_core::*;
use windows_reactor::*;

mod fixtures;
mod runner;

fn main() -> Result<()> {
    std::thread::spawn(|| {
        std::thread::sleep(runner::SUITE_TIMEOUT);
        eprintln!("windows-reactor self-test timed out");
        std::process::exit(1);
    });

    App::run_component::<runner::FixtureRunner>(())
}

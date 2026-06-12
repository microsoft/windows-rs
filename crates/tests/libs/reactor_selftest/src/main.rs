#![windows_subsystem = "console"]
#![allow(non_snake_case)]

use std::io::{IsTerminal, Write};
use std::time::Duration;

use windows::core::Result;

use windows_reactor::*;

#[allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::transmute_ptr_to_ptr,
    clippy::useless_transmute,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments
)]
mod bindings;
mod exec;
mod fixtures;
mod harness;
mod registry;

use crate::bindings::DispatcherQueue;

fn main() -> Result<()> {
    let _bootstrap = bootstrap::initialize()?;
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--list-fixtures") {
        for (name, _) in registry::FIXTURES {
            println!("{name}");
        }
        let _ = std::io::stdout().flush();
        return Ok(());
    }

    let filter: Option<String> = args
        .iter()
        .position(|a| a == "--filter")
        .and_then(|i| args.get(i + 1).cloned());

    let slow = args.iter().any(|a| a == "--slow");

    // CI / headless callers can force headless mode (auto-exit when the run
    // finishes) without relying on stdout TTY detection. Matches the
    // `--headless` flag used by `stress_perf`.
    let force_headless = args.iter().any(|a| a == "--headless");

    let interactive = !force_headless && std::io::stdout().is_terminal();

    run_self_test(filter, slow, interactive)
}

fn run_self_test(filter: Option<String>, slow: bool, interactive: bool) -> Result<()> {
    App::new()
        .title("windows_reactor — self-test")
        .run_custom(move |_app| {
            let harness = harness::Harness::new("windows_reactor — self-test")?;

            let fixtures: Vec<(&'static str, registry::FixtureFn)> = registry::FIXTURES
                .iter()
                .filter(|(n, _)| match &filter {
                    Some(f) => n.to_ascii_lowercase().contains(&f.to_ascii_lowercase()),
                    None => true,
                })
                .copied()
                .collect();

            harness.setup_titlebar(fixtures.len())?;
            harness.activate()?;

            let dispatcher = DispatcherQueue::GetForCurrentThread()?;
            exec::spawn_root(run_all(harness, fixtures, slow, interactive), dispatcher);
            Ok(())
        })
}

async fn run_all(
    harness: harness::Harness,
    fixtures: Vec<(&'static str, registry::FixtureFn)>,
    slow: bool,
    interactive: bool,
) {
    harness.render().await;

    println!("TAP version 14");
    println!("1..{}", fixtures.len());
    let _ = std::io::stdout().flush();

    for (idx, (name, f)) in fixtures.iter().enumerate() {
        let test_index = idx + 1;
        let _ = harness.update_progress(test_index, name);
        let failures_before = harness.failures();

        let capture = harness.capture_stderr();
        f(harness.clone()).await;
        let stderr = capture.finish();
        harness.check_no_reactor_warnings(&format!("{name}_NoWarnings"), &stderr);

        let passed = harness.failures() == failures_before;
        if !passed {
            println!("# FAILED: {name}");
            let _ = std::io::stdout().flush();
        }
        let _ = harness.mark_fixture_result(idx, passed);

        if slow {
            std::thread::sleep(Duration::from_millis(400));
        }
    }

    println!("# Total failures: {}", harness.failures());
    let _ = std::io::stdout().flush();
    harness.finalize_taskbar();

    let code = if harness.failures() > 0 { 1 } else { 0 };
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();

    if interactive {
        let summary = if harness.failures() == 0 {
            "all fixtures passed"
        } else {
            "some fixtures failed (see TAP above)"
        };
        eprintln!(
            "selftest-host: {summary}. Close the window to exit. \
             Use --slow to pace fixtures, or --filter <substr> to focus on one."
        );
    } else {
        std::process::exit(code);
    }
}

//! Live self-test for `windows-webview`.
//!
//! Hosts a real WebView2 in a [`windows_window`] window and drives each unique
//! capability end-to-end through the real COM surface and message pump,
//! reporting results as TAP. It is the WebView2 analogue of
//! `test_reactor_selftest`.
//!
//! WebView2 has no headless/software mode, so this is a live binary rather than
//! a unit-test crate. Run it with:
//!
//! ```text
//! cargo run -p test_webview                 # interactive: keeps the window open
//! cargo run -p test_webview -- --headless   # CI: auto-exits with a status code
//! cargo run -p test_webview -- --filter Ipc # run a subset by name substring
//! cargo run -p test_webview -- --list-fixtures
//! ```
//!
//! Requires the Microsoft Edge WebView2 runtime. If it is unavailable the run
//! reports a TAP skip and exits successfully so CI stays green.

#![windows_subsystem = "console"]

use std::io::{IsTerminal, Write};

mod fixtures;
mod harness;
mod registry;

use harness::Harness;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--list-fixtures") {
        for (name, _) in registry::FIXTURES {
            println!("{name}");
        }
        return;
    }

    let filter = args
        .iter()
        .position(|arg| arg == "--filter")
        .and_then(|index| args.get(index + 1).cloned());
    let headless = args.iter().any(|arg| arg == "--headless");
    let interactive = !headless && std::io::stdout().is_terminal();

    let harness = match Harness::bootstrap("windows-webview - self-test") {
        Ok(harness) => harness,
        Err(error) => {
            println!("TAP version 14");
            println!("1..0");
            println!("# SKIP windows-webview: WebView2 runtime unavailable ({error:?})");
            let _ = std::io::stdout().flush();
            return;
        }
    };

    let fixtures: Vec<&(&str, registry::FixtureFn)> = registry::FIXTURES
        .iter()
        .filter(|(name, _)| match &filter {
            Some(filter) => name
                .to_ascii_lowercase()
                .contains(&filter.to_ascii_lowercase()),
            None => true,
        })
        .collect();

    println!("TAP version 14");
    println!("1..{}", fixtures.len());
    let _ = std::io::stdout().flush();

    for (index, entry) in fixtures.iter().enumerate() {
        let (name, fixture) = **entry;
        let number = index + 1;
        let before = harness.failures();

        harness.reset();
        fixture(&harness);

        if harness.failures() == before {
            println!("ok {number} - {name}");
        } else {
            println!("not ok {number} - {name}");
        }
        let _ = std::io::stdout().flush();
    }

    let failures = harness.failures();
    println!("# {failures} failure(s) across {} fixtures", fixtures.len());
    let _ = std::io::stdout().flush();

    if interactive {
        eprintln!("self-test: fixtures complete. Close the window to exit.");
        windows_window::run();
    }

    std::process::exit(if failures > 0 { 1 } else { 0 });
}

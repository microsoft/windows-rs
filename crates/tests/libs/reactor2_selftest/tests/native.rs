use std::time::Duration;
use test_reactor2_support::{Automation, TestProcess, assert_process_success};

const TIMEOUT: Duration = Duration::from_secs(20);
const WINDOW_NAME: &str = "windows-reactor native self-test";

#[test]
fn smoke() {
    let process =
        TestProcess::spawn_executable(env!("CARGO_BIN_EXE_test_reactor2_selftest"), &[], &[])
            .unwrap();
    let automation = Automation::new().unwrap();
    let window = automation
        .wait_for_window(process.id(), WINDOW_NAME, TIMEOUT)
        .unwrap();

    automation
        .wait_for_descendant_name(&window, "Rows: 5000", TIMEOUT)
        .unwrap();
    automation
        .wait_for_descendant_name(&window, "Text value: initial", TIMEOUT)
        .unwrap();
    automation
        .wait_for_descendant_name(&window, "Checked value: false", TIMEOUT)
        .unwrap();

    window.close_window().unwrap();
    let output = process.wait(TIMEOUT).unwrap();
    assert_process_success(&output);
}

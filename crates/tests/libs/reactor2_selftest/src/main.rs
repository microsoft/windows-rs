#![windows_subsystem = "console"]

use windows_reactor2::{run_reactor_winui, stack_panel, text_block};

fn main() -> windows_core::Result<()> {
    run_reactor_winui(
        "windows-reactor native self-test",
        stack_panel([
            text_block("Rows: 5000"),
            text_block("Text value: initial"),
            text_block("Checked value: false"),
        ]),
    )
}

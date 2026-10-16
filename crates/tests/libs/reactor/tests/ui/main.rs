use bindings::*;
use std::time::{Duration, Instant};

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}

mod automation;
mod process;

use automation::*;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[test]
fn menu_callback_replacement_and_removal() -> Result<()> {
    process::run("menu", |ui| {
        ui.name("state", "Generation 0; attached true")?;
        ui.invoke("open")?;
        let original = ui.menu_item("Command")?;
        ui.invoke("replace")?;
        ui.name("state", "Generation 1; attached true")?;
        let current = ui.menu_item("Command")?;
        unsafe {
            assert!(ui.client.CompareElements(&original, &current)?.as_bool());
            original
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
                .Invoke()
                .ok()?;
        }
        ui.name("selection", "Selected 1; calls 1")?;
        ui.invoke("open")?;
        let stale = unsafe {
            ui.menu_item("Command")?
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
        };
        ui.invoke("remove")?;
        ui.name("state", "Generation 1; attached false")?;
        ui.menu_gone("Command")?;
        let result = unsafe { stale.Invoke() };
        assert!(
            result.is_ok()
                || result == ELEMENT_UNAVAILABLE
                || result == windows_core::HRESULT(0x80004005u32 as i32),
            "unexpected retired-provider result: {result:?}"
        );
        ui.name_stays(
            "selection",
            "Selected 1; calls 1",
            Duration::from_millis(250),
        )
    })
}

#[test]
fn text_box_native_edit_and_controlled_reset() -> Result<()> {
    process::run("text", |ui| {
        ui.value("input", "")?;
        ui.set_value("input", "native edit")?;
        ui.name("echo", "Text: native edit")?;
        ui.value("input", "native edit")?;
        ui.invoke("reset")?;
        ui.name("echo", "Text: ")?;
        ui.value("input", "")?;
        ui.type_text("input", "x")?;
        ui.name("echo", "Text: x")?;
        ui.value("input", "x")
    })
}

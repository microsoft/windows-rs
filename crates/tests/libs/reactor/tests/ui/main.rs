#![cfg(target_arch = "x86_64")]

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
mod drag;
mod pointer;
mod process;

use automation::*;
use pointer::*;

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

#[test]
fn pointer_capture_policy_at_mount() -> Result<()> {
    process::run("pointer-capture-mounted", |ui| capture_policy(ui, false))
}

#[test]
fn pointer_capture_policy_added_to_existing_border() -> Result<()> {
    process::run("pointer-capture-added", |ui| capture_policy(ui, true))
}

#[test]
fn pointer_focus_policy_at_mount() -> Result<()> {
    process::run("pointer-focus-mounted", |ui| focus_policy(ui, false))
}

#[test]
fn pointer_focus_policy_added_to_existing_border() -> Result<()> {
    process::run("pointer-focus-added", |ui| focus_policy(ui, true))
}

fn enable_pointer_policy(ui: &Automation) -> Result<()> {
    let original = ui.element("left")?;
    ui.name("policy", "Absent")?;
    ui.pointer_move("left", 0.5, 2.0)?;
    ui.name("pointer", "Left false; outside false")?;
    ui.pointer_button(true)?;
    ui.pointer_move("right", 0.5, 2.0)?;
    ui.name("pointer", "Left true; outside false")?;
    ui.pointer_button(false)?;
    ui.pointer_move("left", 0.5, 2.0)?;
    ui.name("pointer", "Left false; outside false")?;
    ui.name("focus", "Unfocused")?;
    ui.invoke("enable")?;
    ui.name("policy", "Enabled")?;
    unsafe {
        assert!(
            ui.client
                .CompareElements(&original, &ui.element("left")?)?
                .as_bool()
        );
    }
    Ok(())
}

fn capture_policy(ui: &Automation, added: bool) -> Result<()> {
    ui.focus("baseline")?;
    if added {
        enable_pointer_policy(ui)?;
    }
    ui.pointer_move("left", 0.5, 2.0)?;
    ui.name("pointer", "Left false; outside false")?;
    ui.pointer_button(true)?;
    ui.pointer_move("right", 0.5, 2.0)?;
    ui.name("pointer", "Left true; outside false")?;
    ui.pointer_move("outside", 0.5, 0.5)?;
    ui.name("pointer", "Left true; outside true")?;
    ui.name("outside-state", "No released move outside")?;
    ui.pointer_button(false)?;
    ui.pointer_move("outside", 0.75, 0.5)?;
    ui.name("outside-state", "Released move outside")?;
    ui.pointer_move("right", 0.5, 2.0)?;
    ui.name("pointer", "Left false; outside false")?;
    ui.name("focus", "Unfocused")
}

fn focus_policy(ui: &Automation, added: bool) -> Result<()> {
    ui.focus("baseline")?;
    if added {
        enable_pointer_policy(ui)?;
    }
    ui.pointer_move("right", 0.5, 2.0)?;
    ui.name("pointer", "Left false; outside false")?;
    ui.pointer_button(true)?;
    ui.pointer_move("left", 0.5, 2.0)?;
    ui.name("pointer", "Left true; outside false")?;
    ui.name("focus", "Unfocused")?;
    ui.pointer_button(false)?;
    ui.name("focus", "Pointer focused")
}

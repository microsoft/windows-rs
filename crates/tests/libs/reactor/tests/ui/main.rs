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

mod accessibility;
mod application;
mod automation;
mod background;
mod canvas;
mod controlled;
mod dialog;
mod dialog_theme;
mod drag;
mod flyout;
mod image;
mod keyboard;
mod list;
mod menu;
mod pointer;
mod process;
mod reference;
mod repeater;
mod retirement;
mod selection;
mod tab;
mod text;
mod timer;
mod tree;
mod typography;
mod window;

use automation::*;
use pointer::*;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
fn keyed_reorder_moves_only_displaced_children() -> Result<()> {
    process::run("keyed-reorder", |ui| {
        let keys = ["a", "b", "c", "d"];
        let before = keys
            .iter()
            .map(|key| ui.element(key))
            .collect::<Result<Vec<_>>>()?;
        ui.type_text("b", "x")?;
        ui.name("order", "Order d a b c")?;
        for (key, original) in keys.iter().zip(&before) {
            unsafe {
                assert!(
                    ui.client
                        .CompareElements(original, &ui.element(key)?)?
                        .as_bool(),
                    "{key:?} lost its native identity"
                );
            }
        }
        let reordered = [&before[3], &before[0], &before[1], &before[2]];
        ui.wait(
            "native order becomes d a b c with distinct row positions",
            || {
                reordered
                    .iter()
                    .map(|element| Ok(unsafe { element.CurrentBoundingRectangle()?.top }))
                    .collect::<windows_core::Result<Vec<_>>>()
            },
            |tops| tops.windows(2).all(|pair| pair[0] < pair[1]),
        )?;
        // Removing and reinserting an element drops its focus, so b must not have been moved.
        ui.focused("b")
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

#[test]
fn queued_clicks_reach_the_callback_they_fired_under() -> Result<()> {
    process::run("pointer-clicks", |ui| {
        ui.name("clicks", "Clicks 0")?;
        ui.pointer_move("button", 0.5, 0.5)?;
        // Clicks queued behind a render that replaces the callback are still delivered.
        ui.pointer_clicks(5)?;
        ui.name("clicks", "Clicks 5")?;
        ui.name_stays("clicks", "Clicks 5", Duration::from_millis(500))
    })
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

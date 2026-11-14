use super::*;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct PointerInput {
    pub pressed: Arc<Mutex<bool>>,
    pub modifiers: Arc<Mutex<Vec<i32>>>,
}

impl PointerInput {
    pub fn release(pressed: &Mutex<bool>) -> Result<()> {
        let mut pressed = pressed.lock().unwrap();
        if *pressed {
            send_mouse(MOUSEEVENTF_LEFTUP, 0, 0)?;
            *pressed = false;
        }
        Ok(())
    }

    pub fn release_modifiers(modifiers: &Mutex<Vec<i32>>) -> Result<()> {
        let mut modifiers = modifiers.lock().unwrap();
        if modifiers.is_empty() {
            return Ok(());
        }
        let mut inputs = modifiers
            .iter()
            .rev()
            .map(|&key| key_input(key, KEYEVENTF_KEYUP as u32))
            .collect::<Vec<_>>();
        let released = unsafe {
            SendInput(
                inputs.len() as u32,
                inputs.as_mut_ptr(),
                size_of::<INPUT>() as i32,
            )
        } as usize;
        let remaining = modifiers.len() - released;
        modifiers.truncate(remaining);
        if remaining != 0 {
            return Err(format!("released {released}/{} wheel modifiers", inputs.len()).into());
        }
        Ok(())
    }
}

impl Drop for PointerInput {
    fn drop(&mut self) {
        // The supervisor owns this guard, even if its UIA worker stops responding.
        if let Err(error) = Self::release(&self.pressed) {
            eprintln!("failed to release injected mouse button: {error}");
        }
        if let Err(error) = Self::release_modifiers(&self.modifiers) {
            eprintln!("failed to release injected wheel modifiers: {error}");
        }
    }
}

pub fn send_mouse(flags: i32, x: i32, y: i32) -> Result<()> {
    let mut input = INPUT {
        r#type: INPUT_MOUSE as u32,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: x,
                dy: y,
                dwFlags: flags as u32,
                ..Default::default()
            },
        },
    };
    if unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) } != 1 {
        return Err("mouse input was not accepted".into());
    }
    Ok(())
}

pub fn key_input(key: i32, flags: u32) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD as u32,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key.try_into().unwrap(),
                dwFlags: flags,
                ..Default::default()
            },
        },
    }
}

pub fn send_key_chord(keys: &[i32]) -> Result<()> {
    let mut inputs = keys
        .iter()
        .map(|&key| key_input(key, 0))
        .chain(
            keys.iter()
                .rev()
                .map(|&key| key_input(key, KEYEVENTF_KEYUP as u32)),
        )
        .collect::<Vec<_>>();
    let inserted = unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_mut_ptr(),
            size_of::<INPUT>() as i32,
        )
    } as usize;
    if inserted != inputs.len() {
        // The accepted prefix contains key-downs followed by reversed key-ups.
        let held = inserted.min(inputs.len() - inserted);
        let mut releases = keys[..held]
            .iter()
            .rev()
            .map(|&key| key_input(key, KEYEVENTF_KEYUP as u32))
            .collect::<Vec<_>>();
        let released = if releases.is_empty() {
            0
        } else {
            unsafe {
                SendInput(
                    releases.len() as u32,
                    releases.as_mut_ptr(),
                    size_of::<INPUT>() as i32,
                )
            }
        };
        return Err(format!(
            "input accepted {inserted}/{} events; released {released}/{held} held keys",
            inputs.len()
        )
        .into());
    }
    Ok(())
}

pub struct PhysicalCoordinates(DPI_AWARENESS_CONTEXT);

impl PhysicalCoordinates {
    pub fn new() -> Result<Self> {
        let previous =
            unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
        if previous.is_null() {
            return Err(windows_core::Error::from_thread().into());
        }
        Ok(Self(previous))
    }
}

impl Drop for PhysicalCoordinates {
    fn drop(&mut self) {
        if unsafe { SetThreadDpiAwarenessContext(self.0) }.is_null() {
            eprintln!(
                "failed to restore thread DPI context: {}",
                windows_core::Error::from_thread()
            );
        }
    }
}

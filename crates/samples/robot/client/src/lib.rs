#![cfg(all(test, windows))]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings;
use bindings::*;
use windows_core::*;

#[test]
fn main() -> Result<()> {
    let robot = Robot::new()?;

    robot.Speak(h!("Hello world"))?;

    let handy_robot = unsafe { CreateRobotFromHandle(123 as _)? };

    handy_robot.Speak(h!("Hello handy"))?;

    let interop: IRobotInterop = handy_robot.cast()?;

    let handle = unsafe { interop.Handle() };

    println!("interop handle: {handle:?}");

    Ok(())
}

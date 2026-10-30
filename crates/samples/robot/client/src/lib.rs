#![cfg(all(test, windows))]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
use bindings::*;
use windows_core::*;

#[test]
fn main() -> Result<()> {
    let robot = Robot::new()?;

    robot.Speak(h!("Hello world"))?;

    let mut handy_robot = None;
    unsafe { CreateRobotFromHandle(123 as _, &mut handy_robot).ok()? };
    let handy_robot = handy_robot.unwrap();

    handy_robot.Speak(h!("Hello handy"))?;

    let interop: IRobotInterop = handy_robot.cast()?;

    let handle = unsafe { interop.Handle() };

    println!("interop handle: {handle:?}");

    Ok(())
}

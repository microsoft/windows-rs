#![cfg(target_env = "msvc")]
#![cfg(test)]

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
use windows::core::*;

unsafe extern "system" {
    fn interop() -> HRESULT;
}

#[test]
fn test() -> Result<()> {
    unsafe { interop().ok()? };

    let activatable = Activatable::new()?;
    assert_eq!(activatable.Property()?, 0);

    let activatable = Activatable::WithValue(123)?;
    assert_eq!(activatable.Property()?, 123);

    let composable = Composable::new()?;
    assert_eq!(composable.Property()?, 0);

    let composable = Composable::WithValue(456)?;
    assert_eq!(composable.Property()?, 456);

    Ok(())
}

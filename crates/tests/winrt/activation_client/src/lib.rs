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
use windows::{
    Win32::IActivationFactory,
    Win32::{E_NOINTERFACE, REGDB_E_CLASSNOTREG},
    core::factory,
};

#[test]
fn instance_class() {
    let instance = Instance::new().unwrap();
    assert_eq!(instance.Property().unwrap(), 123);
}

#[test]
fn missing_class() {
    let error = Missing::new().unwrap_err();
    assert_eq!(error.code(), REGDB_E_CLASSNOTREG);
}

#[test]
fn static_class() {
    let value = Static::Property().unwrap();
    assert_eq!(value, 456);
}

#[test]
fn get_factory() {
    factory::<Instance, IActivationFactory>().unwrap();

    let error = factory::<Instance, IInstance>().unwrap_err();
    assert_eq!(error.code(), E_NOINTERFACE);

    let error = factory::<Missing, IActivationFactory>().unwrap_err();
    assert_eq!(error.code(), REGDB_E_CLASSNOTREG);
}

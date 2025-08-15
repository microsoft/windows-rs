#![cfg(test)]

mod bindings;
use bindings::*;
use windows::{
    core::factory,
    Win32::Foundation::{E_NOINTERFACE, REGDB_E_CLASSNOTREG},
    Win32::System::WinRT::IActivationFactory,
};

// Test of an activation factory with a "default constructor" via `IActivationFactory`.
#[test]
fn instance_class() {
    let instance = Instance::new().unwrap();
    assert_eq!(instance.Property().unwrap(), 123);
}

// Test of a missing activation factory returns the original error returned by `RoGetActivationFactory`.
#[test]
fn missing_class() {
    let error = Missing::new().unwrap_err();
    assert_eq!(error.code(), REGDB_E_CLASSNOTREG);
}

// Test of an activation factory that does not support default construction.
#[test]
fn static_class() {
    let value = Static::Property().unwrap();
    assert_eq!(value, 456);
}

// Test for direct factory construction.
#[test]
fn get_factory() {
    factory::<Instance, IActivationFactory>().unwrap();

    let error = factory::<Instance, IInstance>().unwrap_err();
    assert_eq!(error.code(), E_NOINTERFACE);

    let error = factory::<Missing, IActivationFactory>().unwrap_err();
    assert_eq!(error.code(), REGDB_E_CLASSNOTREG);
}

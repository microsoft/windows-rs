// Compilation is sufficient to test.
// This verifies that it is possible to use #[interface]
// in a crate that uses #![no_std].

#![no_std]

use windows::core::{interface, IUnknown, IUnknown_Vtbl};

#[interface("36bb4e8d-0385-477e-a090-e70675f37781")]
pub unsafe trait IFoo: IUnknown {
    fn x(&self) -> u32;
}

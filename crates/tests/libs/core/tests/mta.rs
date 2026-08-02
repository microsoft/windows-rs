#![cfg(windows)]

use windows_core::*;

#[test]
fn init_mta_test() -> Result<()> {
    init_mta()
}

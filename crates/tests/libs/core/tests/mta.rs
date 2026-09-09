#![cfg(windows)]

use windows_core::*;

#[test]
fn init_mta_test() -> Result<()> {
    init_mta()
}

#[test]
fn init_sta_test() -> Result<()> {
    let _apartment = init_sta()?;
    Ok(())
}

use windows_result::*;

fn checked(code: HRESULT) -> Result<()> {
    code.ok()
}

fn main() -> Result<()> {
    checked(HRESULT(0))?;

    match checked(HRESULT(0x8007_0002u32 as i32)) {
        Ok(()) => unreachable!(),
        Err(error) => println!(
            "recovered from {:#010x}: {}",
            error.code().0,
            error.message()
        ),
    }

    Ok(())
}

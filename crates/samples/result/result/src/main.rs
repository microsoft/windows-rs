use windows_result::*;

fn check(code: HRESULT) -> Result<()> {
    code.ok()
}

fn main() -> Result<()> {
    let cancelled = WIN32_ERROR(1223).to_hresult();
    let error = Error::new(cancelled, "operation cancelled");

    println!("code:    {:#010x}", error.code().0);
    println!("message: {}", error.message());

    check(HRESULT(0))?;

    if let Err(error) = check(HRESULT(0x8007_0002u32 as i32)) {
        println!(
            "recovered from {:#010x}: {}",
            error.code().0,
            error.message()
        );
    }

    Ok(())
}

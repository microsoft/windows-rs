use windows::Win32::System::Diagnostics::Debug::FatalExit;

// Compile-only test to check that FatalExit returns `-> !`

#[expect(dead_code)]
fn test() -> i32 {
    unsafe {
        FatalExit(123);
    }
}

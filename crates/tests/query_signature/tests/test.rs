use windows::Win32::System::Diagnostics::Debug::*;

#[test]
fn test() {
    unsafe {
        let _debug: IDebugClient = DebugCreateEx(0).unwrap();
    }
}

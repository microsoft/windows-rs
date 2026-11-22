windows_core::link!("test.dll" "C" fn Evaluate(données : *mut u8));
pub const TEXT: windows_core::PCWSTR =
    windows_core::w!("macro ! (call ) , ; :: * mut \"quoted\" \\ end");

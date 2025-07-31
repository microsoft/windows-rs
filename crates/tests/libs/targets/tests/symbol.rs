// Tests the generation of functions in the windows and windows-sys crates that use
// alternate import symbol names.
#[test]
fn symbol() {
    unsafe {
        use windows_sys::Win32::Security::Authentication::Identity::RtlGenRandom;
        let mut buffer = [0u8; 8];
        assert!(RtlGenRandom(buffer.as_mut_ptr() as _, buffer.len() as u32));
        assert_ne!(&buffer, &[0u8; 8]);
    }
    unsafe {
        use windows::Win32::Security::Authentication::Identity::RtlGenRandom;
        let mut buffer = [0u8; 8];
        assert!(RtlGenRandom(buffer.as_mut_ptr() as _, buffer.len() as u32),);
        assert_ne!(&buffer, &[0u8; 8]);
    }
}

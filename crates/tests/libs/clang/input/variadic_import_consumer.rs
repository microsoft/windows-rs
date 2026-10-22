#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

#[cfg(not(named_import_probe))]
fn main() {
    check_imports();
}

#[cfg(named_import_probe)]
windows_link::link!("kernel32.dll" "system" fn ExitProcess(code: u32) -> !);

#[cfg(named_import_probe)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { ExitProcess(1) }
}

#[cfg(named_import_probe)]
#[unsafe(no_mangle)]
extern "C" fn mainCRTStartup() -> ! {
    check_imports();
    unsafe { ExitProcess(0) }
}

fn check_imports() {
    let mut narrow = [0u8; 32];
    let mut wide = [0u16; 32];
    let format = b"%u-%s\0".map(u16::from);
    let value = b"ok\0".map(u16::from);
    unsafe {
        assert_eq!(
            wsprintfA(
                narrow.as_mut_ptr().cast(),
                c"%u-%s".as_ptr(),
                17u32,
                c"ok".as_ptr()
            ),
            5
        );
        assert_eq!(&narrow[..6], b"17-ok\0");
        assert_eq!(
            wsprintfW(wide.as_mut_ptr(), format.as_ptr(), 17u32, value.as_ptr()),
            5
        );
        assert_eq!(&wide[..6], &b"17-ok\0".map(u16::from));
        assert_eq!(
            wnsprintfA(narrow.as_mut_ptr().cast(), 32, c"%d".as_ptr(), -42i32),
            3
        );
        assert_eq!(&narrow[..4], b"-42\0");
        let format = b"%d\0".map(u16::from);
        assert_eq!(
            wnsprintfW(wide.as_mut_ptr(), 32, format.as_ptr(), -42i32),
            3
        );
        assert_eq!(&wide[..4], &b"-42\0".map(u16::from));
        let pattern = b"{0,number,0.0}".map(u16::from);
        let mut status = U_ZERO_ERROR;
        assert_eq!(
            u_formatMessage(
                c"en_US".as_ptr(),
                pattern.as_ptr(),
                pattern.len() as i32,
                wide.as_mut_ptr(),
                wide.len() as i32,
                &mut status,
                12.5f64,
            ),
            4
        );
        assert!(status <= U_ZERO_ERROR, "{status}");
        assert_eq!(&wide[..5], &b"12.5\0".map(u16::from));
    }
}

use windows::core::*;

#[test]
fn test() {
    unsafe {
        let s: PCSTR = s!("hello world");
        assert_eq!(s.to_string().unwrap(), "hello world");

        // TODO: https://github.com/microsoft/windows-rs/pull/3004 should enable the following test.
        // let w: PCWSTR = w!("wide world");
        // assert_eq!(w.to_string().unwrap(), "wide world");
    }
}

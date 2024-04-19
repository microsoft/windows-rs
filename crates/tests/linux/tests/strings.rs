use windows::core::*;

#[test]
fn test() {
    unsafe {
        let s: PCSTR = s!("hello world");
        assert_eq!(s.to_string().unwrap(), "hello world");

        let w: PCWSTR = w!("wide world");
        assert_eq!(w.to_string().unwrap(), "wide world");
    }
}

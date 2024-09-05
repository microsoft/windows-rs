use windows::Foundation::*;

#[test]
fn nested() {
    let s = DateTime { UniversalTime: 123 };
    assert_eq!(format!("{s:?}"), "DateTime { UniversalTime: 123 }");
}

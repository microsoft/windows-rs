#![expect(non_snake_case)]

use windows_core::*;

#[interface("7e75ffe0-2f8c-4040-953e-b1f83a48f77b")]
unsafe trait IFirst: IUnknown {
    unsafe fn First(&self) -> i32;
}

#[interface("fe43afb2-43a1-45f9-adbb-1079b410cb9a")]
unsafe trait ISecond: IFirst {
    unsafe fn Second(&self) -> i32;
}

#[interface("4b8c8b47-32dd-4aba-8c68-b0d14703b845")]
unsafe trait IThird: ISecond {
    unsafe fn Third(&self) -> i32;
}

#[implement(IFirst, ISecond, IThird)]
struct Class;

impl IFirst_Impl for Class_Impl {
    unsafe fn First(&self) -> i32 {
        1
    }
}

impl ISecond_Impl for Class_Impl {
    unsafe fn Second(&self) -> i32 {
        2
    }
}

impl IThird_Impl for Class_Impl {
    unsafe fn Third(&self) -> i32 {
        3
    }
}

#[test]
fn test() {
    unsafe {
        let third: IThird = Class.into();
        assert_eq!(third.First(), 1);
        assert_eq!(third.Second(), 2);
        assert_eq!(third.Third(), 3);

        let second: ISecond = third.cast().unwrap();
        assert_eq!(second.First(), 1);
        assert_eq!(second.Second(), 2);

        let first: IFirst = third.cast().unwrap();
        assert_eq!(first.First(), 1);
    }
}

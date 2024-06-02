use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[implement(IStringable)]
struct Test;

impl IStringable_Impl for Test_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Err(Error::new(E_INVALIDARG, "Test message"))
    }
}

#[test]
fn test() {
    let test: IStringable = Test.into();
    let result = test.ToString();
    let error = result.unwrap_err();
    assert_eq!(error.code(), E_INVALIDARG);
    assert_eq!(error.message(), "Test message");
}

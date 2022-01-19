use windows::core::*;
use windows::Win32::Foundation::S_OK;
use windows::UI::Xaml::Markup::{IXamlType, IXamlType2, IXamlType2_Impl};

#[implement(IXamlType2)]
struct Test();

#[allow(non_snake_case)]
impl IXamlType2_Impl for Test {
    fn BoxedType(&self) -> Result<IXamlType> {
        Err(Error::OK)
    }
}

#[test]
fn test() -> Result<()> {
    let a: IXamlType2 = Test().into();

    let b = a.BoxedType();
    assert!(b.is_err());

    let e = b.unwrap_err();
    assert!(e.code() == S_OK);
    assert!(e.info().is_none());

    Ok(())
}

use windows::core::*;
use windows::Win32::Foundation::S_OK;
use windows::UI::Xaml::Interop::*;
use windows::UI::Xaml::Markup::*;

// TODO: we can't infer the namespace for the trait types for a particular interfaces
// because associated traits aren't yet supported by Rust.

// TODO: this doesn't support QI to IXamlType (as it does for COM hierarchies)
#[implement(IXamlType2)]
struct Test();

#[allow(non_snake_case)]
impl IXamlType2_Impl for Test {
    fn BoxedType(&self) -> Result<IXamlType> {
        Err(Error::OK)
    }
}

// Note: IXamlType_Impl is required by IXamlType2_Impl (since IXamlType2 requires IXamlType).
#[allow(non_snake_case)]
impl IXamlType_Impl for Test {
    fn BaseType(&self) -> Result<IXamlType> {
        todo!()
    }
    fn ContentProperty(&self) -> Result<IXamlMember> {
        todo!()
    }
    fn FullName(&self) -> Result<HSTRING> {
        todo!()
    }
    fn IsArray(&self) -> Result<bool> {
        todo!()
    }
    fn IsCollection(&self) -> Result<bool> {
        todo!()
    }
    fn IsConstructible(&self) -> Result<bool> {
        todo!()
    }
    fn IsDictionary(&self) -> Result<bool> {
        todo!()
    }
    fn IsMarkupExtension(&self) -> Result<bool> {
        todo!()
    }
    fn IsBindable(&self) -> Result<bool> {
        todo!()
    }
    fn ItemType(&self) -> Result<IXamlType> {
        todo!()
    }
    fn KeyType(&self) -> Result<IXamlType> {
        todo!()
    }
    fn UnderlyingType(&self) -> Result<TypeName> {
        todo!()
    }
    fn ActivateInstance(&self) -> Result<IInspectable> {
        todo!()
    }
    fn CreateFromString(&self, _: &HSTRING) -> Result<IInspectable> {
        todo!()
    }
    fn GetMember(&self, _: &HSTRING) -> Result<IXamlMember> {
        todo!()
    }
    fn AddToVector(&self, _: &Option<IInspectable>, _: &Option<IInspectable>) -> Result<()> {
        todo!()
    }
    fn AddToMap(&self, _: &Option<IInspectable>, _: &Option<IInspectable>, _: &Option<IInspectable>) -> Result<()> {
        todo!()
    }
    fn RunInitializer(&self) -> Result<()> {
        todo!()
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

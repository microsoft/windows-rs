use windows::core::*;
use windows::Win32::Foundation::S_OK;
use windows::UI::Xaml::Markup::*;
use windows::UI::Xaml::Interop::*;

// TODO: we can't necessarily infer the namespace for the trait types for a particular interfaces
// because associated traits aren't yet supported by Rust. 

// TODO: this doesn't support QI to IXamlType (as it does for COM hierarchies)
#[implement(IXamlType2)]
struct Test();

#[allow(non_snake_case)]
impl IXamlType2_Impl for Test {
    fn BoxedType(&mut self) -> Result<IXamlType> {
        Err(Error::OK)
    }
}

// Note: IXamlType_Impl is required by IXamlType2_Impl (since IXamlType2 requires IXamlType).
#[allow(non_snake_case)]
impl IXamlType_Impl for Test {
fn BaseType(&mut self) -> Result<IXamlType> { todo!() }
fn ContentProperty(&mut self) -> Result<IXamlMember> { todo!() }
fn FullName(&mut self) -> Result<HSTRING> { todo!() }
fn IsArray(&mut self) -> Result<bool> { todo!() }
fn IsCollection(&mut self) -> Result<bool> { todo!() }
fn IsConstructible(&mut self) -> Result<bool> { todo!() }
fn IsDictionary(&mut self) -> Result<bool> { todo!() }
fn IsMarkupExtension(&mut self) -> Result<bool> { todo!() }
fn IsBindable(&mut self) -> Result<bool> { todo!() }
fn ItemType(&mut self) -> Result<IXamlType> { todo!() }
fn KeyType(&mut self) -> Result<IXamlType> { todo!() }
fn UnderlyingType(&mut self) -> Result<TypeName> { todo!() }
fn ActivateInstance(&mut self) -> Result<IInspectable> { todo!() }
fn CreateFromString(&mut self, _: &HSTRING) -> Result<IInspectable> { todo!() }
fn GetMember(&mut self, _: &HSTRING) -> Result<IXamlMember> { todo!() }
fn AddToVector(&mut self, _: &Option<IInspectable>, _: &Option<IInspectable>) -> Result<()> { todo!() }
fn AddToMap(&mut self, _: &Option<IInspectable>, _: &Option<IInspectable>, _: &Option<IInspectable>) -> Result<()> { todo!() }
fn RunInitializer(&mut self) -> Result<()> { todo!() }
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

use windows::*;

#[test]
fn implement_escape() {}

#[::windows::core::implement(UI::Xaml::Data::ICustomPropertyProvider)]
struct BookShu {}

#[allow(non_snake_case)]
impl UI::Xaml::Data::ICustomPropertyProvider_Impl for BookShu {
    fn Type(&mut self) -> ::windows::core::Result<UI::Xaml::Interop::TypeName> {
        panic!();
    }

    fn GetCustomProperty(&mut self, _name: &::windows::core::HSTRING) -> ::windows::core::Result<UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    fn GetIndexedProperty(&mut self, _name: &::windows::core::HSTRING, _type: &UI::Xaml::Interop::TypeName) -> ::windows::core::Result<UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    fn GetStringRepresentation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING> {
        panic!();
    }
}

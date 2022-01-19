use test_winrt::*;

#[test]
fn implement_escape() {}

#[::windows::core::implement(Windows::UI::Xaml::Data::ICustomPropertyProvider)]
struct BookShu {}

#[allow(non_snake_case)]
impl BookShu {
    pub fn Type(&self) -> ::windows::core::Result<Windows::UI::Xaml::Interop::TypeName> {
        panic!();
    }

    pub fn GetCustomProperty(&self, _name: &::windows::core::HSTRING) -> ::windows::core::Result<Windows::UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    pub fn GetIndexedProperty(&self, _name: &::windows::core::HSTRING, _type: &Windows::UI::Xaml::Interop::TypeName) -> ::windows::core::Result<Windows::UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    pub fn GetStringRepresentation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        panic!();
    }
}

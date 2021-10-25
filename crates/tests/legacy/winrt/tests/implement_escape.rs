use test_winrt::*;

#[test]
fn implement_escape() {}

#[::windows::runtime::implement(Windows::UI::Xaml::Data::ICustomPropertyProvider)]
struct BookShu {}

#[allow(non_snake_case)]
impl BookShu {
    pub fn Type(&self) -> ::windows::runtime::Result<Windows::UI::Xaml::Interop::TypeName> {
        panic!();
    }

    pub fn GetCustomProperty(
        &self,
        _name: &::windows::runtime::HSTRING,
    ) -> ::windows::runtime::Result<Windows::UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    pub fn GetIndexedProperty(
        &self,
        _name: &::windows::runtime::HSTRING,
        _type: &Windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows::runtime::Result<Windows::UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    pub fn GetStringRepresentation(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        panic!();
    }
}

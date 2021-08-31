use test_winrt::*;

#[test]
fn implement_escape() {}

#[::windows::implement(Windows::UI::Xaml::Data::ICustomPropertyProvider)]
struct BookShu {}

#[allow(non_snake_case)]
impl BookShu {
    pub fn Type(&self) -> ::windows::Result<Windows::UI::Xaml::Interop::TypeName> {
        panic!();
    }

    pub fn GetCustomProperty(
        &self,
        _name: &::windows::HSTRING,
    ) -> ::windows::Result<Windows::UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    pub fn GetIndexedProperty(
        &self,
        _name: &::windows::HSTRING,
        _type: &Windows::UI::Xaml::Interop::TypeName,
    ) -> ::windows::Result<Windows::UI::Xaml::Data::ICustomProperty> {
        panic!();
    }

    pub fn GetStringRepresentation(&self) -> ::windows::Result<::windows::HSTRING> {
        panic!();
    }
}

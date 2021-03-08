use test_winrt::*;

#[test]
fn implement_escape() {}

#[::windows::implement(windows::ui::xaml::data::ICustomPropertyProvider)]
struct BookShu {
}

impl BookShu {
    pub fn r#type(&self) -> ::windows::Result<windows::ui::xaml::interop::TypeName> {
        panic!();
    }

    pub fn get_custom_property(
        &self,
        _name: &::windows::HString,
    ) -> ::windows::Result<windows::ui::xaml::data::ICustomProperty> {
        panic!();
    }

    pub fn get_indexed_property(
        &self,
        _name: &::windows::HString,
        _type: &windows::ui::xaml::interop::TypeName,
    ) -> ::windows::Result<windows::ui::xaml::data::ICustomProperty> {
        panic!();
    }

    pub fn get_string_representation(&self) -> ::windows::Result<::windows::HString> {
        panic!();
    }
}

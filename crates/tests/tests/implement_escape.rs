use tests::*;

#[test]
fn implement_escape() {}

#[winrt::implement(windows::ui::xaml::data::ICustomPropertyProvider)]
struct BookShu {
    title: String,
}

impl BookShu {
    pub fn r#type(&self) -> winrt::Result<windows::ui::xaml::interop::TypeName> {
        Ok(windows::ui::xaml::interop::TypeName {
            name: "BookShu".into(),
            kind: windows::ui::xaml::interop::TypeKind::Custom,
        })
    }

    pub fn get_custom_property<'a, N: Into<winrt::Param<'a, winrt::HString>>>(
        &self,
        _name: N,
    ) -> winrt::Result<windows::ui::xaml::data::ICustomProperty> {
        Err(winrt::ErrorCode::E_NOINTERFACE.into())
    }

    pub fn get_indexed_property<
        'a,
        T0__: Into<::winrt::Param<'a, ::winrt::HString>>,
        T1__: Into<::winrt::Param<'a, windows::ui::xaml::interop::TypeName>>,
    >(
        &self,
        _name: T0__,
        _type: T1__,
    ) -> winrt::Result<windows::ui::xaml::data::ICustomProperty> {
        Err(winrt::ErrorCode::E_NOINTERFACE.into())
    }

    pub fn get_string_representation(&self) -> winrt::Result<winrt::HString> {
        Ok(format!("BookShu[{}]", self.title).into())
    }
}

use tests::*;

#[test]
fn implement_escape() {}

#[::windows::implement(windows::ui::xaml::data::ICustomPropertyProvider)]
struct BookShu {
    title: String,
}

impl BookShu {
    pub fn r#type(&self) -> windows::Result<windows::ui::xaml::interop::TypeName> {
        Ok(windows::ui::xaml::interop::TypeName {
            name: "BookShu".into(),
            kind: windows::ui::xaml::interop::TypeKind::Custom,
        })
    }

    pub fn get_custom_property<'a, N: Into<windows::Param<'a, windows::HString>>>(
        &self,
        _name: N,
    ) -> windows::Result<windows::ui::xaml::data::ICustomProperty> {
        Err(windows::ErrorCode::E_NOINTERFACE.into())
    }

    pub fn get_indexed_property<
        'a,
        T0__: Into<::windows::Param<'a, ::windows::HString>>,
        T1__: Into<::windows::Param<'a, windows::ui::xaml::interop::TypeName>>,
    >(
        &self,
        _name: T0__,
        _type: T1__,
    ) -> windows::Result<windows::ui::xaml::data::ICustomProperty> {
        Err(windows::ErrorCode::E_NOINTERFACE.into())
    }

    pub fn get_string_representation(&self) -> windows::Result<windows::HString> {
        Ok(format!("BookShu[{}]", self.title).into())
    }
}

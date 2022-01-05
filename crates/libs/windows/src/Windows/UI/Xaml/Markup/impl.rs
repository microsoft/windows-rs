pub trait IComponentConnectorImpl: Sized {
    fn Connect(&self, connectionid: i32, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
pub trait IComponentConnector2Impl: Sized {
    fn GetBindingConnector(&self, connectionid: i32, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IComponentConnector>;
}
pub trait IDataTemplateComponentImpl: Sized {
    fn Recycle(&self) -> ::windows::core::Result<()>;
    fn ProcessBindings(&self, item: &::core::option::Option<::windows::core::IInspectable>, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MarkupExtension>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionOverridesImpl: Sized {
    fn ProvideValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBinaryWriterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBinaryWriterStaticsImpl: Sized {
    fn Write(&self, inputstreams: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, outputstreams: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, xamlmetadataprovider: &::core::option::Option<IXamlMetadataProvider>) -> ::windows::core::Result<XamlBinaryWriterErrorInformation>;
}
pub trait IXamlBindScopeDiagnosticsImpl: Sized {
    fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBindingHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBindingHelperStaticsImpl: Sized {
    fn DataTemplateComponentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDataTemplateComponent(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<IDataTemplateComponent>;
    fn SetDataTemplateComponent(&self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<IDataTemplateComponent>) -> ::windows::core::Result<()>;
    fn SuspendRendering(&self, target: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ResumeRendering(&self, target: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ConvertValue(&self, r#type: &super::Interop::TypeName, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetPropertyFromString(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetPropertyFromBoolean(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: bool) -> ::windows::core::Result<()>;
    fn SetPropertyFromChar16(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u16) -> ::windows::core::Result<()>;
    fn SetPropertyFromDateTime(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetPropertyFromDouble(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: f64) -> ::windows::core::Result<()>;
    fn SetPropertyFromInt32(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: i32) -> ::windows::core::Result<()>;
    fn SetPropertyFromUInt32(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u32) -> ::windows::core::Result<()>;
    fn SetPropertyFromInt64(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: i64) -> ::windows::core::Result<()>;
    fn SetPropertyFromUInt64(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u64) -> ::windows::core::Result<()>;
    fn SetPropertyFromSingle(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: f32) -> ::windows::core::Result<()>;
    fn SetPropertyFromPoint(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetPropertyFromRect(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetPropertyFromSize(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn SetPropertyFromTimeSpan(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetPropertyFromByte(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u8) -> ::windows::core::Result<()>;
    fn SetPropertyFromUri(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetPropertyFromObject(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlMarkupHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlMarkupHelperStaticsImpl: Sized {
    fn UnloadObject(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
}
pub trait IXamlMemberImpl: Sized {
    fn IsAttachable(&self) -> ::windows::core::Result<bool>;
    fn IsDependencyProperty(&self) -> ::windows::core::Result<bool>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TargetType(&self) -> ::windows::core::Result<IXamlType>;
    fn Type(&self) -> ::windows::core::Result<IXamlType>;
    fn GetValue(&self, instance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, instance: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
pub trait IXamlMetadataProviderImpl: Sized {
    fn GetXamlType(&self, r#type: &super::Interop::TypeName) -> ::windows::core::Result<IXamlType>;
    fn GetXamlTypeByFullName(&self, fullname: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlType>;
    fn GetXmlnsDefinitions(&self) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlReaderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlReaderStaticsImpl: Sized {
    fn Load(&self, xaml: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn LoadWithInitialTemplateValidation(&self, xaml: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait IXamlTypeImpl: Sized {
    fn BaseType(&self) -> ::windows::core::Result<IXamlType>;
    fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsArray(&self) -> ::windows::core::Result<bool>;
    fn IsCollection(&self) -> ::windows::core::Result<bool>;
    fn IsConstructible(&self) -> ::windows::core::Result<bool>;
    fn IsDictionary(&self) -> ::windows::core::Result<bool>;
    fn IsMarkupExtension(&self) -> ::windows::core::Result<bool>;
    fn IsBindable(&self) -> ::windows::core::Result<bool>;
    fn ItemType(&self) -> ::windows::core::Result<IXamlType>;
    fn KeyType(&self) -> ::windows::core::Result<IXamlType>;
    fn UnderlyingType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateFromString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetMember(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlMember>;
    fn AddToVector(&self, instance: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AddToMap(&self, instance: &::core::option::Option<::windows::core::IInspectable>, key: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RunInitializer(&self) -> ::windows::core::Result<()>;
}
pub trait IXamlType2Impl: Sized + IXamlTypeImpl {
    fn BoxedType(&self) -> ::windows::core::Result<IXamlType>;
}

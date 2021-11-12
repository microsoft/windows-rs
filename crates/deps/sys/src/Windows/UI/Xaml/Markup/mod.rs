#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IComponentConnector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentConnector {}
impl ::core::clone::Clone for IComponentConnector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComponentConnector2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComponentConnector2 {}
impl ::core::clone::Clone for IComponentConnector2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateComponent {}
impl ::core::clone::Clone for IDataTemplateComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMarkupExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMarkupExtension {}
impl ::core::clone::Clone for IMarkupExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMarkupExtensionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMarkupExtensionFactory {}
impl ::core::clone::Clone for IMarkupExtensionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMarkupExtensionOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMarkupExtensionOverrides {}
impl ::core::clone::Clone for IMarkupExtensionOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlBinaryWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlBinaryWriter {}
impl ::core::clone::Clone for IXamlBinaryWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlBinaryWriterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlBinaryWriterStatics {}
impl ::core::clone::Clone for IXamlBinaryWriterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlBindScopeDiagnostics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlBindScopeDiagnostics {}
impl ::core::clone::Clone for IXamlBindScopeDiagnostics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlBindingHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlBindingHelper {}
impl ::core::clone::Clone for IXamlBindingHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlBindingHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlBindingHelperStatics {}
impl ::core::clone::Clone for IXamlBindingHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlMarkupHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlMarkupHelper {}
impl ::core::clone::Clone for IXamlMarkupHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlMarkupHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlMarkupHelperStatics {}
impl ::core::clone::Clone for IXamlMarkupHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlMember(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlMember {}
impl ::core::clone::Clone for IXamlMember {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlMetadataProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlMetadataProvider {}
impl ::core::clone::Clone for IXamlMetadataProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlReader {}
impl ::core::clone::Clone for IXamlReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlReaderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlReaderStatics {}
impl ::core::clone::Clone for IXamlReaderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlType {}
impl ::core::clone::Clone for IXamlType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlType2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlType2 {}
impl ::core::clone::Clone for IXamlType2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MarkupExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MarkupExtension {}
impl ::core::clone::Clone for MarkupExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlBinaryWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlBinaryWriter {}
impl ::core::clone::Clone for XamlBinaryWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XamlBinaryWriterErrorInformation {
    pub InputStreamIndex: u32,
    pub LineNumber: u32,
    pub LinePosition: u32,
}
impl ::core::marker::Copy for XamlBinaryWriterErrorInformation {}
impl ::core::clone::Clone for XamlBinaryWriterErrorInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlBindingHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlBindingHelper {}
impl ::core::clone::Clone for XamlBindingHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlMarkupHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlMarkupHelper {}
impl ::core::clone::Clone for XamlMarkupHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XamlReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XamlReader {}
impl ::core::clone::Clone for XamlReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct XmlnsDefinition {
    pub XmlNamespace: ::windows_sys::core::HSTRING,
    pub Namespace: ::windows_sys::core::HSTRING,
}
impl ::core::marker::Copy for XmlnsDefinition {}
impl ::core::clone::Clone for XmlnsDefinition {
    fn clone(&self) -> Self {
        *self
    }
}

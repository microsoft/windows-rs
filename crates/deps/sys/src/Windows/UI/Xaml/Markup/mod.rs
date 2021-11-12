#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IComponentConnector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComponentConnector2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateComponent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupExtensionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMarkupExtensionOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlBinaryWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlBinaryWriterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlBindScopeDiagnostics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlBindingHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlBindingHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlMarkupHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlMarkupHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlMember(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlMetadataProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlReaderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlType2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MarkupExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlBinaryWriter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct XamlBinaryWriterErrorInformation(i32);
#[repr(transparent)]
pub struct XamlBindingHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlMarkupHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlReader(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct XmlnsDefinition(i32);

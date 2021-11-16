#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub type BaseValueSource = i32;
pub const BaseValueSourceUnknown: BaseValueSource = 0i32;
pub const BaseValueSourceDefault: BaseValueSource = 1i32;
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2i32;
pub const BaseValueSourceStyle: BaseValueSource = 3i32;
pub const BaseValueSourceLocal: BaseValueSource = 4i32;
pub const Inherited: BaseValueSource = 5i32;
pub const DefaultStyleTrigger: BaseValueSource = 6i32;
pub const TemplateTrigger: BaseValueSource = 7i32;
pub const StyleTrigger: BaseValueSource = 8i32;
pub const ImplicitStyleReference: BaseValueSource = 9i32;
pub const ParentTemplate: BaseValueSource = 10i32;
pub const ParentTemplateTrigger: BaseValueSource = 11i32;
pub const Animation: BaseValueSource = 12i32;
pub const Coercion: BaseValueSource = 13i32;
pub const BaseValueSourceVisualState: BaseValueSource = 14i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub AlphaMode: super::super::super::Graphics::Dxgi::Common::DXGI_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for BitmapDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub MetadataBits: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CollectionElementValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CollectionElementValue {
    fn clone(&self) -> Self {
        *self
    }
}
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = -2144665560i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct EnumType {
    pub Name: super::super::super::Foundation::BSTR,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for EnumType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for EnumType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IBitmapData = *mut ::core::ffi::c_void;
pub type IVisualTreeService = *mut ::core::ffi::c_void;
pub type IVisualTreeService2 = *mut ::core::ffi::c_void;
pub type IVisualTreeService3 = *mut ::core::ffi::c_void;
pub type IVisualTreeServiceCallback = *mut ::core::ffi::c_void;
pub type IVisualTreeServiceCallback2 = *mut ::core::ffi::c_void;
pub type IXamlDiagnostics = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: Self = Self(0i32);
    pub const IsValueHandle: Self = Self(1i32);
    pub const IsPropertyReadOnly: Self = Self(2i32);
    pub const IsValueCollection: Self = Self(4i32);
    pub const IsValueCollectionReadOnly: Self = Self(8i32);
    pub const IsValueBindingExpression: Self = Self(16i32);
    pub const IsValueNull: Self = Self(32i32);
    pub const IsValueHandleAndEvaluatedValue: Self = Self(64i32);
}
impl ::core::marker::Copy for MetadataBit {}
impl ::core::clone::Clone for MetadataBit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
impl ::core::marker::Copy for ParentChildRelation {}
impl ::core::clone::Clone for ParentChildRelation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainSource {
    pub Handle: u64,
    pub TargetType: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PropertyChainSource {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: super::super::super::Foundation::BSTR,
    pub DeclaringType: super::super::super::Foundation::BSTR,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub ItemType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub Overridden: super::super::super::Foundation::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: super::super::super::Foundation::BSTR,
    pub PropertyChainIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PropertyChainValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainValue {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RenderTargetBitmapOptions = i32;
pub const RenderTarget: RenderTargetBitmapOptions = 0i32;
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1i32;
pub type ResourceType = i32;
pub const ResourceTypeStatic: ResourceType = 0i32;
pub const ResourceTypeTheme: ResourceType = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SourceInfo {
    pub FileName: super::super::super::Foundation::BSTR,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SourceInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SourceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub NumChildren: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VisualElement {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VisualElement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type VisualElementState = i32;
pub const ErrorResolved: VisualElementState = 0i32;
pub const ErrorResourceNotFound: VisualElementState = 1i32;
pub const ErrorInvalidResource: VisualElementState = 2i32;
pub type VisualMutationType = i32;
pub const Add: VisualMutationType = 0i32;
pub const Remove: VisualMutationType = 1i32;

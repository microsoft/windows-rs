#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct BaseValueSource(pub i32);
pub const BaseValueSourceUnknown: BaseValueSource = BaseValueSource(0i32);
pub const BaseValueSourceDefault: BaseValueSource = BaseValueSource(1i32);
pub const BaseValueSourceBuiltInStyle: BaseValueSource = BaseValueSource(2i32);
pub const BaseValueSourceStyle: BaseValueSource = BaseValueSource(3i32);
pub const BaseValueSourceLocal: BaseValueSource = BaseValueSource(4i32);
pub const Inherited: BaseValueSource = BaseValueSource(5i32);
pub const DefaultStyleTrigger: BaseValueSource = BaseValueSource(6i32);
pub const TemplateTrigger: BaseValueSource = BaseValueSource(7i32);
pub const StyleTrigger: BaseValueSource = BaseValueSource(8i32);
pub const ImplicitStyleReference: BaseValueSource = BaseValueSource(9i32);
pub const ParentTemplate: BaseValueSource = BaseValueSource(10i32);
pub const ParentTemplateTrigger: BaseValueSource = BaseValueSource(11i32);
pub const Animation: BaseValueSource = BaseValueSource(12i32);
pub const Coercion: BaseValueSource = BaseValueSource(13i32);
pub const BaseValueSourceVisualState: BaseValueSource = BaseValueSource(14i32);
impl ::core::marker::Copy for BaseValueSource {}
impl ::core::clone::Clone for BaseValueSource {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144665560i32 as _);
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
#[repr(transparent)]
pub struct IBitmapData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapData {}
impl ::core::clone::Clone for IBitmapData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeService {}
impl ::core::clone::Clone for IVisualTreeService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeService2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeService2 {}
impl ::core::clone::Clone for IVisualTreeService2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeService3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeService3 {}
impl ::core::clone::Clone for IVisualTreeService3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeServiceCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeServiceCallback {}
impl ::core::clone::Clone for IVisualTreeServiceCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualTreeServiceCallback2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualTreeServiceCallback2 {}
impl ::core::clone::Clone for IVisualTreeServiceCallback2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXamlDiagnostics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXamlDiagnostics {}
impl ::core::clone::Clone for IXamlDiagnostics {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct RenderTargetBitmapOptions(pub i32);
pub const RenderTarget: RenderTargetBitmapOptions = RenderTargetBitmapOptions(0i32);
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = RenderTargetBitmapOptions(1i32);
impl ::core::marker::Copy for RenderTargetBitmapOptions {}
impl ::core::clone::Clone for RenderTargetBitmapOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceType(pub i32);
pub const ResourceTypeStatic: ResourceType = ResourceType(0i32);
pub const ResourceTypeTheme: ResourceType = ResourceType(1i32);
impl ::core::marker::Copy for ResourceType {}
impl ::core::clone::Clone for ResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct VisualElementState(pub i32);
pub const ErrorResolved: VisualElementState = VisualElementState(0i32);
pub const ErrorResourceNotFound: VisualElementState = VisualElementState(1i32);
pub const ErrorInvalidResource: VisualElementState = VisualElementState(2i32);
impl ::core::marker::Copy for VisualElementState {}
impl ::core::clone::Clone for VisualElementState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualMutationType(pub i32);
pub const Add: VisualMutationType = VisualMutationType(0i32);
pub const Remove: VisualMutationType = VisualMutationType(1i32);
impl ::core::marker::Copy for VisualMutationType {}
impl ::core::clone::Clone for VisualMutationType {
    fn clone(&self) -> Self {
        *self
    }
}

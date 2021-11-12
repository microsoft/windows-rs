#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[repr(C)]
pub struct BitmapDescription(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CollectionElementValue(i32);
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144665560i32 as _);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct EnumType(i32);
#[repr(transparent)]
pub struct IBitmapData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeService3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeServiceCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualTreeServiceCallback2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlDiagnostics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: MetadataBit = MetadataBit(0i32);
    pub const IsValueHandle: MetadataBit = MetadataBit(1i32);
    pub const IsPropertyReadOnly: MetadataBit = MetadataBit(2i32);
    pub const IsValueCollection: MetadataBit = MetadataBit(4i32);
    pub const IsValueCollectionReadOnly: MetadataBit = MetadataBit(8i32);
    pub const IsValueBindingExpression: MetadataBit = MetadataBit(16i32);
    pub const IsValueNull: MetadataBit = MetadataBit(32i32);
    pub const IsValueHandleAndEvaluatedValue: MetadataBit = MetadataBit(64i32);
}
#[repr(C)]
pub struct ParentChildRelation(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PropertyChainSource(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PropertyChainValue(i32);
#[repr(transparent)]
pub struct RenderTargetBitmapOptions(pub i32);
pub const RenderTarget: RenderTargetBitmapOptions = RenderTargetBitmapOptions(0i32);
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = RenderTargetBitmapOptions(1i32);
#[repr(transparent)]
pub struct ResourceType(pub i32);
pub const ResourceTypeStatic: ResourceType = ResourceType(0i32);
pub const ResourceTypeTheme: ResourceType = ResourceType(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SourceInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VisualElement(i32);
#[repr(transparent)]
pub struct VisualElementState(pub i32);
pub const ErrorResolved: VisualElementState = VisualElementState(0i32);
pub const ErrorResourceNotFound: VisualElementState = VisualElementState(1i32);
pub const ErrorInvalidResource: VisualElementState = VisualElementState(2i32);
#[repr(transparent)]
pub struct VisualMutationType(pub i32);
pub const Add: VisualMutationType = VisualMutationType(0i32);
pub const Remove: VisualMutationType = VisualMutationType(1i32);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub struct BaseValueSource(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct BitmapDescription(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CollectionElementValue(i32);
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144665560i32 as _);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
pub struct MetadataBit(i32);
pub struct ParentChildRelation(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainSource(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainValue(i32);
pub struct RenderTargetBitmapOptions(i32);
pub struct ResourceType(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct SourceInfo(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct VisualElement(i32);
pub struct VisualElementState(i32);
pub struct VisualMutationType(i32);

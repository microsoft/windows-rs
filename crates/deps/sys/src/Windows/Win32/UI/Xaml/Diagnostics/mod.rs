#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct BaseValueSource(i32);
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
#[repr(C)]
pub struct MetadataBit(i32);
#[repr(C)]
pub struct ParentChildRelation(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PropertyChainSource(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PropertyChainValue(i32);
#[repr(C)]
pub struct RenderTargetBitmapOptions(i32);
#[repr(C)]
pub struct ResourceType(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SourceInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct VisualElement(i32);
#[repr(C)]
pub struct VisualElementState(i32);
#[repr(C)]
pub struct VisualMutationType(i32);

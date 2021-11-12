#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows_sys::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub struct BaseValueSource(i32);
pub struct BitmapDescription(i32);
pub struct CollectionElementValue(i32);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
pub const E_UNKNOWNTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144665560i32 as _);
pub struct EnumType(i32);
pub struct IBitmapData(i32);
pub struct IVisualTreeService(i32);
pub struct IVisualTreeService2(i32);
pub struct IVisualTreeService3(i32);
pub struct IVisualTreeServiceCallback(i32);
pub struct IVisualTreeServiceCallback2(i32);
pub struct IXamlDiagnostics(i32);
pub struct MetadataBit(i32);
pub struct ParentChildRelation(i32);
pub struct PropertyChainSource(i32);
pub struct PropertyChainValue(i32);
pub struct RenderTargetBitmapOptions(i32);
pub struct ResourceType(i32);
pub struct SourceInfo(i32);
pub struct VisualElement(i32);
pub struct VisualElementState(i32);
pub struct VisualMutationType(i32);

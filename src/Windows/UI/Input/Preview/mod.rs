#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputActivationListenerPreviewStatics {
    type Vtable = IInputActivationListenerPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4032109797, 3558, 23520, [165, 137, 247, 55, 32, 26, 69, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
#[doc = "*Required features: `UI_Input_Preview`*"]
pub struct InputActivationListenerPreview {}
impl InputActivationListenerPreview {
    #[cfg(feature = "UI_WindowManagement")]
    #[doc = "*Required features: `UI_Input_Preview`, `UI_WindowManagement`*"]
    pub fn CreateForApplicationWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows::runtime::Result<super::InputActivationListener> {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<super::InputActivationListener>(result__)
        })
    }
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}

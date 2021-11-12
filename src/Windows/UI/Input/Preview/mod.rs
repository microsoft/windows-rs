#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[repr(transparent)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInputActivationListenerPreviewStatics {
    type Vtable = IInputActivationListenerPreviewStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0551ce5_0de6_5be0_a589_f737201a4582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_WindowManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))] usize,
);
pub struct InputActivationListenerPreview {}
impl InputActivationListenerPreview {
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateForApplicationWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows::core::Result<super::InputActivationListener> {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<super::InputActivationListener>(result__)
        })
    }
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}

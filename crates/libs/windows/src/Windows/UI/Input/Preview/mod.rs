#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListenerPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputActivationListenerPreviewStatics {
    type Vtable = IInputActivationListenerPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0551ce5_0de6_5be0_a589_f737201a4582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateForApplicationWindow: usize,
}
#[doc = "*Required features: `\"UI_Input_Preview\"`*"]
pub struct InputActivationListenerPreview {}
impl InputActivationListenerPreview {
    #[doc = "*Required features: `\"UI_Input_Preview\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateForApplicationWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::WindowManagement::AppWindow>>(window: Param0) -> ::windows::core::Result<super::InputActivationListener> {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForApplicationWindow)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<super::InputActivationListener>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

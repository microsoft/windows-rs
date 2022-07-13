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
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateForApplicationWindow: usize,
}
#[doc = "*Required features: `\"UI_Input_Preview\"`*"]
pub struct InputActivationListenerPreview;
impl InputActivationListenerPreview {
    #[doc = "*Required features: `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateForApplicationWindow<'a, P0>(window: P0) -> ::windows::core::Result<super::InputActivationListener>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::WindowManagement::AppWindow>>,
    {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateForApplicationWindow)(::windows::core::Interface::as_raw(this), window.into().abi(), result__.as_mut_ptr()).from_abi::<super::InputActivationListener>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

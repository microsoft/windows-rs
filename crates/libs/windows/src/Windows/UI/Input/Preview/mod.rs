#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListenerPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputActivationListenerPreviewStatics {
    type Vtable = IInputActivationListenerPreviewStatics_Vtbl;
}
impl ::core::clone::Clone for IInputActivationListenerPreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInputActivationListenerPreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0551ce5_0de6_5be0_a589_f737201a4582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
    pub fn CreateForApplicationWindow(window: &super::super::WindowManagement::AppWindow) -> ::windows::core::Result<super::InputActivationListener> {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::InputActivationListener>();
            (::windows::core::Interface::vtable(this).CreateForApplicationWindow)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(window), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

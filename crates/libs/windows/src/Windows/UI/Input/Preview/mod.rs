#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputActivationListenerPreviewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInputActivationListenerPreviewStatics {
    type Vtable = IInputActivationListenerPreviewStatics_Vtbl;
}
impl ::core::clone::Clone for IInputActivationListenerPreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInputActivationListenerPreviewStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0551ce5_0de6_5be0_a589_f737201a4582);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerPreviewStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateForApplicationWindow: usize,
}
#[doc = "*Required features: `\"UI_Input_Preview\"`*"]
pub struct InputActivationListenerPreview;
impl InputActivationListenerPreview {
    #[doc = "*Required features: `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateForApplicationWindow<P0>(window: P0) -> ::windows_core::Result<super::InputActivationListener>
    where
        P0: ::windows_core::IntoParam<super::super::WindowManagement::AppWindow>,
    {
        Self::IInputActivationListenerPreviewStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForApplicationWindow)(::windows_core::Interface::as_raw(this), window.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputActivationListenerPreviewStatics<R, F: FnOnce(&IInputActivationListenerPreviewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InputActivationListenerPreview, IInputActivationListenerPreviewStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}

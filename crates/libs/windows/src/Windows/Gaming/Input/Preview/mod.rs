#[doc = "*Required features: `\"Gaming_Input_Preview\"`*"]
pub struct GameControllerProviderInfo;
impl GameControllerProviderInfo {
    #[doc = "*Required features: `\"Gaming_Input_Preview\"`, `\"Gaming_Input_Custom\"`*"]
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetParentProviderId<'a, Param0: ::windows::core::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GetParentProviderId)(::windows::core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Gaming_Input_Preview\"`, `\"Gaming_Input_Custom\"`*"]
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetProviderId<'a, Param0: ::windows::core::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GetProviderId)(::windows::core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerProviderInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameControllerProviderInfoStatics {
    type Vtable = IGameControllerProviderInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be1e6c5_d9bd_44ee_8362_488b2e464bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetParentProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetParentProviderId: usize,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetProviderId: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

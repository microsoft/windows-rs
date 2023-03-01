#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerProviderInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameControllerProviderInfoStatics {
    type Vtable = IGameControllerProviderInfoStatics_Vtbl;
}
impl ::core::clone::Clone for IGameControllerProviderInfoStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGameControllerProviderInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be1e6c5_d9bd_44ee_8362_488b2e464bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetParentProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetParentProviderId: usize,
    #[cfg(feature = "Gaming_Input_Custom")]
    pub GetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))]
    GetProviderId: usize,
}
#[doc = "*Required features: `\"Gaming_Input_Preview\"`*"]
pub struct GameControllerProviderInfo;
impl GameControllerProviderInfo {
    #[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetParentProviderId<P0>(provider: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::windows::core::TryIntoParam<super::Custom::IGameControllerProvider>,
    {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetParentProviderId)(::windows::core::Interface::as_raw(this), provider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Gaming_Input_Custom\"`*"]
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetProviderId<P0>(provider: P0) -> ::windows::core::Result<::windows::core::HSTRING>
    where
        P0: ::windows::core::TryIntoParam<super::Custom::IGameControllerProvider>,
    {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetProviderId)(::windows::core::Interface::as_raw(this), provider.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

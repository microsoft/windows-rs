#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct GameControllerProviderInfo {}
impl GameControllerProviderInfo {
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetParentProviderId<'a, Param0: ::windows::core::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Gaming_Input_Custom")]
    pub fn GetProviderId<'a, Param0: ::windows::core::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGameControllerProviderInfoStatics {
    type Vtable = IGameControllerProviderInfoStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0be1e6c5_d9bd_44ee_8362_488b2e464bfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Gaming_Input_Custom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provider: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))] usize,
    #[cfg(feature = "Gaming_Input_Custom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, provider: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))] usize,
);

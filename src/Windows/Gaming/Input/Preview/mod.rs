#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Gaming_Input_Preview`*"]
pub struct GameControllerProviderInfo {}
impl GameControllerProviderInfo {
    #[cfg(feature = "Gaming_Input_Custom")]
    #[doc = "*Required features: `Gaming_Input_Preview`, `Gaming_Input_Custom`*"]
    pub fn GetParentProviderId<'a, Param0: ::windows::runtime::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Gaming_Input_Custom")]
    #[doc = "*Required features: `Gaming_Input_Preview`, `Gaming_Input_Custom`*"]
    pub fn GetProviderId<'a, Param0: ::windows::runtime::IntoParam<'a, super::Custom::IGameControllerProvider>>(provider: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IGameControllerProviderInfoStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IGameControllerProviderInfoStatics<R, F: FnOnce(&IGameControllerProviderInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GameControllerProviderInfo, IGameControllerProviderInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GameControllerProviderInfo {
    const NAME: &'static str = "Windows.Gaming.Input.Preview.GameControllerProviderInfo";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGameControllerProviderInfoStatics {
    type Vtable = IGameControllerProviderInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(199354053, 55741, 17646, [131, 98, 72, 139, 46, 70, 75, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProviderInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Gaming_Input_Custom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))] usize,
    #[cfg(feature = "Gaming_Input_Custom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Gaming_Input_Custom"))] usize,
);

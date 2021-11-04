#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_UserActivities_Core`*"]
pub struct CoreUserActivityManager {}
impl CoreUserActivityManager {
    #[doc = "*Required features: `ApplicationModel_UserActivities_Core`*"]
    pub fn CreateUserActivitySessionInBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::UserActivity>>(activity: Param0) -> ::windows::runtime::Result<super::UserActivitySession> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activity.into_param().abi(), &mut result__).from_abi::<super::UserActivitySession>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserActivities_Core`, `Foundation`*"]
    pub fn DeleteUserActivitySessionsInTimeRangeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::UserActivityChannel>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>>(channel: Param0, starttime: Param1, endtime: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), channel.into_param().abi(), starttime.into_param().abi(), endtime.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ICoreUserActivityManagerStatics<R, F: FnOnce(&ICoreUserActivityManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreUserActivityManager, ICoreUserActivityManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICoreUserActivityManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreUserActivityManagerStatics {
    type Vtable = ICoreUserActivityManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3392854786, 42174, 19789, [191, 168, 103, 149, 244, 38, 78, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreUserActivityManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activity: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channel: ::windows::runtime::RawPtr, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);

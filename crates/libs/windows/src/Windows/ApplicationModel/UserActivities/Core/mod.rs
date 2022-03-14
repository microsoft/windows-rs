#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_UserActivities_Core\"`*"]
pub struct CoreUserActivityManager {}
impl CoreUserActivityManager {
    #[doc = "*Required features: `\"ApplicationModel_UserActivities_Core\"`*"]
    pub fn CreateUserActivitySessionInBackground<'a, Param0: ::windows::core::IntoParam<'a, super::UserActivity>>(activity: Param0) -> ::windows::core::Result<super::UserActivitySession> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateUserActivitySessionInBackground)(::core::mem::transmute_copy(this), activity.into_param().abi(), &mut result__).from_abi::<super::UserActivitySession>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserActivities_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteUserActivitySessionsInTimeRangeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::UserActivityChannel>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(channel: Param0, starttime: Param1, endtime: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteUserActivitySessionsInTimeRangeAsync)(::core::mem::transmute_copy(this), channel.into_param().abi(), starttime.into_param().abi(), endtime.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreUserActivityManagerStatics<R, F: FnOnce(&ICoreUserActivityManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CoreUserActivityManager, ICoreUserActivityManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreUserActivityManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICoreUserActivityManagerStatics {
    type Vtable = ICoreUserActivityManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3adb02_a4be_4d4d_bfa8_6795f4264efb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreUserActivityManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteUserActivitySessionsInTimeRangeAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

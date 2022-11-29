#[doc(hidden)]
#[repr(transparent)]
pub struct ICoreUserActivityManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICoreUserActivityManagerStatics {
    type Vtable = ICoreUserActivityManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICoreUserActivityManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3adb02_a4be_4d4d_bfa8_6795f4264efb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreUserActivityManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activity: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteUserActivitySessionsInTimeRangeAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserActivities_Core\"`*"]
pub struct CoreUserActivityManager;
impl CoreUserActivityManager {
    pub fn CreateUserActivitySessionInBackground(activity: &super::UserActivity) -> ::windows::core::Result<super::UserActivitySession> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUserActivitySessionInBackground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activity), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteUserActivitySessionsInTimeRangeAsync(channel: &super::UserActivityChannel, starttime: super::super::super::Foundation::DateTime, endtime: super::super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ICoreUserActivityManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteUserActivitySessionsInTimeRangeAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(channel), starttime, endtime, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICoreUserActivityManagerStatics<R, F: FnOnce(&ICoreUserActivityManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CoreUserActivityManager, ICoreUserActivityManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

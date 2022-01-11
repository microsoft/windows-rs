#[cfg(feature = "Win32_Foundation")]
pub trait INotificationActivationCallbackImpl: Sized {
    fn Activate();
}
#[cfg(feature = "Win32_Foundation")]
impl INotificationActivationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationActivationCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationActivationCallbackVtbl {
        unsafe extern "system" fn Activate<Impl: INotificationActivationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: super::super::Foundation::PWSTR, invokedargs: super::super::Foundation::PWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Activate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationActivationCallback as ::windows::core::Interface>::IID
    }
}

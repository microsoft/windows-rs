#[cfg(feature = "Win32_Foundation")]
pub trait INotificationActivationCallback_Impl: Sized {
    fn Activate(&mut self, appusermodelid: super::super::Foundation::PWSTR, invokedargs: super::super::Foundation::PWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INotificationActivationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationActivationCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INotificationActivationCallback_Vtbl {
        unsafe extern "system" fn Activate<Impl: INotificationActivationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: super::super::Foundation::PWSTR, invokedargs: super::super::Foundation::PWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&appusermodelid), ::core::mem::transmute_copy(&invokedargs), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Activate: Activate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationActivationCallback as ::windows::core::Interface>::IID
    }
}

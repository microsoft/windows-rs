pub trait INotificationActivationCallback_Impl: Sized {
    fn Activate(&self, appusermodelid: &::windows::core::PCWSTR, invokedargs: &::windows::core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INotificationActivationCallback {}
impl INotificationActivationCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INotificationActivationCallback_Impl, const OFFSET: isize>() -> INotificationActivationCallback_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INotificationActivationCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: ::windows::core::PCWSTR, invokedargs: ::windows::core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute(&appusermodelid), ::core::mem::transmute(&invokedargs), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INotificationActivationCallback as ::windows::core::Interface>::IID
    }
}

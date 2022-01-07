pub trait INotificationActivationCallbackImpl: Sized {
    fn Activate();
}
impl ::windows::core::RuntimeName for INotificationActivationCallback {
    const NAME: &'static str = "Windows.Win32.UI.Notifications.INotificationActivationCallback";
}
impl INotificationActivationCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INotificationActivationCallbackImpl, const OFFSET: isize>() -> INotificationActivationCallbackVtbl {
        unsafe extern "system" fn Activate<Impl: INotificationActivationCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: super::super::Foundation::PWSTR, invokedargs: super::super::Foundation::PWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(
                &*(&appusermodelid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&invokedargs as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&data as *const <NOTIFICATION_USER_INPUT_DATA as ::windows::core::Abi>::Abi as *const <NOTIFICATION_USER_INPUT_DATA as ::windows::core::DefaultType>::DefaultType),
                count,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INotificationActivationCallback>, ::windows::core::GetTrustLevel, Activate::<Impl, OFFSET>)
    }
}

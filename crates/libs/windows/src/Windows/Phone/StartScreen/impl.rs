#[cfg(feature = "UI_Notifications")]
pub trait IToastNotificationManagerStatics3_Impl: Sized {
    fn CreateToastNotifierForSecondaryTile(&mut self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(feature = "UI_Notifications")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics3 {
    const NAME: &'static str = "Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
}
#[cfg(feature = "UI_Notifications")]
impl IToastNotificationManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>() -> IToastNotificationManagerStatics3_Vtbl {
        unsafe extern "system" fn CreateToastNotifierForSecondaryTile<Identity: ::windows::core::IUnknownImpl, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateToastNotifierForSecondaryTile(&*(&tileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToastNotificationManagerStatics3, OFFSET>(),
            CreateToastNotifierForSecondaryTile: CreateToastNotifierForSecondaryTile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics3 as ::windows::core::Interface>::IID
    }
}

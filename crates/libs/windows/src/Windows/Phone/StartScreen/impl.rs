#[doc = "*Required features: `\"Phone_StartScreen\"`, `\"UI_Notifications\"`, `\"implement\"`*"]
#[cfg(feature = "UI_Notifications")]
pub trait IToastNotificationManagerStatics3_Impl: Sized {
    fn CreateToastNotifierForSecondaryTile(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotifier>;
}
#[cfg(feature = "UI_Notifications")]
impl ::windows::core::RuntimeName for IToastNotificationManagerStatics3 {
    const NAME: &'static str = "Windows.Phone.StartScreen.IToastNotificationManagerStatics3";
}
#[cfg(feature = "UI_Notifications")]
impl IToastNotificationManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>() -> IToastNotificationManagerStatics3_Vtbl {
        unsafe extern "system" fn CreateToastNotifierForSecondaryTile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IToastNotificationManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateToastNotifierForSecondaryTile(::core::mem::transmute(&tileid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IToastNotificationManagerStatics3, OFFSET>(),
            CreateToastNotifierForSecondaryTile: CreateToastNotifierForSecondaryTile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToastNotificationManagerStatics3 as ::windows::core::ComInterface>::IID
    }
}

#[cfg(feature = "Foundation_Collections")]
pub trait IAdaptiveNotificationContent_Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AdaptiveNotificationContentKind>;
    fn Hints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAdaptiveNotificationContent {
    const NAME: &'static str = "Windows.UI.Notifications.IAdaptiveNotificationContent";
}
#[cfg(feature = "Foundation_Collections")]
impl IAdaptiveNotificationContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: isize>() -> IAdaptiveNotificationContent_Vtbl {
        unsafe extern "system" fn Kind<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hints<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveNotificationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Hints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveNotificationContent, OFFSET>(),
            Kind: Kind::<Identity, Impl, OFFSET>,
            Hints: Hints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveNotificationContent as ::windows::core::Interface>::IID
    }
}

#[cfg(feature = "Foundation")]
pub trait IAccessoryNotificationTriggerDetails_Impl: Sized {
    fn TimeCreated(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessoryNotificationType(&mut self) -> ::windows::core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&mut self) -> ::windows::core::Result<bool>;
    fn SetStartedProcessing(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAccessoryNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryNotificationTriggerDetails";
}
#[cfg(feature = "Foundation")]
impl IAccessoryNotificationTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessoryNotificationTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessoryNotificationTriggerDetails_Vtbl {
        unsafe extern "system" fn TimeCreated<Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessoryNotificationType<Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessoryNotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartedProcessing<Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartedProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartedProcessing<Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartedProcessing(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessoryNotificationTriggerDetails, BASE_OFFSET>(),
            TimeCreated: TimeCreated::<Impl, IMPL_OFFSET>,
            AppDisplayName: AppDisplayName::<Impl, IMPL_OFFSET>,
            AppId: AppId::<Impl, IMPL_OFFSET>,
            AccessoryNotificationType: AccessoryNotificationType::<Impl, IMPL_OFFSET>,
            StartedProcessing: StartedProcessing::<Impl, IMPL_OFFSET>,
            SetStartedProcessing: SetStartedProcessing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessoryNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}

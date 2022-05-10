#[cfg(feature = "Foundation")]
pub trait IAccessoryNotificationTriggerDetails_Impl: Sized {
    fn TimeCreated(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessoryNotificationType(&self) -> ::windows::core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&self) -> ::windows::core::Result<bool>;
    fn SetStartedProcessing(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IAccessoryNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryNotificationTriggerDetails";
}
#[cfg(feature = "Foundation")]
impl IAccessoryNotificationTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>() -> IAccessoryNotificationTriggerDetails_Vtbl {
        unsafe extern "system" fn TimeCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TimeCreated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessoryNotificationType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessoryNotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartedProcessing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartedProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartedProcessing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartedProcessing(value).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IAccessoryNotificationTriggerDetails, OFFSET>(),
            TimeCreated: TimeCreated::<Identity, Impl, OFFSET>,
            AppDisplayName: AppDisplayName::<Identity, Impl, OFFSET>,
            AppId: AppId::<Identity, Impl, OFFSET>,
            AccessoryNotificationType: AccessoryNotificationType::<Identity, Impl, OFFSET>,
            StartedProcessing: StartedProcessing::<Identity, Impl, OFFSET>,
            SetStartedProcessing: SetStartedProcessing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessoryNotificationTriggerDetails as ::windows::core::Interface>::IID
    }
}

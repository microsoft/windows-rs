#[doc = "*Required features: `\"Phone_Notification_Management\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IAccessoryNotificationTriggerDetails_Impl: Sized {
    fn TimeCreated(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime>;
    fn AppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AccessoryNotificationType(&self) -> ::windows_core::Result<AccessoryNotificationType>;
    fn StartedProcessing(&self) -> ::windows_core::Result<bool>;
    fn SetStartedProcessing(&self, value: bool) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IAccessoryNotificationTriggerDetails {
    const NAME: &'static str = "Windows.Phone.Notification.Management.IAccessoryNotificationTriggerDetails";
}
#[cfg(feature = "Foundation")]
impl IAccessoryNotificationTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>() -> IAccessoryNotificationTriggerDetails_Vtbl {
        unsafe extern "system" fn TimeCreated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TimeCreated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessoryNotificationType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AccessoryNotificationType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessoryNotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartedProcessing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartedProcessing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartedProcessing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAccessoryNotificationTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartedProcessing(value).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IAccessoryNotificationTriggerDetails, OFFSET>(),
            TimeCreated: TimeCreated::<Identity, Impl, OFFSET>,
            AppDisplayName: AppDisplayName::<Identity, Impl, OFFSET>,
            AppId: AppId::<Identity, Impl, OFFSET>,
            AccessoryNotificationType: AccessoryNotificationType::<Identity, Impl, OFFSET>,
            StartedProcessing: StartedProcessing::<Identity, Impl, OFFSET>,
            SetStartedProcessing: SetStartedProcessing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAccessoryNotificationTriggerDetails as ::windows_core::ComInterface>::IID
    }
}

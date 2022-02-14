#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdates_Impl: Sized + super::Com::IDispatch_Impl {
    fn DetectNow(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn ShowSettingsDialog(&self) -> ::windows::core::Result<()>;
    fn Settings(&self) -> ::windows::core::Result<IAutomaticUpdatesSettings>;
    fn ServiceEnabled(&self) -> ::windows::core::Result<i16>;
    fn EnableService(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>() -> IAutomaticUpdates_Vtbl {
        unsafe extern "system" fn DetectNow<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DetectNow().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ShowSettingsDialog<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowSettingsDialog().into()
        }
        unsafe extern "system" fn Settings<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableService<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableService().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DetectNow: DetectNow::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            ShowSettingsDialog: ShowSettingsDialog::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            ServiceEnabled: ServiceEnabled::<Identity, Impl, OFFSET>,
            EnableService: EnableService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdates as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdates2_Impl: Sized + super::Com::IDispatch_Impl + IAutomaticUpdates_Impl {
    fn Results(&self) -> ::windows::core::Result<IAutomaticUpdatesResults>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdates2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates2_Impl, const OFFSET: isize>() -> IAutomaticUpdates2_Vtbl {
        unsafe extern "system" fn Results<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAutomaticUpdates_Vtbl::new::<Identity, Impl, OFFSET>(), Results: Results::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdates2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAutomaticUpdates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesResults_Impl: Sized + super::Com::IDispatch_Impl {
    fn LastSearchSuccessDate(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn LastInstallationSuccessDate(&self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>() -> IAutomaticUpdatesResults_Vtbl {
        unsafe extern "system" fn LastSearchSuccessDate<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastSearchSuccessDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastInstallationSuccessDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LastSearchSuccessDate: LastSearchSuccessDate::<Identity, Impl, OFFSET>,
            LastInstallationSuccessDate: LastInstallationSuccessDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesResults as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn NotificationLevel(&self) -> ::windows::core::Result<AutomaticUpdatesNotificationLevel>;
    fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> ::windows::core::Result<()>;
    fn ReadOnly(&self) -> ::windows::core::Result<i16>;
    fn Required(&self) -> ::windows::core::Result<i16>;
    fn ScheduledInstallationDay(&self) -> ::windows::core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::Result<()>;
    fn ScheduledInstallationTime(&self) -> ::windows::core::Result<i32>;
    fn SetScheduledInstallationTime(&self, value: i32) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Save(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>() -> IAutomaticUpdatesSettings_Vtbl {
        unsafe extern "system" fn NotificationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NotificationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationLevel<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotificationLevel(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Required<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Required() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledInstallationDay<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScheduledInstallationDay() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScheduledInstallationDay(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ScheduledInstallationTime<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScheduledInstallationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScheduledInstallationTime(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NotificationLevel: NotificationLevel::<Identity, Impl, OFFSET>,
            SetNotificationLevel: SetNotificationLevel::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Required: Required::<Identity, Impl, OFFSET>,
            ScheduledInstallationDay: ScheduledInstallationDay::<Identity, Impl, OFFSET>,
            SetScheduledInstallationDay: SetScheduledInstallationDay::<Identity, Impl, OFFSET>,
            ScheduledInstallationTime: ScheduledInstallationTime::<Identity, Impl, OFFSET>,
            SetScheduledInstallationTime: SetScheduledInstallationTime::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesSettings2_Impl: Sized + super::Com::IDispatch_Impl + IAutomaticUpdatesSettings_Impl {
    fn IncludeRecommendedUpdates(&self) -> ::windows::core::Result<i16>;
    fn SetIncludeRecommendedUpdates(&self, value: i16) -> ::windows::core::Result<()>;
    fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>() -> IAutomaticUpdatesSettings2_Vtbl {
        unsafe extern "system" fn IncludeRecommendedUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IncludeRecommendedUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIncludeRecommendedUpdates(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CheckPermission<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckPermission(::core::mem::transmute_copy(&usertype), ::core::mem::transmute_copy(&permissiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *userhaspermission = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAutomaticUpdatesSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            IncludeRecommendedUpdates: IncludeRecommendedUpdates::<Identity, Impl, OFFSET>,
            SetIncludeRecommendedUpdates: SetIncludeRecommendedUpdates::<Identity, Impl, OFFSET>,
            CheckPermission: CheckPermission::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesSettings3_Impl: Sized + super::Com::IDispatch_Impl + IAutomaticUpdatesSettings_Impl + IAutomaticUpdatesSettings2_Impl {
    fn NonAdministratorsElevated(&self) -> ::windows::core::Result<i16>;
    fn SetNonAdministratorsElevated(&self, value: i16) -> ::windows::core::Result<()>;
    fn FeaturedUpdatesEnabled(&self) -> ::windows::core::Result<i16>;
    fn SetFeaturedUpdatesEnabled(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesSettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>() -> IAutomaticUpdatesSettings3_Vtbl {
        unsafe extern "system" fn NonAdministratorsElevated<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NonAdministratorsElevated() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNonAdministratorsElevated(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FeaturedUpdatesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFeaturedUpdatesEnabled(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IAutomaticUpdatesSettings2_Vtbl::new::<Identity, Impl, OFFSET>(),
            NonAdministratorsElevated: NonAdministratorsElevated::<Identity, Impl, OFFSET>,
            SetNonAdministratorsElevated: SetNonAdministratorsElevated::<Identity, Impl, OFFSET>,
            FeaturedUpdatesEnabled: FeaturedUpdatesEnabled::<Identity, Impl, OFFSET>,
            SetFeaturedUpdatesEnabled: SetFeaturedUpdatesEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAutomaticUpdatesSettings as ::windows::core::Interface>::IID || iid == &<IAutomaticUpdatesSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICategory_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CategoryID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Children(&self) -> ::windows::core::Result<ICategoryCollection>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Image(&self) -> ::windows::core::Result<IImageInformation>;
    fn Order(&self) -> ::windows::core::Result<i32>;
    fn Parent(&self) -> ::windows::core::Result<ICategory>;
    fn Type(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Updates(&self) -> ::windows::core::Result<IUpdateCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICategory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>() -> ICategory_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CategoryID<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CategoryID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Order<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Order() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            CategoryID: CategoryID::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            Order: Order::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICategory as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICategoryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<ICategory>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICategoryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICategoryCollection_Impl, const OFFSET: isize>() -> ICategoryCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICategoryCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallback_Impl: Sized {
    fn Invoke(&self, downloadjob: &::core::option::Option<IDownloadJob>, callbackargs: &::core::option::Option<IDownloadCompletedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadCompletedCallback_Impl, const OFFSET: isize>() -> IDownloadCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&downloadjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadCompletedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadCompletedCallbackArgs_Impl, const OFFSET: isize>() -> IDownloadCompletedCallbackArgs_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadCompletedCallbackArgs as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsCompleted(&self) -> ::windows::core::Result<i16>;
    fn Updates(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> ::windows::core::Result<()>;
    fn GetProgress(&self) -> ::windows::core::Result<IDownloadProgress>;
    fn RequestAbort(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>() -> IDownloadJob_Vtbl {
        unsafe extern "system" fn AsyncState<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CleanUp().into()
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestAbort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadJob as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateBytesDownloaded(&self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateBytesToDownload(&self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateIndex(&self) -> ::windows::core::Result<i32>;
    fn PercentComplete(&self) -> ::windows::core::Result<i32>;
    fn TotalBytesDownloaded(&self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn TotalBytesToDownload(&self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows::core::Result<IUpdateDownloadResult>;
    fn CurrentUpdateDownloadPhase(&self) -> ::windows::core::Result<DownloadPhase>;
    fn CurrentUpdatePercentComplete(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>() -> IDownloadProgress_Vtbl {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdateBytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdateBytesToDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateIndex<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdateIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesDownloaded<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalBytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesToDownload<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalBytesToDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPhase) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdateDownloadPhase() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdatePercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentUpdateBytesDownloaded: CurrentUpdateBytesDownloaded::<Identity, Impl, OFFSET>,
            CurrentUpdateBytesToDownload: CurrentUpdateBytesToDownload::<Identity, Impl, OFFSET>,
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, Impl, OFFSET>,
            PercentComplete: PercentComplete::<Identity, Impl, OFFSET>,
            TotalBytesDownloaded: TotalBytesDownloaded::<Identity, Impl, OFFSET>,
            TotalBytesToDownload: TotalBytesToDownload::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
            CurrentUpdateDownloadPhase: CurrentUpdateDownloadPhase::<Identity, Impl, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgress as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallback_Impl: Sized {
    fn Invoke(&self, downloadjob: &::core::option::Option<IDownloadJob>, callbackargs: &::core::option::Option<IDownloadProgressChangedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>() -> IDownloadProgressChangedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&downloadjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&self) -> ::windows::core::Result<IDownloadProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> IDownloadProgressChangedCallbackArgs_Vtbl {
        unsafe extern "system" fn Progress<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Progress: Progress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallbackArgs as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows::core::Result<i32>;
    fn ResultCode(&self) -> ::windows::core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows::core::Result<IUpdateDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadResult_Impl, const OFFSET: isize>() -> IDownloadResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadResult as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IImageInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn AltText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn Source(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Width(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IImageInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageInformation_Impl, const OFFSET: isize>() -> IImageInformation_Vtbl {
        unsafe extern "system" fn AltText<Identity: ::windows::core::IUnknownImpl, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AltText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows::core::IUnknownImpl, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Identity: ::windows::core::IUnknownImpl, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AltText: AltText::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Source: Source::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageInformation as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationAgent_Impl: Sized + super::Com::IDispatch_Impl {
    fn RecordInstallationResult(&self, installationresultcookie: &super::super::Foundation::BSTR, hresult: i32, extendedreportingdata: &::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationAgent_Impl, const OFFSET: isize>() -> IInstallationAgent_Vtbl {
        unsafe extern "system" fn RecordInstallationResult<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationresultcookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hresult: i32, extendedreportingdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecordInstallationResult(::core::mem::transmute(&installationresultcookie), ::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&extendedreportingdata)).into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), RecordInstallationResult: RecordInstallationResult::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationAgent as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationBehavior_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanRequestUserInput(&self) -> ::windows::core::Result<i16>;
    fn Impact(&self) -> ::windows::core::Result<InstallationImpact>;
    fn RebootBehavior(&self) -> ::windows::core::Result<InstallationRebootBehavior>;
    fn RequiresNetworkConnectivity(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationBehavior_Impl, const OFFSET: isize>() -> IInstallationBehavior_Vtbl {
        unsafe extern "system" fn CanRequestUserInput<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRequestUserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Impact<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationImpact) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Impact() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequiresNetworkConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CanRequestUserInput: CanRequestUserInput::<Identity, Impl, OFFSET>,
            Impact: Impact::<Identity, Impl, OFFSET>,
            RebootBehavior: RebootBehavior::<Identity, Impl, OFFSET>,
            RequiresNetworkConnectivity: RequiresNetworkConnectivity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationBehavior as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallback_Impl: Sized {
    fn Invoke(&self, installationjob: &::core::option::Option<IInstallationJob>, callbackargs: &::core::option::Option<IInstallationCompletedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationCompletedCallback_Impl, const OFFSET: isize>() -> IInstallationCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&installationjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationCompletedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationCompletedCallbackArgs_Impl, const OFFSET: isize>() -> IInstallationCompletedCallbackArgs_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationCompletedCallbackArgs as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsCompleted(&self) -> ::windows::core::Result<i16>;
    fn Updates(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> ::windows::core::Result<()>;
    fn GetProgress(&self) -> ::windows::core::Result<IInstallationProgress>;
    fn RequestAbort(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>() -> IInstallationJob_Vtbl {
        unsafe extern "system" fn AsyncState<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CleanUp().into()
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestAbort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationJob as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateIndex(&self) -> ::windows::core::Result<i32>;
    fn CurrentUpdatePercentComplete(&self) -> ::windows::core::Result<i32>;
    fn PercentComplete(&self) -> ::windows::core::Result<i32>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows::core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgress_Impl, const OFFSET: isize>() -> IInstallationProgress_Vtbl {
        unsafe extern "system" fn CurrentUpdateIndex<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdateIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentUpdatePercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, Impl, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, Impl, OFFSET>,
            PercentComplete: PercentComplete::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationProgress as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallback_Impl: Sized {
    fn Invoke(&self, installationjob: &::core::option::Option<IInstallationJob>, callbackargs: &::core::option::Option<IInstallationProgressChangedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>() -> IInstallationProgressChangedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&installationjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&self) -> ::windows::core::Result<IInstallationProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> IInstallationProgressChangedCallbackArgs_Vtbl {
        unsafe extern "system" fn Progress<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Progress: Progress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallbackArgs as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows::core::Result<i32>;
    fn RebootRequired(&self) -> ::windows::core::Result<i16>;
    fn ResultCode(&self) -> ::windows::core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows::core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationResult_Impl, const OFFSET: isize>() -> IInstallationResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationResult as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInvalidProductLicenseException_Impl: Sized + super::Com::IDispatch_Impl + IUpdateException_Impl {
    fn Product(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInvalidProductLicenseException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvalidProductLicenseException_Impl, const OFFSET: isize>() -> IInvalidProductLicenseException_Vtbl {
        unsafe extern "system" fn Product<Identity: ::windows::core::IUnknownImpl, Impl: IInvalidProductLicenseException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Product() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateException_Vtbl::new::<Identity, Impl, OFFSET>(), Product: Product::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvalidProductLicenseException as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateException as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallback_Impl: Sized {
    fn Invoke(&self, searchjob: &::core::option::Option<ISearchJob>, callbackargs: &::core::option::Option<ISearchCompletedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCompletedCallback_Impl, const OFFSET: isize>() -> ISearchCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&searchjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCompletedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISearchCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISearchCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCompletedCallbackArgs_Impl, const OFFSET: isize>() -> ISearchCompletedCallbackArgs_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCompletedCallbackArgs as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISearchJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsCompleted(&self) -> ::windows::core::Result<i16>;
    fn CleanUp(&self) -> ::windows::core::Result<()>;
    fn RequestAbort(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISearchJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchJob_Impl, const OFFSET: isize>() -> ISearchJob_Vtbl {
        unsafe extern "system" fn AsyncState<Identity: ::windows::core::IUnknownImpl, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows::core::IUnknownImpl, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows::core::IUnknownImpl, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CleanUp().into()
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows::core::IUnknownImpl, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RequestAbort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchJob as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISearchResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn ResultCode(&self) -> ::windows::core::Result<OperationResultCode>;
    fn RootCategories(&self) -> ::windows::core::Result<ICategoryCollection>;
    fn Updates(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn Warnings(&self) -> ::windows::core::Result<IUpdateExceptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISearchResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchResult_Impl, const OFFSET: isize>() -> ISearchResult_Vtbl {
        unsafe extern "system" fn ResultCode<Identity: ::windows::core::IUnknownImpl, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootCategories<Identity: ::windows::core::IUnknownImpl, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RootCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows::core::IUnknownImpl, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Warnings<Identity: ::windows::core::IUnknownImpl, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Warnings() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            RootCategories: RootCategories::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            Warnings: Warnings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchResult as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStringCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetItem(&self, index: i32, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn ReadOnly(&self) -> ::windows::core::Result<i16>;
    fn Add(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn Copy(&self) -> ::windows::core::Result<IStringCollection>;
    fn Insert(&self, index: i32, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>() -> IStringCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetItem(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            SetItem: SetItem::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStringCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn OemHardwareSupportLink(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RebootRequired(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemInformation_Impl, const OFFSET: isize>() -> ISystemInformation_Vtbl {
        unsafe extern "system" fn OemHardwareSupportLink<Identity: ::windows::core::IUnknownImpl, Impl: ISystemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OemHardwareSupportLink() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows::core::IUnknownImpl, Impl: ISystemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OemHardwareSupportLink: OemHardwareSupportLink::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemInformation as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate_Impl: Sized + super::Com::IDispatch_Impl {
    fn Title(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AutoSelectOnWebSites(&self) -> ::windows::core::Result<i16>;
    fn BundledUpdates(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn CanRequireSource(&self) -> ::windows::core::Result<i16>;
    fn Categories(&self) -> ::windows::core::Result<ICategoryCollection>;
    fn Deadline(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DeltaCompressedContentAvailable(&self) -> ::windows::core::Result<i16>;
    fn DeltaCompressedContentPreferred(&self) -> ::windows::core::Result<i16>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EulaAccepted(&self) -> ::windows::core::Result<i16>;
    fn EulaText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HandlerID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Identity(&self) -> ::windows::core::Result<IUpdateIdentity>;
    fn Image(&self) -> ::windows::core::Result<IImageInformation>;
    fn InstallationBehavior(&self) -> ::windows::core::Result<IInstallationBehavior>;
    fn IsBeta(&self) -> ::windows::core::Result<i16>;
    fn IsDownloaded(&self) -> ::windows::core::Result<i16>;
    fn IsHidden(&self) -> ::windows::core::Result<i16>;
    fn SetIsHidden(&self, value: i16) -> ::windows::core::Result<()>;
    fn IsInstalled(&self) -> ::windows::core::Result<i16>;
    fn IsMandatory(&self) -> ::windows::core::Result<i16>;
    fn IsUninstallable(&self) -> ::windows::core::Result<i16>;
    fn Languages(&self) -> ::windows::core::Result<IStringCollection>;
    fn LastDeploymentChangeTime(&self) -> ::windows::core::Result<f64>;
    fn MaxDownloadSize(&self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn MinDownloadSize(&self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn MoreInfoUrls(&self) -> ::windows::core::Result<IStringCollection>;
    fn MsrcSeverity(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RecommendedCpuSpeed(&self) -> ::windows::core::Result<i32>;
    fn RecommendedHardDiskSpace(&self) -> ::windows::core::Result<i32>;
    fn RecommendedMemory(&self) -> ::windows::core::Result<i32>;
    fn ReleaseNotes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SecurityBulletinIDs(&self) -> ::windows::core::Result<IStringCollection>;
    fn SupersededUpdateIDs(&self) -> ::windows::core::Result<IStringCollection>;
    fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&self) -> ::windows::core::Result<UpdateType>;
    fn UninstallationNotes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UninstallationBehavior(&self) -> ::windows::core::Result<IInstallationBehavior>;
    fn UninstallationSteps(&self) -> ::windows::core::Result<IStringCollection>;
    fn KBArticleIDs(&self) -> ::windows::core::Result<IStringCollection>;
    fn AcceptEula(&self) -> ::windows::core::Result<()>;
    fn DeploymentAction(&self) -> ::windows::core::Result<DeploymentAction>;
    fn CopyFromCache(&self, path: &super::super::Foundation::BSTR, toextractcabfiles: i16) -> ::windows::core::Result<()>;
    fn DownloadPriority(&self) -> ::windows::core::Result<DownloadPriority>;
    fn DownloadContents(&self) -> ::windows::core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>() -> IUpdate_Vtbl {
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoSelectOnWebSites() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BundledUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BundledUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequireSource<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRequireSource() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Categories<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Categories() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeltaCompressedContentAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeltaCompressedContentPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaAccepted<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EulaAccepted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaText<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EulaText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandlerID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HandlerID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identity<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallationBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstallationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBeta<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsBeta() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloaded<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsHidden(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IsInstalled<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMandatory<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMandatory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastDeploymentChangeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDownloadSize<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDownloadSize<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreInfoUrls<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoreInfoUrls() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsrcSeverity<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MsrcSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecommendedCpuSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecommendedHardDiskSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedMemory<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RecommendedMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseNotes<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReleaseNotes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityBulletinIDs<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityBulletinIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupersededUpdateIDs<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupersededUpdateIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UninstallationNotes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UninstallationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UninstallationSteps() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KBArticleIDs<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KBArticleIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptEula<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcceptEula().into()
        }
        unsafe extern "system" fn DeploymentAction<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DeploymentAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeploymentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromCache<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, toextractcabfiles: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyFromCache(::core::mem::transmute(&path), ::core::mem::transmute_copy(&toextractcabfiles)).into()
        }
        unsafe extern "system" fn DownloadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadContents<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadContents() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Title: Title::<Identity, Impl, OFFSET>,
            AutoSelectOnWebSites: AutoSelectOnWebSites::<Identity, Impl, OFFSET>,
            BundledUpdates: BundledUpdates::<Identity, Impl, OFFSET>,
            CanRequireSource: CanRequireSource::<Identity, Impl, OFFSET>,
            Categories: Categories::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
            DeltaCompressedContentAvailable: DeltaCompressedContentAvailable::<Identity, Impl, OFFSET>,
            DeltaCompressedContentPreferred: DeltaCompressedContentPreferred::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            EulaAccepted: EulaAccepted::<Identity, Impl, OFFSET>,
            EulaText: EulaText::<Identity, Impl, OFFSET>,
            HandlerID: HandlerID::<Identity, Impl, OFFSET>,
            Identity: Identity::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            InstallationBehavior: InstallationBehavior::<Identity, Impl, OFFSET>,
            IsBeta: IsBeta::<Identity, Impl, OFFSET>,
            IsDownloaded: IsDownloaded::<Identity, Impl, OFFSET>,
            IsHidden: IsHidden::<Identity, Impl, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, Impl, OFFSET>,
            IsInstalled: IsInstalled::<Identity, Impl, OFFSET>,
            IsMandatory: IsMandatory::<Identity, Impl, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, Impl, OFFSET>,
            Languages: Languages::<Identity, Impl, OFFSET>,
            LastDeploymentChangeTime: LastDeploymentChangeTime::<Identity, Impl, OFFSET>,
            MaxDownloadSize: MaxDownloadSize::<Identity, Impl, OFFSET>,
            MinDownloadSize: MinDownloadSize::<Identity, Impl, OFFSET>,
            MoreInfoUrls: MoreInfoUrls::<Identity, Impl, OFFSET>,
            MsrcSeverity: MsrcSeverity::<Identity, Impl, OFFSET>,
            RecommendedCpuSpeed: RecommendedCpuSpeed::<Identity, Impl, OFFSET>,
            RecommendedHardDiskSpace: RecommendedHardDiskSpace::<Identity, Impl, OFFSET>,
            RecommendedMemory: RecommendedMemory::<Identity, Impl, OFFSET>,
            ReleaseNotes: ReleaseNotes::<Identity, Impl, OFFSET>,
            SecurityBulletinIDs: SecurityBulletinIDs::<Identity, Impl, OFFSET>,
            SupersededUpdateIDs: SupersededUpdateIDs::<Identity, Impl, OFFSET>,
            SupportUrl: SupportUrl::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, Impl, OFFSET>,
            UninstallationBehavior: UninstallationBehavior::<Identity, Impl, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, Impl, OFFSET>,
            KBArticleIDs: KBArticleIDs::<Identity, Impl, OFFSET>,
            AcceptEula: AcceptEula::<Identity, Impl, OFFSET>,
            DeploymentAction: DeploymentAction::<Identity, Impl, OFFSET>,
            CopyFromCache: CopyFromCache::<Identity, Impl, OFFSET>,
            DownloadPriority: DownloadPriority::<Identity, Impl, OFFSET>,
            DownloadContents: DownloadContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate2_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl {
    fn RebootRequired(&self) -> ::windows::core::Result<i16>;
    fn IsPresent(&self) -> ::windows::core::Result<i16>;
    fn CveIDs(&self) -> ::windows::core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: &::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate2_Impl, const OFFSET: isize>() -> IUpdate2_Vtbl {
        unsafe extern "system" fn RebootRequired<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CveIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiles: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyToCache(::core::mem::transmute(&pfiles)).into()
        }
        Self {
            base: IUpdate_Vtbl::new::<Identity, Impl, OFFSET>(),
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            IsPresent: IsPresent::<Identity, Impl, OFFSET>,
            CveIDs: CveIDs::<Identity, Impl, OFFSET>,
            CopyToCache: CopyToCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate3_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IUpdate2_Impl {
    fn BrowseOnly(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate3_Impl, const OFFSET: isize>() -> IUpdate3_Vtbl {
        unsafe extern "system" fn BrowseOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BrowseOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdate2_Vtbl::new::<Identity, Impl, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IUpdate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate4_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IUpdate2_Impl + IUpdate3_Impl {
    fn PerUser(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate4_Impl, const OFFSET: isize>() -> IUpdate4_Vtbl {
        unsafe extern "system" fn PerUser<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdate3_Vtbl::new::<Identity, Impl, OFFSET>(), PerUser: PerUser::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IUpdate2 as ::windows::core::Interface>::IID || iid == &<IUpdate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate5_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IUpdate2_Impl + IUpdate3_Impl + IUpdate4_Impl {
    fn AutoSelection(&self) -> ::windows::core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> ::windows::core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate5_Impl, const OFFSET: isize>() -> IUpdate5_Vtbl {
        unsafe extern "system" fn AutoSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdate4_Vtbl::new::<Identity, Impl, OFFSET>(),
            AutoSelection: AutoSelection::<Identity, Impl, OFFSET>,
            AutoDownload: AutoDownload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate5 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IUpdate2 as ::windows::core::Interface>::IID || iid == &<IUpdate3 as ::windows::core::Interface>::IID || iid == &<IUpdate4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<IUpdate>;
    fn SetItem(&self, index: i32, value: &::core::option::Option<IUpdate>) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn ReadOnly(&self) -> ::windows::core::Result<i16>;
    fn Add(&self, value: &::core::option::Option<IUpdate>) -> ::windows::core::Result<i32>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn Copy(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn Insert(&self, index: i32, value: &::core::option::Option<IUpdate>) -> ::windows::core::Result<()>;
    fn RemoveAt(&self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>() -> IUpdateCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetItem(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            SetItem: SetItem::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn DownloadUrl(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContent_Impl, const OFFSET: isize>() -> IUpdateDownloadContent_Vtbl {
        unsafe extern "system" fn DownloadUrl<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadContent as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadContent2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateDownloadContent_Impl {
    fn IsDeltaCompressedContent(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadContent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContent2_Impl, const OFFSET: isize>() -> IUpdateDownloadContent2_Vtbl {
        unsafe extern "system" fn IsDeltaCompressedContent<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDeltaCompressedContent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateDownloadContent_Vtbl::new::<Identity, Impl, OFFSET>(), IsDeltaCompressedContent: IsDeltaCompressedContent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadContent2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateDownloadContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadContentCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<IUpdateDownloadContent>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadContentCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>() -> IUpdateDownloadContentCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadContentCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows::core::Result<i32>;
    fn ResultCode(&self) -> ::windows::core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>() -> IUpdateDownloadResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadResult as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloader_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsForced(&self) -> ::windows::core::Result<i16>;
    fn SetIsForced(&self, value: i16) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<DownloadPriority>;
    fn SetPriority(&self, value: DownloadPriority) -> ::windows::core::Result<()>;
    fn Updates(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: &::core::option::Option<IUpdateCollection>) -> ::windows::core::Result<()>;
    fn BeginDownload(&self, onprogresschanged: &::core::option::Option<::windows::core::IUnknown>, oncompleted: &::core::option::Option<::windows::core::IUnknown>, state: &super::Com::VARIANT) -> ::windows::core::Result<IDownloadJob>;
    fn Download(&self) -> ::windows::core::Result<IDownloadResult>;
    fn EndDownload(&self, value: &::core::option::Option<IDownloadJob>) -> ::windows::core::Result<IDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>() -> IUpdateDownloader_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsForced<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsForced() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsForced(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Updates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdates(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginDownload(::core::mem::transmute(&onprogresschanged), ::core::mem::transmute(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Download() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDownload<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndDownload(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            IsForced: IsForced::<Identity, Impl, OFFSET>,
            SetIsForced: SetIsForced::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            SetUpdates: SetUpdates::<Identity, Impl, OFFSET>,
            BeginDownload: BeginDownload::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            EndDownload: EndDownload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloader as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateException_Impl: Sized + super::Com::IDispatch_Impl {
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HResult(&self) -> ::windows::core::Result<i32>;
    fn Context(&self) -> ::windows::core::Result<UpdateExceptionContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateException_Impl, const OFFSET: isize>() -> IUpdateException_Vtbl {
        unsafe extern "system" fn Message<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateExceptionContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Message: Message::<Identity, Impl, OFFSET>,
            HResult: HResult::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateException as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateExceptionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<IUpdateException>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateExceptionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>() -> IUpdateExceptionCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateExceptionCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateHistoryEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Operation(&self) -> ::windows::core::Result<UpdateOperation>;
    fn ResultCode(&self) -> ::windows::core::Result<OperationResultCode>;
    fn HResult(&self) -> ::windows::core::Result<i32>;
    fn Date(&self) -> ::windows::core::Result<f64>;
    fn UpdateIdentity(&self) -> ::windows::core::Result<IUpdateIdentity>;
    fn Title(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UnmappedResultCode(&self) -> ::windows::core::Result<i32>;
    fn ClientApplicationID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServerSelection(&self) -> ::windows::core::Result<ServerSelection>;
    fn ServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UninstallationSteps(&self) -> ::windows::core::Result<IStringCollection>;
    fn UninstallationNotes(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateHistoryEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>() -> IUpdateHistoryEntry_Vtbl {
        unsafe extern "system" fn Operation<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmappedResultCode<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnmappedResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UninstallationSteps() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UninstallationNotes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Operation: Operation::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            HResult: HResult::<Identity, Impl, OFFSET>,
            Date: Date::<Identity, Impl, OFFSET>,
            UpdateIdentity: UpdateIdentity::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            UnmappedResultCode: UnmappedResultCode::<Identity, Impl, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            ServerSelection: ServerSelection::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            UninstallationSteps: UninstallationSteps::<Identity, Impl, OFFSET>,
            UninstallationNotes: UninstallationNotes::<Identity, Impl, OFFSET>,
            SupportUrl: SupportUrl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateHistoryEntry2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateHistoryEntry_Impl {
    fn Categories(&self) -> ::windows::core::Result<ICategoryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateHistoryEntry2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry2_Impl, const OFFSET: isize>() -> IUpdateHistoryEntry2_Vtbl {
        unsafe extern "system" fn Categories<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Categories() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateHistoryEntry_Vtbl::new::<Identity, Impl, OFFSET>(), Categories: Categories::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateHistoryEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateHistoryEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<IUpdateHistoryEntry>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateHistoryEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>() -> IUpdateHistoryEntryCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateHistoryEntryCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateIdentity_Impl: Sized + super::Com::IDispatch_Impl {
    fn RevisionNumber(&self) -> ::windows::core::Result<i32>;
    fn UpdateID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateIdentity_Impl, const OFFSET: isize>() -> IUpdateIdentity_Vtbl {
        unsafe extern "system" fn RevisionNumber<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RevisionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RevisionNumber: RevisionNumber::<Identity, Impl, OFFSET>,
            UpdateID: UpdateID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateIdentity as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows::core::Result<i32>;
    fn RebootRequired(&self) -> ::windows::core::Result<i16>;
    fn ResultCode(&self) -> ::windows::core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstallationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>() -> IUpdateInstallationResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstallationResult as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsForced(&self) -> ::windows::core::Result<i16>;
    fn SetIsForced(&self, value: i16) -> ::windows::core::Result<()>;
    fn ParentHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn SetParentHwnd(&self, value: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn SetParentWindow(&self, value: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ParentWindow(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Updates(&self) -> ::windows::core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: &::core::option::Option<IUpdateCollection>) -> ::windows::core::Result<()>;
    fn BeginInstall(&self, onprogresschanged: &::core::option::Option<::windows::core::IUnknown>, oncompleted: &::core::option::Option<::windows::core::IUnknown>, state: &super::Com::VARIANT) -> ::windows::core::Result<IInstallationJob>;
    fn BeginUninstall(&self, onprogresschanged: &::core::option::Option<::windows::core::IUnknown>, oncompleted: &::core::option::Option<::windows::core::IUnknown>, state: &super::Com::VARIANT) -> ::windows::core::Result<IInstallationJob>;
    fn EndInstall(&self, value: &::core::option::Option<IInstallationJob>) -> ::windows::core::Result<IInstallationResult>;
    fn EndUninstall(&self, value: &::core::option::Option<IInstallationJob>) -> ::windows::core::Result<IInstallationResult>;
    fn Install(&self) -> ::windows::core::Result<IInstallationResult>;
    fn RunWizard(&self, dialogtitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<IInstallationResult>;
    fn IsBusy(&self) -> ::windows::core::Result<i16>;
    fn Uninstall(&self) -> ::windows::core::Result<IInstallationResult>;
    fn AllowSourcePrompts(&self) -> ::windows::core::Result<i16>;
    fn SetAllowSourcePrompts(&self, value: i16) -> ::windows::core::Result<()>;
    fn RebootRequiredBeforeInstallation(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>() -> IUpdateInstaller_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsForced<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsForced() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsForced(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParentHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParentHwnd(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetParentWindow<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetParentWindow(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdates(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BeginInstall<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginInstall(::core::mem::transmute(&onprogresschanged), ::core::mem::transmute(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstall<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginUninstall(::core::mem::transmute(&onprogresschanged), ::core::mem::transmute(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInstall<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndInstall(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUninstall<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndUninstall(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Install() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWizard<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dialogtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RunWizard(::core::mem::transmute(&dialogtitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBusy<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsBusy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninstall<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Uninstall() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSourcePrompts<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowSourcePrompts() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowSourcePrompts(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootRequiredBeforeInstallation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            IsForced: IsForced::<Identity, Impl, OFFSET>,
            SetIsForced: SetIsForced::<Identity, Impl, OFFSET>,
            ParentHwnd: ParentHwnd::<Identity, Impl, OFFSET>,
            SetParentHwnd: SetParentHwnd::<Identity, Impl, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, Impl, OFFSET>,
            ParentWindow: ParentWindow::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            SetUpdates: SetUpdates::<Identity, Impl, OFFSET>,
            BeginInstall: BeginInstall::<Identity, Impl, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, Impl, OFFSET>,
            EndInstall: EndInstall::<Identity, Impl, OFFSET>,
            EndUninstall: EndUninstall::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            RunWizard: RunWizard::<Identity, Impl, OFFSET>,
            IsBusy: IsBusy::<Identity, Impl, OFFSET>,
            Uninstall: Uninstall::<Identity, Impl, OFFSET>,
            AllowSourcePrompts: AllowSourcePrompts::<Identity, Impl, OFFSET>,
            SetAllowSourcePrompts: SetAllowSourcePrompts::<Identity, Impl, OFFSET>,
            RebootRequiredBeforeInstallation: RebootRequiredBeforeInstallation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateInstaller_Impl {
    fn ForceQuiet(&self) -> ::windows::core::Result<i16>;
    fn SetForceQuiet(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller2_Impl, const OFFSET: isize>() -> IUpdateInstaller2_Vtbl {
        unsafe extern "system" fn ForceQuiet<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForceQuiet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceQuiet<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForceQuiet(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateInstaller_Vtbl::new::<Identity, Impl, OFFSET>(),
            ForceQuiet: ForceQuiet::<Identity, Impl, OFFSET>,
            SetForceQuiet: SetForceQuiet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateInstaller as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller3_Impl: Sized + super::Com::IDispatch_Impl + IUpdateInstaller_Impl + IUpdateInstaller2_Impl {
    fn AttemptCloseAppsIfNecessary(&self) -> ::windows::core::Result<i16>;
    fn SetAttemptCloseAppsIfNecessary(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller3_Impl, const OFFSET: isize>() -> IUpdateInstaller3_Vtbl {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AttemptCloseAppsIfNecessary() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttemptCloseAppsIfNecessary(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateInstaller2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AttemptCloseAppsIfNecessary: AttemptCloseAppsIfNecessary::<Identity, Impl, OFFSET>,
            SetAttemptCloseAppsIfNecessary: SetAttemptCloseAppsIfNecessary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateInstaller as ::windows::core::Interface>::IID || iid == &<IUpdateInstaller2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller4_Impl: Sized + super::Com::IDispatch_Impl + IUpdateInstaller_Impl + IUpdateInstaller2_Impl + IUpdateInstaller3_Impl {
    fn Commit(&self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller4_Impl, const OFFSET: isize>() -> IUpdateInstaller4_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: IUpdateInstaller3_Vtbl::new::<Identity, Impl, OFFSET>(), Commit: Commit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateInstaller as ::windows::core::Interface>::IID || iid == &<IUpdateInstaller2 as ::windows::core::Interface>::IID || iid == &<IUpdateInstaller3 as ::windows::core::Interface>::IID
    }
}
pub trait IUpdateLockdown_Impl: Sized {
    fn LockDown(&self, flags: i32) -> ::windows::core::Result<()>;
}
impl IUpdateLockdown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateLockdown_Impl, const OFFSET: isize>() -> IUpdateLockdown_Vtbl {
        unsafe extern "system" fn LockDown<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateLockdown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockDown(::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), LockDown: LockDown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateLockdown as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSearcher_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanAutomaticallyUpgradeService(&self) -> ::windows::core::Result<i16>;
    fn SetCanAutomaticallyUpgradeService(&self, value: i16) -> ::windows::core::Result<()>;
    fn ClientApplicationID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IncludePotentiallySupersededUpdates(&self) -> ::windows::core::Result<i16>;
    fn SetIncludePotentiallySupersededUpdates(&self, value: i16) -> ::windows::core::Result<()>;
    fn ServerSelection(&self) -> ::windows::core::Result<ServerSelection>;
    fn SetServerSelection(&self, value: ServerSelection) -> ::windows::core::Result<()>;
    fn BeginSearch(&self, criteria: &super::super::Foundation::BSTR, oncompleted: &::core::option::Option<::windows::core::IUnknown>, state: &super::Com::VARIANT) -> ::windows::core::Result<ISearchJob>;
    fn EndSearch(&self, searchjob: &::core::option::Option<ISearchJob>) -> ::windows::core::Result<ISearchResult>;
    fn EscapeString(&self, unescaped: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn QueryHistory(&self, startindex: i32, count: i32) -> ::windows::core::Result<IUpdateHistoryEntryCollection>;
    fn Search(&self, criteria: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISearchResult>;
    fn Online(&self) -> ::windows::core::Result<i16>;
    fn SetOnline(&self, value: i16) -> ::windows::core::Result<()>;
    fn GetTotalHistoryCount(&self) -> ::windows::core::Result<i32>;
    fn ServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceID(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSearcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>() -> IUpdateSearcher_Vtbl {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanAutomaticallyUpgradeService() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCanAutomaticallyUpgradeService(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IncludePotentiallySupersededUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIncludePotentiallySupersededUpdates(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ServerSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServerSelection(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BeginSearch<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginSearch(::core::mem::transmute(&criteria), ::core::mem::transmute(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSearch<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchjob: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndSearch(::core::mem::transmute(&searchjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeString<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unescaped: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EscapeString(::core::mem::transmute(&unescaped)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: i32, count: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHistory(::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Search<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Search(::core::mem::transmute(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Online() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOnline<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOnline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTotalHistoryCount<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTotalHistoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceID(::core::mem::transmute(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CanAutomaticallyUpgradeService: CanAutomaticallyUpgradeService::<Identity, Impl, OFFSET>,
            SetCanAutomaticallyUpgradeService: SetCanAutomaticallyUpgradeService::<Identity, Impl, OFFSET>,
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            IncludePotentiallySupersededUpdates: IncludePotentiallySupersededUpdates::<Identity, Impl, OFFSET>,
            SetIncludePotentiallySupersededUpdates: SetIncludePotentiallySupersededUpdates::<Identity, Impl, OFFSET>,
            ServerSelection: ServerSelection::<Identity, Impl, OFFSET>,
            SetServerSelection: SetServerSelection::<Identity, Impl, OFFSET>,
            BeginSearch: BeginSearch::<Identity, Impl, OFFSET>,
            EndSearch: EndSearch::<Identity, Impl, OFFSET>,
            EscapeString: EscapeString::<Identity, Impl, OFFSET>,
            QueryHistory: QueryHistory::<Identity, Impl, OFFSET>,
            Search: Search::<Identity, Impl, OFFSET>,
            Online: Online::<Identity, Impl, OFFSET>,
            SetOnline: SetOnline::<Identity, Impl, OFFSET>,
            GetTotalHistoryCount: GetTotalHistoryCount::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            SetServiceID: SetServiceID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSearcher as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSearcher2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSearcher_Impl {
    fn IgnoreDownloadPriority(&self) -> ::windows::core::Result<i16>;
    fn SetIgnoreDownloadPriority(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSearcher2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher2_Impl, const OFFSET: isize>() -> IUpdateSearcher2_Vtbl {
        unsafe extern "system" fn IgnoreDownloadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IgnoreDownloadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIgnoreDownloadPriority(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateSearcher_Vtbl::new::<Identity, Impl, OFFSET>(),
            IgnoreDownloadPriority: IgnoreDownloadPriority::<Identity, Impl, OFFSET>,
            SetIgnoreDownloadPriority: SetIgnoreDownloadPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSearcher2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateSearcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSearcher3_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSearcher_Impl + IUpdateSearcher2_Impl {
    fn SearchScope(&self) -> ::windows::core::Result<SearchScope>;
    fn SetSearchScope(&self, value: SearchScope) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSearcher3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher3_Impl, const OFFSET: isize>() -> IUpdateSearcher3_Vtbl {
        unsafe extern "system" fn SearchScope<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut SearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SearchScope() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchScope<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSearchScope(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateSearcher2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SearchScope: SearchScope::<Identity, Impl, OFFSET>,
            SetSearchScope: SetSearchScope::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSearcher3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateSearcher as ::windows::core::Interface>::IID || iid == &<IUpdateSearcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateService_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ContentValidationCert(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ExpirationDate(&self) -> ::windows::core::Result<f64>;
    fn IsManaged(&self) -> ::windows::core::Result<i16>;
    fn IsRegisteredWithAU(&self) -> ::windows::core::Result<i16>;
    fn IssueDate(&self) -> ::windows::core::Result<f64>;
    fn OffersWindowsUpdates(&self) -> ::windows::core::Result<i16>;
    fn RedirectUrls(&self) -> ::windows::core::Result<IStringCollection>;
    fn ServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsScanPackageService(&self) -> ::windows::core::Result<i16>;
    fn CanRegisterWithAU(&self) -> ::windows::core::Result<i16>;
    fn ServiceUrl(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetupPrefix(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>() -> IUpdateService_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentValidationCert<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentValidationCert() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsManaged<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsManaged() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredWithAU<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRegisteredWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDate<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IssueDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffersWindowsUpdates<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OffersWindowsUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectUrls<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RedirectUrls() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanPackageService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsScanPackageService() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRegisterWithAU<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRegisterWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUrl<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetupPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            ContentValidationCert: ContentValidationCert::<Identity, Impl, OFFSET>,
            ExpirationDate: ExpirationDate::<Identity, Impl, OFFSET>,
            IsManaged: IsManaged::<Identity, Impl, OFFSET>,
            IsRegisteredWithAU: IsRegisteredWithAU::<Identity, Impl, OFFSET>,
            IssueDate: IssueDate::<Identity, Impl, OFFSET>,
            OffersWindowsUpdates: OffersWindowsUpdates::<Identity, Impl, OFFSET>,
            RedirectUrls: RedirectUrls::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            IsScanPackageService: IsScanPackageService::<Identity, Impl, OFFSET>,
            CanRegisterWithAU: CanRegisterWithAU::<Identity, Impl, OFFSET>,
            ServiceUrl: ServiceUrl::<Identity, Impl, OFFSET>,
            SetupPrefix: SetupPrefix::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateService as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateService2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateService_Impl {
    fn IsDefaultAUService(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService2_Impl, const OFFSET: isize>() -> IUpdateService2_Vtbl {
        unsafe extern "system" fn IsDefaultAUService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDefaultAUService() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateService_Vtbl::new::<Identity, Impl, OFFSET>(), IsDefaultAUService: IsDefaultAUService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateService2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<IUpdateService>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>() -> IUpdateServiceCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Services(&self) -> ::windows::core::Result<IUpdateServiceCollection>;
    fn AddService(&self, serviceid: &super::super::Foundation::BSTR, authorizationcabpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IUpdateService>;
    fn RegisterServiceWithAU(&self, serviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveService(&self, serviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UnregisterServiceWithAU(&self, serviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddScanPackageService(&self, servicename: &super::super::Foundation::BSTR, scanfilelocation: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<IUpdateService>;
    fn SetOption(&self, optionname: &super::super::Foundation::BSTR, optionvalue: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>() -> IUpdateServiceManager_Vtbl {
        unsafe extern "system" fn Services<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddService(::core::mem::transmute(&serviceid), ::core::mem::transmute(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServiceWithAU<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterServiceWithAU(::core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn RemoveService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveService(::core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterServiceWithAU(::core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn AddScanPackageService<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scanfilelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddScanPackageService(::core::mem::transmute(&servicename), ::core::mem::transmute(&scanfilelocation), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOption(::core::mem::transmute(&optionname), ::core::mem::transmute(&optionvalue)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Services: Services::<Identity, Impl, OFFSET>,
            AddService: AddService::<Identity, Impl, OFFSET>,
            RegisterServiceWithAU: RegisterServiceWithAU::<Identity, Impl, OFFSET>,
            RemoveService: RemoveService::<Identity, Impl, OFFSET>,
            UnregisterServiceWithAU: UnregisterServiceWithAU::<Identity, Impl, OFFSET>,
            AddScanPackageService: AddScanPackageService::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceManager2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateServiceManager_Impl {
    fn ClientApplicationID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryServiceRegistration(&self, serviceid: &super::super::Foundation::BSTR) -> ::windows::core::Result<IUpdateServiceRegistration>;
    fn AddService2(&self, serviceid: &super::super::Foundation::BSTR, flags: i32, authorizationcabpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<IUpdateServiceRegistration>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>() -> IUpdateServiceManager2_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn QueryServiceRegistration<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryServiceRegistration(::core::mem::transmute(&serviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService2<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddService2(::core::mem::transmute(&serviceid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdateServiceManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Identity, Impl, OFFSET>,
            AddService2: AddService2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceManager2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateServiceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceRegistration_Impl: Sized + super::Com::IDispatch_Impl {
    fn RegistrationState(&self) -> ::windows::core::Result<UpdateServiceRegistrationState>;
    fn ServiceID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsPendingRegistrationWithAU(&self) -> ::windows::core::Result<i16>;
    fn Service(&self) -> ::windows::core::Result<IUpdateService2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>() -> IUpdateServiceRegistration_Vtbl {
        unsafe extern "system" fn RegistrationState<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPendingRegistrationWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegistrationState: RegistrationState::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            IsPendingRegistrationWithAU: IsPendingRegistrationWithAU::<Identity, Impl, OFFSET>,
            Service: Service::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceRegistration as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReadOnly(&self) -> ::windows::core::Result<i16>;
    fn WebProxy(&self) -> ::windows::core::Result<IWebProxy>;
    fn SetWebProxy(&self, value: &::core::option::Option<IWebProxy>) -> ::windows::core::Result<()>;
    fn CreateUpdateSearcher(&self) -> ::windows::core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(&self) -> ::windows::core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(&self) -> ::windows::core::Result<IUpdateInstaller>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>() -> IUpdateSession_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxy<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WebProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxy<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWebProxy(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CreateUpdateSearcher<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateUpdateSearcher() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateDownloader<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateUpdateDownloader() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateUpdateInstaller() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            WebProxy: WebProxy::<Identity, Impl, OFFSET>,
            SetWebProxy: SetWebProxy::<Identity, Impl, OFFSET>,
            CreateUpdateSearcher: CreateUpdateSearcher::<Identity, Impl, OFFSET>,
            CreateUpdateDownloader: CreateUpdateDownloader::<Identity, Impl, OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSession as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSession2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSession_Impl {
    fn UserLocale(&self) -> ::windows::core::Result<u32>;
    fn SetUserLocale(&self, lcid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession2_Impl, const OFFSET: isize>() -> IUpdateSession2_Vtbl {
        unsafe extern "system" fn UserLocale<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserLocale<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        Self {
            base: IUpdateSession_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserLocale: UserLocale::<Identity, Impl, OFFSET>,
            SetUserLocale: SetUserLocale::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSession2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSession3_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSession_Impl + IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(&self) -> ::windows::core::Result<IUpdateServiceManager2>;
    fn QueryHistory(&self, criteria: &super::super::Foundation::BSTR, startindex: i32, count: i32) -> ::windows::core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSession3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession3_Impl, const OFFSET: isize>() -> IUpdateSession3_Vtbl {
        unsafe extern "system" fn CreateUpdateServiceManager<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateUpdateServiceManager() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, startindex: i32, count: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryHistory(::core::mem::transmute(&criteria), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdateSession2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateUpdateServiceManager: CreateUpdateServiceManager::<Identity, Impl, OFFSET>,
            QueryHistory: QueryHistory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSession3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdateSession as ::windows::core::Interface>::IID || iid == &<IUpdateSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWebProxy_Impl: Sized + super::Com::IDispatch_Impl {
    fn Address(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAddress(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BypassList(&self) -> ::windows::core::Result<IStringCollection>;
    fn SetBypassList(&self, value: &::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
    fn BypassProxyOnLocal(&self) -> ::windows::core::Result<i16>;
    fn SetBypassProxyOnLocal(&self, value: i16) -> ::windows::core::Result<()>;
    fn ReadOnly(&self) -> ::windows::core::Result<i16>;
    fn UserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUserName(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPassword(&self, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PromptForCredentials(&self, parentwindow: &::core::option::Option<::windows::core::IUnknown>, title: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PromptForCredentialsFromHwnd(&self, parentwindow: super::super::Foundation::HWND, title: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AutoDetect(&self) -> ::windows::core::Result<i16>;
    fn SetAutoDetect(&self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWebProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>() -> IWebProxy_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAddress(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BypassList<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BypassList() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassList<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBypassList(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BypassProxyOnLocal<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BypassProxyOnLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBypassProxyOnLocal(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPassword(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PromptForCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwindow: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PromptForCredentials(::core::mem::transmute(&parentwindow), ::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PromptForCredentialsFromHwnd(::core::mem::transmute_copy(&parentwindow), ::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn AutoDetect<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoDetect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetect<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoDetect(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            SetAddress: SetAddress::<Identity, Impl, OFFSET>,
            BypassList: BypassList::<Identity, Impl, OFFSET>,
            SetBypassList: SetBypassList::<Identity, Impl, OFFSET>,
            BypassProxyOnLocal: BypassProxyOnLocal::<Identity, Impl, OFFSET>,
            SetBypassProxyOnLocal: SetBypassProxyOnLocal::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            UserName: UserName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            PromptForCredentials: PromptForCredentials::<Identity, Impl, OFFSET>,
            PromptForCredentialsFromHwnd: PromptForCredentialsFromHwnd::<Identity, Impl, OFFSET>,
            AutoDetect: AutoDetect::<Identity, Impl, OFFSET>,
            SetAutoDetect: SetAutoDetect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProxy as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl {
    fn DriverClass(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverHardwareID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverManufacturer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverModel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverProvider(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverVerDate(&self) -> ::windows::core::Result<f64>;
    fn DeviceProblemNumber(&self) -> ::windows::core::Result<i32>;
    fn DeviceStatus(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate_Vtbl {
        unsafe extern "system" fn DriverClass<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverClass() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverHardwareID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverModel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverVerDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceProblemNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdate_Vtbl::new::<Identity, Impl, OFFSET>(),
            DriverClass: DriverClass::<Identity, Impl, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, Impl, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, Impl, OFFSET>,
            DriverModel: DriverModel::<Identity, Impl, OFFSET>,
            DriverProvider: DriverProvider::<Identity, Impl, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, Impl, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, Impl, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate2_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl {
    fn RebootRequired(&self) -> ::windows::core::Result<i16>;
    fn IsPresent(&self) -> ::windows::core::Result<i16>;
    fn CveIDs(&self) -> ::windows::core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: &::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate2_Vtbl {
        unsafe extern "system" fn RebootRequired<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CveIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiles: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyToCache(::core::mem::transmute(&pfiles)).into()
        }
        Self {
            base: IWindowsDriverUpdate_Vtbl::new::<Identity, Impl, OFFSET>(),
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            IsPresent: IsPresent::<Identity, Impl, OFFSET>,
            CveIDs: CveIDs::<Identity, Impl, OFFSET>,
            CopyToCache: CopyToCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate3_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl + IWindowsDriverUpdate2_Impl {
    fn BrowseOnly(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate3_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate3_Vtbl {
        unsafe extern "system" fn BrowseOnly<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BrowseOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWindowsDriverUpdate2_Vtbl::new::<Identity, Impl, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate4_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl + IWindowsDriverUpdate2_Impl + IWindowsDriverUpdate3_Impl {
    fn WindowsDriverUpdateEntries(&self) -> ::windows::core::Result<IWindowsDriverUpdateEntryCollection>;
    fn PerUser(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate4_Vtbl {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowsDriverUpdateEntries() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerUser<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWindowsDriverUpdate3_Vtbl::new::<Identity, Impl, OFFSET>(),
            WindowsDriverUpdateEntries: WindowsDriverUpdateEntries::<Identity, Impl, OFFSET>,
            PerUser: PerUser::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate4 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate5_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl + IWindowsDriverUpdate2_Impl + IWindowsDriverUpdate3_Impl + IWindowsDriverUpdate4_Impl {
    fn AutoSelection(&self) -> ::windows::core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> ::windows::core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate5_Vtbl {
        unsafe extern "system" fn AutoSelection<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWindowsDriverUpdate4_Vtbl::new::<Identity, Impl, OFFSET>(),
            AutoSelection: AutoSelection::<Identity, Impl, OFFSET>,
            AutoDownload: AutoDownload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate5 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate2 as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate3 as ::windows::core::Interface>::IID || iid == &<IWindowsDriverUpdate4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdateEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn DriverClass(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverHardwareID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverManufacturer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverModel(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverProvider(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverVerDate(&self) -> ::windows::core::Result<f64>;
    fn DeviceProblemNumber(&self) -> ::windows::core::Result<i32>;
    fn DeviceStatus(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdateEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>() -> IWindowsDriverUpdateEntry_Vtbl {
        unsafe extern "system" fn DriverClass<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverClass() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverHardwareID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverModel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DriverVerDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceProblemNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DriverClass: DriverClass::<Identity, Impl, OFFSET>,
            DriverHardwareID: DriverHardwareID::<Identity, Impl, OFFSET>,
            DriverManufacturer: DriverManufacturer::<Identity, Impl, OFFSET>,
            DriverModel: DriverModel::<Identity, Impl, OFFSET>,
            DriverProvider: DriverProvider::<Identity, Impl, OFFSET>,
            DriverVerDate: DriverVerDate::<Identity, Impl, OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Identity, Impl, OFFSET>,
            DeviceStatus: DeviceStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntry as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdateEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: i32) -> ::windows::core::Result<IWindowsDriverUpdateEntry>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdateEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>() -> IWindowsDriverUpdateEntryCollection_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntryCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsUpdateAgentInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetInfo(&self, varinfoidentifier: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsUpdateAgentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>() -> IWindowsUpdateAgentInfo_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinfoidentifier: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute(&varinfoidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetInfo: GetInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsUpdateAgentInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}

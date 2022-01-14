#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdates_Impl: Sized + super::Com::IDispatch_Impl {
    fn DetectNow(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn ShowSettingsDialog(&mut self) -> ::windows::core::Result<()>;
    fn Settings(&mut self) -> ::windows::core::Result<IAutomaticUpdatesSettings>;
    fn ServiceEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn EnableService(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdates_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomaticUpdates_Vtbl {
        unsafe extern "system" fn DetectNow<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectNow().into()
        }
        unsafe extern "system" fn Pause<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn ShowSettingsDialog<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowSettingsDialog().into()
        }
        unsafe extern "system" fn Settings<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceEnabled<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableService<Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableService().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DetectNow: DetectNow::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
            ShowSettingsDialog: ShowSettingsDialog::<Impl, IMPL_OFFSET>,
            Settings: Settings::<Impl, IMPL_OFFSET>,
            ServiceEnabled: ServiceEnabled::<Impl, IMPL_OFFSET>,
            EnableService: EnableService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdates as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdates2_Impl: Sized + super::Com::IDispatch_Impl + IAutomaticUpdates_Impl {
    fn Results(&mut self) -> ::windows::core::Result<IAutomaticUpdatesResults>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdates2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdates2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomaticUpdates2_Vtbl {
        unsafe extern "system" fn Results<Impl: IAutomaticUpdates2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Results() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAutomaticUpdates_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Results: Results::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdates2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesResults_Impl: Sized + super::Com::IDispatch_Impl {
    fn LastSearchSuccessDate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn LastInstallationSuccessDate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesResults_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomaticUpdatesResults_Vtbl {
        unsafe extern "system" fn LastSearchSuccessDate<Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSearchSuccessDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastInstallationSuccessDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LastSearchSuccessDate: LastSearchSuccessDate::<Impl, IMPL_OFFSET>,
            LastInstallationSuccessDate: LastInstallationSuccessDate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesResults as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn NotificationLevel(&mut self) -> ::windows::core::Result<AutomaticUpdatesNotificationLevel>;
    fn SetNotificationLevel(&mut self, value: AutomaticUpdatesNotificationLevel) -> ::windows::core::Result<()>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn Required(&mut self) -> ::windows::core::Result<i16>;
    fn ScheduledInstallationDay(&mut self) -> ::windows::core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn SetScheduledInstallationDay(&mut self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::Result<()>;
    fn ScheduledInstallationTime(&mut self) -> ::windows::core::Result<i32>;
    fn SetScheduledInstallationTime(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Save(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomaticUpdatesSettings_Vtbl {
        unsafe extern "system" fn NotificationLevel<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationLevel<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationLevel(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Required<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Required() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledInstallationDay<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledInstallationDay() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScheduledInstallationDay(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ScheduledInstallationTime<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScheduledInstallationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScheduledInstallationTime(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Refresh<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Save<Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NotificationLevel: NotificationLevel::<Impl, IMPL_OFFSET>,
            SetNotificationLevel: SetNotificationLevel::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            Required: Required::<Impl, IMPL_OFFSET>,
            ScheduledInstallationDay: ScheduledInstallationDay::<Impl, IMPL_OFFSET>,
            SetScheduledInstallationDay: SetScheduledInstallationDay::<Impl, IMPL_OFFSET>,
            ScheduledInstallationTime: ScheduledInstallationTime::<Impl, IMPL_OFFSET>,
            SetScheduledInstallationTime: SetScheduledInstallationTime::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesSettings2_Impl: Sized + super::Com::IDispatch_Impl + IAutomaticUpdatesSettings_Impl {
    fn IncludeRecommendedUpdates(&mut self) -> ::windows::core::Result<i16>;
    fn SetIncludeRecommendedUpdates(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn CheckPermission(&mut self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomaticUpdatesSettings2_Vtbl {
        unsafe extern "system" fn IncludeRecommendedUpdates<Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeRecommendedUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeRecommendedUpdates(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CheckPermission<Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckPermission(::core::mem::transmute_copy(&usertype), ::core::mem::transmute_copy(&permissiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    *userhaspermission = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAutomaticUpdatesSettings_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IncludeRecommendedUpdates: IncludeRecommendedUpdates::<Impl, IMPL_OFFSET>,
            SetIncludeRecommendedUpdates: SetIncludeRecommendedUpdates::<Impl, IMPL_OFFSET>,
            CheckPermission: CheckPermission::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAutomaticUpdatesSettings3_Impl: Sized + super::Com::IDispatch_Impl + IAutomaticUpdatesSettings_Impl + IAutomaticUpdatesSettings2_Impl {
    fn NonAdministratorsElevated(&mut self) -> ::windows::core::Result<i16>;
    fn SetNonAdministratorsElevated(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn FeaturedUpdatesEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetFeaturedUpdatesEnabled(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAutomaticUpdatesSettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomaticUpdatesSettings3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomaticUpdatesSettings3_Vtbl {
        unsafe extern "system" fn NonAdministratorsElevated<Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonAdministratorsElevated() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNonAdministratorsElevated(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeaturedUpdatesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFeaturedUpdatesEnabled(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IAutomaticUpdatesSettings2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NonAdministratorsElevated: NonAdministratorsElevated::<Impl, IMPL_OFFSET>,
            SetNonAdministratorsElevated: SetNonAdministratorsElevated::<Impl, IMPL_OFFSET>,
            FeaturedUpdatesEnabled: FeaturedUpdatesEnabled::<Impl, IMPL_OFFSET>,
            SetFeaturedUpdatesEnabled: SetFeaturedUpdatesEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICategory_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CategoryID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Children(&mut self) -> ::windows::core::Result<ICategoryCollection>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Image(&mut self) -> ::windows::core::Result<IImageInformation>;
    fn Order(&mut self) -> ::windows::core::Result<i32>;
    fn Parent(&mut self) -> ::windows::core::Result<ICategory>;
    fn Type(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Updates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICategory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICategory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICategory_Vtbl {
        unsafe extern "system" fn Name<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CategoryID<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CategoryID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Order<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Order() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            CategoryID: CategoryID::<Impl, IMPL_OFFSET>,
            Children: Children::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            Order: Order::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            Updates: Updates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICategory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICategoryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<ICategory>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICategoryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICategoryCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICategoryCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICategoryCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallback_Impl: Sized {
    fn Invoke(&mut self, downloadjob: ::core::option::Option<IDownloadJob>, callbackargs: ::core::option::Option<IDownloadCompletedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadCompletedCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IDownloadCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&downloadjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadCompletedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadCompletedCallbackArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadCompletedCallbackArgs_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadCompletedCallbackArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsCompleted(&mut self) -> ::windows::core::Result<i16>;
    fn Updates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn CleanUp(&mut self) -> ::windows::core::Result<()>;
    fn GetProgress(&mut self) -> ::windows::core::Result<IDownloadProgress>;
    fn RequestAbort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadJob_Vtbl {
        unsafe extern "system" fn AsyncState<Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CleanUp().into()
        }
        unsafe extern "system" fn GetProgress<Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAbort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AsyncState: AsyncState::<Impl, IMPL_OFFSET>,
            IsCompleted: IsCompleted::<Impl, IMPL_OFFSET>,
            Updates: Updates::<Impl, IMPL_OFFSET>,
            CleanUp: CleanUp::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
            RequestAbort: RequestAbort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateBytesDownloaded(&mut self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateBytesToDownload(&mut self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateIndex(&mut self) -> ::windows::core::Result<i32>;
    fn PercentComplete(&mut self) -> ::windows::core::Result<i32>;
    fn TotalBytesDownloaded(&mut self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn TotalBytesToDownload(&mut self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn GetUpdateResult(&mut self, updateindex: i32) -> ::windows::core::Result<IUpdateDownloadResult>;
    fn CurrentUpdateDownloadPhase(&mut self) -> ::windows::core::Result<DownloadPhase>;
    fn CurrentUpdatePercentComplete(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadProgress_Vtbl {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateBytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateBytesToDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateIndex<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesDownloaded<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalBytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesToDownload<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalBytesToDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPhase) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateDownloadPhase() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdatePercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentUpdateBytesDownloaded: CurrentUpdateBytesDownloaded::<Impl, IMPL_OFFSET>,
            CurrentUpdateBytesToDownload: CurrentUpdateBytesToDownload::<Impl, IMPL_OFFSET>,
            CurrentUpdateIndex: CurrentUpdateIndex::<Impl, IMPL_OFFSET>,
            PercentComplete: PercentComplete::<Impl, IMPL_OFFSET>,
            TotalBytesDownloaded: TotalBytesDownloaded::<Impl, IMPL_OFFSET>,
            TotalBytesToDownload: TotalBytesToDownload::<Impl, IMPL_OFFSET>,
            GetUpdateResult: GetUpdateResult::<Impl, IMPL_OFFSET>,
            CurrentUpdateDownloadPhase: CurrentUpdateDownloadPhase::<Impl, IMPL_OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallback_Impl: Sized {
    fn Invoke(&mut self, downloadjob: ::core::option::Option<IDownloadJob>, callbackargs: ::core::option::Option<IDownloadProgressChangedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressChangedCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadProgressChangedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&downloadjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&mut self) -> ::windows::core::Result<IDownloadProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadProgressChangedCallbackArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadProgressChangedCallbackArgs_Vtbl {
        unsafe extern "system" fn Progress<Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Progress: Progress::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallbackArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&mut self) -> ::windows::core::Result<i32>;
    fn ResultCode(&mut self) -> ::windows::core::Result<OperationResultCode>;
    fn GetUpdateResult(&mut self, updateindex: i32) -> ::windows::core::Result<IUpdateDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadResult_Vtbl {
        unsafe extern "system" fn HResult<Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HResult: HResult::<Impl, IMPL_OFFSET>,
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
            GetUpdateResult: GetUpdateResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IImageInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn AltText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Height(&mut self) -> ::windows::core::Result<i32>;
    fn Source(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IImageInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageInformation_Vtbl {
        unsafe extern "system" fn AltText<Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AltText: AltText::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationAgent_Impl: Sized + super::Com::IDispatch_Impl {
    fn RecordInstallationResult(&mut self, installationresultcookie: super::super::Foundation::BSTR, hresult: i32, extendedreportingdata: ::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationAgent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationAgent_Vtbl {
        unsafe extern "system" fn RecordInstallationResult<Impl: IInstallationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationresultcookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hresult: i32, extendedreportingdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordInstallationResult(::core::mem::transmute_copy(&installationresultcookie), ::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&extendedreportingdata)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RecordInstallationResult: RecordInstallationResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationBehavior_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanRequestUserInput(&mut self) -> ::windows::core::Result<i16>;
    fn Impact(&mut self) -> ::windows::core::Result<InstallationImpact>;
    fn RebootBehavior(&mut self) -> ::windows::core::Result<InstallationRebootBehavior>;
    fn RequiresNetworkConnectivity(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationBehavior_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationBehavior_Vtbl {
        unsafe extern "system" fn CanRequestUserInput<Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRequestUserInput() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Impact<Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationImpact) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Impact() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootBehavior<Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiresNetworkConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CanRequestUserInput: CanRequestUserInput::<Impl, IMPL_OFFSET>,
            Impact: Impact::<Impl, IMPL_OFFSET>,
            RebootBehavior: RebootBehavior::<Impl, IMPL_OFFSET>,
            RequiresNetworkConnectivity: RequiresNetworkConnectivity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallback_Impl: Sized {
    fn Invoke(&mut self, installationjob: ::core::option::Option<IInstallationJob>, callbackargs: ::core::option::Option<IInstallationCompletedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationCompletedCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IInstallationCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&installationjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationCompletedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationCompletedCallbackArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationCompletedCallbackArgs_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationCompletedCallbackArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsCompleted(&mut self) -> ::windows::core::Result<i16>;
    fn Updates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn CleanUp(&mut self) -> ::windows::core::Result<()>;
    fn GetProgress(&mut self) -> ::windows::core::Result<IInstallationProgress>;
    fn RequestAbort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationJob_Vtbl {
        unsafe extern "system" fn AsyncState<Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CleanUp().into()
        }
        unsafe extern "system" fn GetProgress<Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAbort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AsyncState: AsyncState::<Impl, IMPL_OFFSET>,
            IsCompleted: IsCompleted::<Impl, IMPL_OFFSET>,
            Updates: Updates::<Impl, IMPL_OFFSET>,
            CleanUp: CleanUp::<Impl, IMPL_OFFSET>,
            GetProgress: GetProgress::<Impl, IMPL_OFFSET>,
            RequestAbort: RequestAbort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateIndex(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentUpdatePercentComplete(&mut self) -> ::windows::core::Result<i32>;
    fn PercentComplete(&mut self) -> ::windows::core::Result<i32>;
    fn GetUpdateResult(&mut self, updateindex: i32) -> ::windows::core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationProgress_Vtbl {
        unsafe extern "system" fn CurrentUpdateIndex<Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentUpdatePercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentUpdateIndex: CurrentUpdateIndex::<Impl, IMPL_OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Impl, IMPL_OFFSET>,
            PercentComplete: PercentComplete::<Impl, IMPL_OFFSET>,
            GetUpdateResult: GetUpdateResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallback_Impl: Sized {
    fn Invoke(&mut self, installationjob: ::core::option::Option<IInstallationJob>, callbackargs: ::core::option::Option<IInstallationProgressChangedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgressChangedCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationProgressChangedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&installationjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&mut self) -> ::windows::core::Result<IInstallationProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationProgressChangedCallbackArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationProgressChangedCallbackArgs_Vtbl {
        unsafe extern "system" fn Progress<Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Progress: Progress::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallbackArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&mut self) -> ::windows::core::Result<i32>;
    fn RebootRequired(&mut self) -> ::windows::core::Result<i16>;
    fn ResultCode(&mut self) -> ::windows::core::Result<OperationResultCode>;
    fn GetUpdateResult(&mut self, updateindex: i32) -> ::windows::core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInstallationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInstallationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInstallationResult_Vtbl {
        unsafe extern "system" fn HResult<Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HResult: HResult::<Impl, IMPL_OFFSET>,
            RebootRequired: RebootRequired::<Impl, IMPL_OFFSET>,
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
            GetUpdateResult: GetUpdateResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInstallationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IInvalidProductLicenseException_Impl: Sized + super::Com::IDispatch_Impl + IUpdateException_Impl {
    fn Product(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IInvalidProductLicenseException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvalidProductLicenseException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInvalidProductLicenseException_Vtbl {
        unsafe extern "system" fn Product<Impl: IInvalidProductLicenseException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Product() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateException_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Product: Product::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvalidProductLicenseException as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallback_Impl: Sized {
    fn Invoke(&mut self, searchjob: ::core::option::Option<ISearchJob>, callbackargs: ::core::option::Option<ISearchCompletedCallbackArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCompletedCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Impl: ISearchCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke(::core::mem::transmute(&searchjob), ::core::mem::transmute(&callbackargs)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCompletedCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISearchCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISearchCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchCompletedCallbackArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchCompletedCallbackArgs_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchCompletedCallbackArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISearchJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsCompleted(&mut self) -> ::windows::core::Result<i16>;
    fn CleanUp(&mut self) -> ::windows::core::Result<()>;
    fn RequestAbort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISearchJob_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchJob_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchJob_Vtbl {
        unsafe extern "system" fn AsyncState<Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CleanUp().into()
        }
        unsafe extern "system" fn RequestAbort<Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestAbort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AsyncState: AsyncState::<Impl, IMPL_OFFSET>,
            IsCompleted: IsCompleted::<Impl, IMPL_OFFSET>,
            CleanUp: CleanUp::<Impl, IMPL_OFFSET>,
            RequestAbort: RequestAbort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchJob as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISearchResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn ResultCode(&mut self) -> ::windows::core::Result<OperationResultCode>;
    fn RootCategories(&mut self) -> ::windows::core::Result<ICategoryCollection>;
    fn Updates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn Warnings(&mut self) -> ::windows::core::Result<IUpdateExceptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISearchResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchResult_Vtbl {
        unsafe extern "system" fn ResultCode<Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootCategories<Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootCategories() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Warnings<Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Warnings() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
            RootCategories: RootCategories::<Impl, IMPL_OFFSET>,
            Updates: Updates::<Impl, IMPL_OFFSET>,
            Warnings: Warnings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IStringCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetItem(&mut self, index: i32, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn Add(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Copy(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn Insert(&mut self, index: i32, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IStringCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStringCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItem(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Copy<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            SetItem: SetItem::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStringCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn OemHardwareSupportLink(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RebootRequired(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemInformation_Vtbl {
        unsafe extern "system" fn OemHardwareSupportLink<Impl: ISystemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OemHardwareSupportLink() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Impl: ISystemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OemHardwareSupportLink: OemHardwareSupportLink::<Impl, IMPL_OFFSET>,
            RebootRequired: RebootRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate_Impl: Sized + super::Com::IDispatch_Impl {
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AutoSelectOnWebSites(&mut self) -> ::windows::core::Result<i16>;
    fn BundledUpdates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn CanRequireSource(&mut self) -> ::windows::core::Result<i16>;
    fn Categories(&mut self) -> ::windows::core::Result<ICategoryCollection>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn DeltaCompressedContentAvailable(&mut self) -> ::windows::core::Result<i16>;
    fn DeltaCompressedContentPreferred(&mut self) -> ::windows::core::Result<i16>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EulaAccepted(&mut self) -> ::windows::core::Result<i16>;
    fn EulaText(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HandlerID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Identity(&mut self) -> ::windows::core::Result<IUpdateIdentity>;
    fn Image(&mut self) -> ::windows::core::Result<IImageInformation>;
    fn InstallationBehavior(&mut self) -> ::windows::core::Result<IInstallationBehavior>;
    fn IsBeta(&mut self) -> ::windows::core::Result<i16>;
    fn IsDownloaded(&mut self) -> ::windows::core::Result<i16>;
    fn IsHidden(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsHidden(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn IsInstalled(&mut self) -> ::windows::core::Result<i16>;
    fn IsMandatory(&mut self) -> ::windows::core::Result<i16>;
    fn IsUninstallable(&mut self) -> ::windows::core::Result<i16>;
    fn Languages(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn LastDeploymentChangeTime(&mut self) -> ::windows::core::Result<f64>;
    fn MaxDownloadSize(&mut self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn MinDownloadSize(&mut self) -> ::windows::core::Result<super::super::Foundation::DECIMAL>;
    fn MoreInfoUrls(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn MsrcSeverity(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RecommendedCpuSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RecommendedHardDiskSpace(&mut self) -> ::windows::core::Result<i32>;
    fn RecommendedMemory(&mut self) -> ::windows::core::Result<i32>;
    fn ReleaseNotes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SecurityBulletinIDs(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn SupersededUpdateIDs(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn SupportUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Type(&mut self) -> ::windows::core::Result<UpdateType>;
    fn UninstallationNotes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UninstallationBehavior(&mut self) -> ::windows::core::Result<IInstallationBehavior>;
    fn UninstallationSteps(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn KBArticleIDs(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn AcceptEula(&mut self) -> ::windows::core::Result<()>;
    fn DeploymentAction(&mut self) -> ::windows::core::Result<DeploymentAction>;
    fn CopyFromCache(&mut self, path: super::super::Foundation::BSTR, toextractcabfiles: i16) -> ::windows::core::Result<()>;
    fn DownloadPriority(&mut self) -> ::windows::core::Result<DownloadPriority>;
    fn DownloadContents(&mut self) -> ::windows::core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdate_Vtbl {
        unsafe extern "system" fn Title<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoSelectOnWebSites() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BundledUpdates<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BundledUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequireSource<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRequireSource() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Categories<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Categories() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaCompressedContentAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeltaCompressedContentPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaAccepted<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EulaAccepted() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaText<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EulaText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandlerID<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandlerID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identity<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallationBehavior<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBeta<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBeta() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloaded<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHidden(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IsInstalled<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMandatory<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMandatory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Languages() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastDeploymentChangeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDownloadSize<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDownloadSize<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreInfoUrls<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoreInfoUrls() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsrcSeverity<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MsrcSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendedCpuSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendedHardDiskSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedMemory<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendedMemory() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseNotes<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseNotes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityBulletinIDs<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityBulletinIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupersededUpdateIDs<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupersededUpdateIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallationNotes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationBehavior<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallationSteps() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KBArticleIDs<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KBArticleIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptEula<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptEula().into()
        }
        unsafe extern "system" fn DeploymentAction<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DeploymentAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeploymentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromCache<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, toextractcabfiles: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyFromCache(::core::mem::transmute_copy(&path), ::core::mem::transmute_copy(&toextractcabfiles)).into()
        }
        unsafe extern "system" fn DownloadPriority<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadContents<Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadContents() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            AutoSelectOnWebSites: AutoSelectOnWebSites::<Impl, IMPL_OFFSET>,
            BundledUpdates: BundledUpdates::<Impl, IMPL_OFFSET>,
            CanRequireSource: CanRequireSource::<Impl, IMPL_OFFSET>,
            Categories: Categories::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
            DeltaCompressedContentAvailable: DeltaCompressedContentAvailable::<Impl, IMPL_OFFSET>,
            DeltaCompressedContentPreferred: DeltaCompressedContentPreferred::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            EulaAccepted: EulaAccepted::<Impl, IMPL_OFFSET>,
            EulaText: EulaText::<Impl, IMPL_OFFSET>,
            HandlerID: HandlerID::<Impl, IMPL_OFFSET>,
            Identity: Identity::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            InstallationBehavior: InstallationBehavior::<Impl, IMPL_OFFSET>,
            IsBeta: IsBeta::<Impl, IMPL_OFFSET>,
            IsDownloaded: IsDownloaded::<Impl, IMPL_OFFSET>,
            IsHidden: IsHidden::<Impl, IMPL_OFFSET>,
            SetIsHidden: SetIsHidden::<Impl, IMPL_OFFSET>,
            IsInstalled: IsInstalled::<Impl, IMPL_OFFSET>,
            IsMandatory: IsMandatory::<Impl, IMPL_OFFSET>,
            IsUninstallable: IsUninstallable::<Impl, IMPL_OFFSET>,
            Languages: Languages::<Impl, IMPL_OFFSET>,
            LastDeploymentChangeTime: LastDeploymentChangeTime::<Impl, IMPL_OFFSET>,
            MaxDownloadSize: MaxDownloadSize::<Impl, IMPL_OFFSET>,
            MinDownloadSize: MinDownloadSize::<Impl, IMPL_OFFSET>,
            MoreInfoUrls: MoreInfoUrls::<Impl, IMPL_OFFSET>,
            MsrcSeverity: MsrcSeverity::<Impl, IMPL_OFFSET>,
            RecommendedCpuSpeed: RecommendedCpuSpeed::<Impl, IMPL_OFFSET>,
            RecommendedHardDiskSpace: RecommendedHardDiskSpace::<Impl, IMPL_OFFSET>,
            RecommendedMemory: RecommendedMemory::<Impl, IMPL_OFFSET>,
            ReleaseNotes: ReleaseNotes::<Impl, IMPL_OFFSET>,
            SecurityBulletinIDs: SecurityBulletinIDs::<Impl, IMPL_OFFSET>,
            SupersededUpdateIDs: SupersededUpdateIDs::<Impl, IMPL_OFFSET>,
            SupportUrl: SupportUrl::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            UninstallationNotes: UninstallationNotes::<Impl, IMPL_OFFSET>,
            UninstallationBehavior: UninstallationBehavior::<Impl, IMPL_OFFSET>,
            UninstallationSteps: UninstallationSteps::<Impl, IMPL_OFFSET>,
            KBArticleIDs: KBArticleIDs::<Impl, IMPL_OFFSET>,
            AcceptEula: AcceptEula::<Impl, IMPL_OFFSET>,
            DeploymentAction: DeploymentAction::<Impl, IMPL_OFFSET>,
            CopyFromCache: CopyFromCache::<Impl, IMPL_OFFSET>,
            DownloadPriority: DownloadPriority::<Impl, IMPL_OFFSET>,
            DownloadContents: DownloadContents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate2_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl {
    fn RebootRequired(&mut self) -> ::windows::core::Result<i16>;
    fn IsPresent(&mut self) -> ::windows::core::Result<i16>;
    fn CveIDs(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn CopyToCache(&mut self, pfiles: ::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdate2_Vtbl {
        unsafe extern "system" fn RebootRequired<Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CveIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiles: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyToCache(::core::mem::transmute(&pfiles)).into()
        }
        Self {
            base: IUpdate_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RebootRequired: RebootRequired::<Impl, IMPL_OFFSET>,
            IsPresent: IsPresent::<Impl, IMPL_OFFSET>,
            CveIDs: CveIDs::<Impl, IMPL_OFFSET>,
            CopyToCache: CopyToCache::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate3_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IUpdate2_Impl {
    fn BrowseOnly(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdate3_Vtbl {
        unsafe extern "system" fn BrowseOnly<Impl: IUpdate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrowseOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdate2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BrowseOnly: BrowseOnly::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate4_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IUpdate2_Impl + IUpdate3_Impl {
    fn PerUser(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdate4_Vtbl {
        unsafe extern "system" fn PerUser<Impl: IUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdate3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), PerUser: PerUser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdate5_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IUpdate2_Impl + IUpdate3_Impl + IUpdate4_Impl {
    fn AutoSelection(&mut self) -> ::windows::core::Result<AutoSelectionMode>;
    fn AutoDownload(&mut self) -> ::windows::core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdate5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdate5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdate5_Vtbl {
        unsafe extern "system" fn AutoSelection<Impl: IUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Impl: IUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdate4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AutoSelection: AutoSelection::<Impl, IMPL_OFFSET>,
            AutoDownload: AutoDownload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdate5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IUpdate>;
    fn SetItem(&mut self, index: i32, value: ::core::option::Option<IUpdate>) -> ::windows::core::Result<()>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn Add(&mut self, value: ::core::option::Option<IUpdate>) -> ::windows::core::Result<i32>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn Copy(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn Insert(&mut self, index: i32, value: ::core::option::Option<IUpdate>) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItem(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn Copy<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            SetItem: SetItem::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Insert: Insert::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn DownloadUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateDownloadContent_Vtbl {
        unsafe extern "system" fn DownloadUrl<Impl: IUpdateDownloadContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DownloadUrl: DownloadUrl::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadContent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadContent2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateDownloadContent_Impl {
    fn IsDeltaCompressedContent(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadContent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateDownloadContent2_Vtbl {
        unsafe extern "system" fn IsDeltaCompressedContent<Impl: IUpdateDownloadContent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeltaCompressedContent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdateDownloadContent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsDeltaCompressedContent: IsDeltaCompressedContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadContent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadContentCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IUpdateDownloadContent>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadContentCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadContentCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateDownloadContentCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadContentCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&mut self) -> ::windows::core::Result<i32>;
    fn ResultCode(&mut self) -> ::windows::core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloadResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateDownloadResult_Vtbl {
        unsafe extern "system" fn HResult<Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HResult: HResult::<Impl, IMPL_OFFSET>,
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloadResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateDownloader_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsForced(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsForced(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn Priority(&mut self) -> ::windows::core::Result<DownloadPriority>;
    fn SetPriority(&mut self, value: DownloadPriority) -> ::windows::core::Result<()>;
    fn Updates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn SetUpdates(&mut self, value: ::core::option::Option<IUpdateCollection>) -> ::windows::core::Result<()>;
    fn BeginDownload(&mut self, onprogresschanged: ::core::option::Option<::windows::core::IUnknown>, oncompleted: ::core::option::Option<::windows::core::IUnknown>, state: super::Com::VARIANT) -> ::windows::core::Result<IDownloadJob>;
    fn Download(&mut self) -> ::windows::core::Result<IDownloadResult>;
    fn EndDownload(&mut self, value: ::core::option::Option<IDownloadJob>) -> ::windows::core::Result<IDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateDownloader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateDownloader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateDownloader_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IsForced<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsForced() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsForced(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Priority<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Updates<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdates(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BeginDownload<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginDownload(::core::mem::transmute(&onprogresschanged), ::core::mem::transmute(&oncompleted), ::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Download() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDownload<Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDownload(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Impl, IMPL_OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Impl, IMPL_OFFSET>,
            IsForced: IsForced::<Impl, IMPL_OFFSET>,
            SetIsForced: SetIsForced::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            Updates: Updates::<Impl, IMPL_OFFSET>,
            SetUpdates: SetUpdates::<Impl, IMPL_OFFSET>,
            BeginDownload: BeginDownload::<Impl, IMPL_OFFSET>,
            Download: Download::<Impl, IMPL_OFFSET>,
            EndDownload: EndDownload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateDownloader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateException_Impl: Sized + super::Com::IDispatch_Impl {
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn HResult(&mut self) -> ::windows::core::Result<i32>;
    fn Context(&mut self) -> ::windows::core::Result<UpdateExceptionContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateException_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateException_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateException_Vtbl {
        unsafe extern "system" fn Message<Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateExceptionContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            HResult: HResult::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateException as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateExceptionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IUpdateException>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateExceptionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateExceptionCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateExceptionCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateExceptionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateHistoryEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Operation(&mut self) -> ::windows::core::Result<UpdateOperation>;
    fn ResultCode(&mut self) -> ::windows::core::Result<OperationResultCode>;
    fn HResult(&mut self) -> ::windows::core::Result<i32>;
    fn Date(&mut self) -> ::windows::core::Result<f64>;
    fn UpdateIdentity(&mut self) -> ::windows::core::Result<IUpdateIdentity>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UnmappedResultCode(&mut self) -> ::windows::core::Result<i32>;
    fn ClientApplicationID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServerSelection(&mut self) -> ::windows::core::Result<ServerSelection>;
    fn ServiceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UninstallationSteps(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn UninstallationNotes(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SupportUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateHistoryEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateHistoryEntry_Vtbl {
        unsafe extern "system" fn Operation<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Date() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateIdentity<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmappedResultCode<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnmappedResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerSelection<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallationSteps() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UninstallationNotes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Operation: Operation::<Impl, IMPL_OFFSET>,
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
            HResult: HResult::<Impl, IMPL_OFFSET>,
            Date: Date::<Impl, IMPL_OFFSET>,
            UpdateIdentity: UpdateIdentity::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            UnmappedResultCode: UnmappedResultCode::<Impl, IMPL_OFFSET>,
            ClientApplicationID: ClientApplicationID::<Impl, IMPL_OFFSET>,
            ServerSelection: ServerSelection::<Impl, IMPL_OFFSET>,
            ServiceID: ServiceID::<Impl, IMPL_OFFSET>,
            UninstallationSteps: UninstallationSteps::<Impl, IMPL_OFFSET>,
            UninstallationNotes: UninstallationNotes::<Impl, IMPL_OFFSET>,
            SupportUrl: SupportUrl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateHistoryEntry2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateHistoryEntry_Impl {
    fn Categories(&mut self) -> ::windows::core::Result<ICategoryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateHistoryEntry2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntry2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateHistoryEntry2_Vtbl {
        unsafe extern "system" fn Categories<Impl: IUpdateHistoryEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Categories() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateHistoryEntry_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Categories: Categories::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateHistoryEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IUpdateHistoryEntry>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateHistoryEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateHistoryEntryCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateHistoryEntryCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateHistoryEntryCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateIdentity_Impl: Sized + super::Com::IDispatch_Impl {
    fn RevisionNumber(&mut self) -> ::windows::core::Result<i32>;
    fn UpdateID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateIdentity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateIdentity_Vtbl {
        unsafe extern "system" fn RevisionNumber<Impl: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevisionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateID<Impl: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RevisionNumber: RevisionNumber::<Impl, IMPL_OFFSET>,
            UpdateID: UpdateID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&mut self) -> ::windows::core::Result<i32>;
    fn RebootRequired(&mut self) -> ::windows::core::Result<i16>;
    fn ResultCode(&mut self) -> ::windows::core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstallationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstallationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateInstallationResult_Vtbl {
        unsafe extern "system" fn HResult<Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HResult() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            HResult: HResult::<Impl, IMPL_OFFSET>,
            RebootRequired: RebootRequired::<Impl, IMPL_OFFSET>,
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstallationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsForced(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsForced(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ParentHwnd(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn SetParentHwnd(&mut self, value: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn SetParentWindow(&mut self, value: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ParentWindow(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Updates(&mut self) -> ::windows::core::Result<IUpdateCollection>;
    fn SetUpdates(&mut self, value: ::core::option::Option<IUpdateCollection>) -> ::windows::core::Result<()>;
    fn BeginInstall(&mut self, onprogresschanged: ::core::option::Option<::windows::core::IUnknown>, oncompleted: ::core::option::Option<::windows::core::IUnknown>, state: super::Com::VARIANT) -> ::windows::core::Result<IInstallationJob>;
    fn BeginUninstall(&mut self, onprogresschanged: ::core::option::Option<::windows::core::IUnknown>, oncompleted: ::core::option::Option<::windows::core::IUnknown>, state: super::Com::VARIANT) -> ::windows::core::Result<IInstallationJob>;
    fn EndInstall(&mut self, value: ::core::option::Option<IInstallationJob>) -> ::windows::core::Result<IInstallationResult>;
    fn EndUninstall(&mut self, value: ::core::option::Option<IInstallationJob>) -> ::windows::core::Result<IInstallationResult>;
    fn Install(&mut self) -> ::windows::core::Result<IInstallationResult>;
    fn RunWizard(&mut self, dialogtitle: super::super::Foundation::BSTR) -> ::windows::core::Result<IInstallationResult>;
    fn IsBusy(&mut self) -> ::windows::core::Result<i16>;
    fn Uninstall(&mut self) -> ::windows::core::Result<IInstallationResult>;
    fn AllowSourcePrompts(&mut self) -> ::windows::core::Result<i16>;
    fn SetAllowSourcePrompts(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn RebootRequiredBeforeInstallation(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateInstaller_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IsForced<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsForced() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsForced(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentHwnd<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentHwnd<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentHwnd(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetParentWindow<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParentWindow(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdates(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BeginInstall<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginInstall(::core::mem::transmute(&onprogresschanged), ::core::mem::transmute(&oncompleted), ::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstall<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginUninstall(::core::mem::transmute(&onprogresschanged), ::core::mem::transmute(&oncompleted), ::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInstall<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndInstall(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUninstall<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndUninstall(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Install() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWizard<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dialogtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunWizard(::core::mem::transmute_copy(&dialogtitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBusy<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBusy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninstall<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uninstall() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSourcePrompts<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowSourcePrompts() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowSourcePrompts(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootRequiredBeforeInstallation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Impl, IMPL_OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Impl, IMPL_OFFSET>,
            IsForced: IsForced::<Impl, IMPL_OFFSET>,
            SetIsForced: SetIsForced::<Impl, IMPL_OFFSET>,
            ParentHwnd: ParentHwnd::<Impl, IMPL_OFFSET>,
            SetParentHwnd: SetParentHwnd::<Impl, IMPL_OFFSET>,
            SetParentWindow: SetParentWindow::<Impl, IMPL_OFFSET>,
            ParentWindow: ParentWindow::<Impl, IMPL_OFFSET>,
            Updates: Updates::<Impl, IMPL_OFFSET>,
            SetUpdates: SetUpdates::<Impl, IMPL_OFFSET>,
            BeginInstall: BeginInstall::<Impl, IMPL_OFFSET>,
            BeginUninstall: BeginUninstall::<Impl, IMPL_OFFSET>,
            EndInstall: EndInstall::<Impl, IMPL_OFFSET>,
            EndUninstall: EndUninstall::<Impl, IMPL_OFFSET>,
            Install: Install::<Impl, IMPL_OFFSET>,
            RunWizard: RunWizard::<Impl, IMPL_OFFSET>,
            IsBusy: IsBusy::<Impl, IMPL_OFFSET>,
            Uninstall: Uninstall::<Impl, IMPL_OFFSET>,
            AllowSourcePrompts: AllowSourcePrompts::<Impl, IMPL_OFFSET>,
            SetAllowSourcePrompts: SetAllowSourcePrompts::<Impl, IMPL_OFFSET>,
            RebootRequiredBeforeInstallation: RebootRequiredBeforeInstallation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateInstaller_Impl {
    fn ForceQuiet(&mut self) -> ::windows::core::Result<i16>;
    fn SetForceQuiet(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateInstaller2_Vtbl {
        unsafe extern "system" fn ForceQuiet<Impl: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceQuiet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceQuiet<Impl: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceQuiet(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateInstaller_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ForceQuiet: ForceQuiet::<Impl, IMPL_OFFSET>,
            SetForceQuiet: SetForceQuiet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller3_Impl: Sized + super::Com::IDispatch_Impl + IUpdateInstaller_Impl + IUpdateInstaller2_Impl {
    fn AttemptCloseAppsIfNecessary(&mut self) -> ::windows::core::Result<i16>;
    fn SetAttemptCloseAppsIfNecessary(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateInstaller3_Vtbl {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Impl: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttemptCloseAppsIfNecessary() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Impl: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttemptCloseAppsIfNecessary(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateInstaller2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AttemptCloseAppsIfNecessary: AttemptCloseAppsIfNecessary::<Impl, IMPL_OFFSET>,
            SetAttemptCloseAppsIfNecessary: SetAttemptCloseAppsIfNecessary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateInstaller4_Impl: Sized + super::Com::IDispatch_Impl + IUpdateInstaller_Impl + IUpdateInstaller2_Impl + IUpdateInstaller3_Impl {
    fn Commit(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateInstaller4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateInstaller4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateInstaller4_Vtbl {
        unsafe extern "system" fn Commit<Impl: IUpdateInstaller4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: IUpdateInstaller3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Commit: Commit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateInstaller4 as ::windows::core::Interface>::IID
    }
}
pub trait IUpdateLockdown_Impl: Sized {
    fn LockDown(&mut self, flags: i32) -> ::windows::core::Result<()>;
}
impl IUpdateLockdown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateLockdown_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateLockdown_Vtbl {
        unsafe extern "system" fn LockDown<Impl: IUpdateLockdown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockDown(::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LockDown: LockDown::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateLockdown as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSearcher_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanAutomaticallyUpgradeService(&mut self) -> ::windows::core::Result<i16>;
    fn SetCanAutomaticallyUpgradeService(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ClientApplicationID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IncludePotentiallySupersededUpdates(&mut self) -> ::windows::core::Result<i16>;
    fn SetIncludePotentiallySupersededUpdates(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ServerSelection(&mut self) -> ::windows::core::Result<ServerSelection>;
    fn SetServerSelection(&mut self, value: ServerSelection) -> ::windows::core::Result<()>;
    fn BeginSearch(&mut self, criteria: super::super::Foundation::BSTR, oncompleted: ::core::option::Option<::windows::core::IUnknown>, state: super::Com::VARIANT) -> ::windows::core::Result<ISearchJob>;
    fn EndSearch(&mut self, searchjob: ::core::option::Option<ISearchJob>) -> ::windows::core::Result<ISearchResult>;
    fn EscapeString(&mut self, unescaped: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn QueryHistory(&mut self, startindex: i32, count: i32) -> ::windows::core::Result<IUpdateHistoryEntryCollection>;
    fn Search(&mut self, criteria: super::super::Foundation::BSTR) -> ::windows::core::Result<ISearchResult>;
    fn Online(&mut self) -> ::windows::core::Result<i16>;
    fn SetOnline(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn GetTotalHistoryCount(&mut self) -> ::windows::core::Result<i32>;
    fn ServiceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetServiceID(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSearcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSearcher_Vtbl {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAutomaticallyUpgradeService() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanAutomaticallyUpgradeService(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludePotentiallySupersededUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludePotentiallySupersededUpdates(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ServerSelection<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerSelection<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerSelection(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BeginSearch<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSearch(::core::mem::transmute_copy(&criteria), ::core::mem::transmute(&oncompleted), ::core::mem::transmute_copy(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSearch<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchjob: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSearch(::core::mem::transmute(&searchjob)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeString<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unescaped: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeString(::core::mem::transmute_copy(&unescaped)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: i32, count: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHistory(::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Search<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Search(::core::mem::transmute_copy(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Online() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOnline<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOnline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTotalHistoryCount<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalHistoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceID<Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceID(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CanAutomaticallyUpgradeService: CanAutomaticallyUpgradeService::<Impl, IMPL_OFFSET>,
            SetCanAutomaticallyUpgradeService: SetCanAutomaticallyUpgradeService::<Impl, IMPL_OFFSET>,
            ClientApplicationID: ClientApplicationID::<Impl, IMPL_OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Impl, IMPL_OFFSET>,
            IncludePotentiallySupersededUpdates: IncludePotentiallySupersededUpdates::<Impl, IMPL_OFFSET>,
            SetIncludePotentiallySupersededUpdates: SetIncludePotentiallySupersededUpdates::<Impl, IMPL_OFFSET>,
            ServerSelection: ServerSelection::<Impl, IMPL_OFFSET>,
            SetServerSelection: SetServerSelection::<Impl, IMPL_OFFSET>,
            BeginSearch: BeginSearch::<Impl, IMPL_OFFSET>,
            EndSearch: EndSearch::<Impl, IMPL_OFFSET>,
            EscapeString: EscapeString::<Impl, IMPL_OFFSET>,
            QueryHistory: QueryHistory::<Impl, IMPL_OFFSET>,
            Search: Search::<Impl, IMPL_OFFSET>,
            Online: Online::<Impl, IMPL_OFFSET>,
            SetOnline: SetOnline::<Impl, IMPL_OFFSET>,
            GetTotalHistoryCount: GetTotalHistoryCount::<Impl, IMPL_OFFSET>,
            ServiceID: ServiceID::<Impl, IMPL_OFFSET>,
            SetServiceID: SetServiceID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSearcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSearcher2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSearcher_Impl {
    fn IgnoreDownloadPriority(&mut self) -> ::windows::core::Result<i16>;
    fn SetIgnoreDownloadPriority(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSearcher2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSearcher2_Vtbl {
        unsafe extern "system" fn IgnoreDownloadPriority<Impl: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnoreDownloadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Impl: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreDownloadPriority(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateSearcher_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IgnoreDownloadPriority: IgnoreDownloadPriority::<Impl, IMPL_OFFSET>,
            SetIgnoreDownloadPriority: SetIgnoreDownloadPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSearcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSearcher3_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSearcher_Impl + IUpdateSearcher2_Impl {
    fn SearchScope(&mut self) -> ::windows::core::Result<SearchScope>;
    fn SetSearchScope(&mut self, value: SearchScope) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSearcher3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSearcher3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSearcher3_Vtbl {
        unsafe extern "system" fn SearchScope<Impl: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut SearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchScope() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchScope<Impl: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchScope(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: IUpdateSearcher2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SearchScope: SearchScope::<Impl, IMPL_OFFSET>,
            SetSearchScope: SetSearchScope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSearcher3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateService_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ContentValidationCert(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ExpirationDate(&mut self) -> ::windows::core::Result<f64>;
    fn IsManaged(&mut self) -> ::windows::core::Result<i16>;
    fn IsRegisteredWithAU(&mut self) -> ::windows::core::Result<i16>;
    fn IssueDate(&mut self) -> ::windows::core::Result<f64>;
    fn OffersWindowsUpdates(&mut self) -> ::windows::core::Result<i16>;
    fn RedirectUrls(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn ServiceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsScanPackageService(&mut self) -> ::windows::core::Result<i16>;
    fn CanRegisterWithAU(&mut self) -> ::windows::core::Result<i16>;
    fn ServiceUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetupPrefix(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateService_Vtbl {
        unsafe extern "system" fn Name<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentValidationCert<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentValidationCert() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsManaged<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsManaged() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredWithAU<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRegisteredWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDate<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssueDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffersWindowsUpdates<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OffersWindowsUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectUrls<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectUrls() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanPackageService<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScanPackageService() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRegisterWithAU<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRegisterWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUrl<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupPrefix<Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetupPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            ContentValidationCert: ContentValidationCert::<Impl, IMPL_OFFSET>,
            ExpirationDate: ExpirationDate::<Impl, IMPL_OFFSET>,
            IsManaged: IsManaged::<Impl, IMPL_OFFSET>,
            IsRegisteredWithAU: IsRegisteredWithAU::<Impl, IMPL_OFFSET>,
            IssueDate: IssueDate::<Impl, IMPL_OFFSET>,
            OffersWindowsUpdates: OffersWindowsUpdates::<Impl, IMPL_OFFSET>,
            RedirectUrls: RedirectUrls::<Impl, IMPL_OFFSET>,
            ServiceID: ServiceID::<Impl, IMPL_OFFSET>,
            IsScanPackageService: IsScanPackageService::<Impl, IMPL_OFFSET>,
            CanRegisterWithAU: CanRegisterWithAU::<Impl, IMPL_OFFSET>,
            ServiceUrl: ServiceUrl::<Impl, IMPL_OFFSET>,
            SetupPrefix: SetupPrefix::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateService2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateService_Impl {
    fn IsDefaultAUService(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateService2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateService2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateService2_Vtbl {
        unsafe extern "system" fn IsDefaultAUService<Impl: IUpdateService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefaultAUService() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUpdateService_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), IsDefaultAUService: IsDefaultAUService::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IUpdateService>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateServiceCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Services(&mut self) -> ::windows::core::Result<IUpdateServiceCollection>;
    fn AddService(&mut self, serviceid: super::super::Foundation::BSTR, authorizationcabpath: super::super::Foundation::BSTR) -> ::windows::core::Result<IUpdateService>;
    fn RegisterServiceWithAU(&mut self, serviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveService(&mut self, serviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn UnregisterServiceWithAU(&mut self, serviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddScanPackageService(&mut self, servicename: super::super::Foundation::BSTR, scanfilelocation: super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<IUpdateService>;
    fn SetOption(&mut self, optionname: super::super::Foundation::BSTR, optionvalue: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateServiceManager_Vtbl {
        unsafe extern "system" fn Services<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddService(::core::mem::transmute_copy(&serviceid), ::core::mem::transmute_copy(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServiceWithAU<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterServiceWithAU(::core::mem::transmute_copy(&serviceid)).into()
        }
        unsafe extern "system" fn RemoveService<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveService(::core::mem::transmute_copy(&serviceid)).into()
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterServiceWithAU(::core::mem::transmute_copy(&serviceid)).into()
        }
        unsafe extern "system" fn AddScanPackageService<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scanfilelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddScanPackageService(::core::mem::transmute_copy(&servicename), ::core::mem::transmute_copy(&scanfilelocation), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOption(::core::mem::transmute_copy(&optionname), ::core::mem::transmute_copy(&optionvalue)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Services: Services::<Impl, IMPL_OFFSET>,
            AddService: AddService::<Impl, IMPL_OFFSET>,
            RegisterServiceWithAU: RegisterServiceWithAU::<Impl, IMPL_OFFSET>,
            RemoveService: RemoveService::<Impl, IMPL_OFFSET>,
            UnregisterServiceWithAU: UnregisterServiceWithAU::<Impl, IMPL_OFFSET>,
            AddScanPackageService: AddScanPackageService::<Impl, IMPL_OFFSET>,
            SetOption: SetOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceManager2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateServiceManager_Impl {
    fn ClientApplicationID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryServiceRegistration(&mut self, serviceid: super::super::Foundation::BSTR) -> ::windows::core::Result<IUpdateServiceRegistration>;
    fn AddService2(&mut self, serviceid: super::super::Foundation::BSTR, flags: i32, authorizationcabpath: super::super::Foundation::BSTR) -> ::windows::core::Result<IUpdateServiceRegistration>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateServiceManager2_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn QueryServiceRegistration<Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryServiceRegistration(::core::mem::transmute_copy(&serviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService2<Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddService2(::core::mem::transmute_copy(&serviceid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdateServiceManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Impl, IMPL_OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Impl, IMPL_OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Impl, IMPL_OFFSET>,
            AddService2: AddService2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateServiceRegistration_Impl: Sized + super::Com::IDispatch_Impl {
    fn RegistrationState(&mut self) -> ::windows::core::Result<UpdateServiceRegistrationState>;
    fn ServiceID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsPendingRegistrationWithAU(&mut self) -> ::windows::core::Result<i16>;
    fn Service(&mut self) -> ::windows::core::Result<IUpdateService2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateServiceRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateServiceRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateServiceRegistration_Vtbl {
        unsafe extern "system" fn RegistrationState<Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPendingRegistrationWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegistrationState: RegistrationState::<Impl, IMPL_OFFSET>,
            ServiceID: ServiceID::<Impl, IMPL_OFFSET>,
            IsPendingRegistrationWithAU: IsPendingRegistrationWithAU::<Impl, IMPL_OFFSET>,
            Service: Service::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateServiceRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetClientApplicationID(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn WebProxy(&mut self) -> ::windows::core::Result<IWebProxy>;
    fn SetWebProxy(&mut self, value: ::core::option::Option<IWebProxy>) -> ::windows::core::Result<()>;
    fn CreateUpdateSearcher(&mut self) -> ::windows::core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(&mut self) -> ::windows::core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(&mut self) -> ::windows::core::Result<IUpdateInstaller>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSession_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientApplicationID(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxy<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxy<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWebProxy(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CreateUpdateSearcher<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUpdateSearcher() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateDownloader<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUpdateDownloader() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUpdateInstaller() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Impl, IMPL_OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            WebProxy: WebProxy::<Impl, IMPL_OFFSET>,
            SetWebProxy: SetWebProxy::<Impl, IMPL_OFFSET>,
            CreateUpdateSearcher: CreateUpdateSearcher::<Impl, IMPL_OFFSET>,
            CreateUpdateDownloader: CreateUpdateDownloader::<Impl, IMPL_OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSession2_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSession_Impl {
    fn UserLocale(&mut self) -> ::windows::core::Result<u32>;
    fn SetUserLocale(&mut self, lcid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSession2_Vtbl {
        unsafe extern "system" fn UserLocale<Impl: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserLocale() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserLocale<Impl: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        Self {
            base: IUpdateSession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UserLocale: UserLocale::<Impl, IMPL_OFFSET>,
            SetUserLocale: SetUserLocale::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUpdateSession3_Impl: Sized + super::Com::IDispatch_Impl + IUpdateSession_Impl + IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(&mut self) -> ::windows::core::Result<IUpdateServiceManager2>;
    fn QueryHistory(&mut self, criteria: super::super::Foundation::BSTR, startindex: i32, count: i32) -> ::windows::core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUpdateSession3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSession3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSession3_Vtbl {
        unsafe extern "system" fn CreateUpdateServiceManager<Impl: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUpdateServiceManager() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Impl: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, startindex: i32, count: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHistory(::core::mem::transmute_copy(&criteria), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdateSession2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateUpdateServiceManager: CreateUpdateServiceManager::<Impl, IMPL_OFFSET>,
            QueryHistory: QueryHistory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSession3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWebProxy_Impl: Sized + super::Com::IDispatch_Impl {
    fn Address(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAddress(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BypassList(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn SetBypassList(&mut self, value: ::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
    fn BypassProxyOnLocal(&mut self) -> ::windows::core::Result<i16>;
    fn SetBypassProxyOnLocal(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUserName(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPassword(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PromptForCredentials(&mut self, parentwindow: ::core::option::Option<::windows::core::IUnknown>, title: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PromptForCredentialsFromHwnd(&mut self, parentwindow: super::super::Foundation::HWND, title: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AutoDetect(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoDetect(&mut self, value: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWebProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebProxy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebProxy_Vtbl {
        unsafe extern "system" fn Address<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BypassList<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BypassList() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassList<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBypassList(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BypassProxyOnLocal<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BypassProxyOnLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBypassProxyOnLocal(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetPassword<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn PromptForCredentials<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwindow: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PromptForCredentials(::core::mem::transmute(&parentwindow), ::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PromptForCredentialsFromHwnd(::core::mem::transmute_copy(&parentwindow), ::core::mem::transmute_copy(&title)).into()
        }
        unsafe extern "system" fn AutoDetect<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDetect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetect<Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoDetect(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            SetAddress: SetAddress::<Impl, IMPL_OFFSET>,
            BypassList: BypassList::<Impl, IMPL_OFFSET>,
            SetBypassList: SetBypassList::<Impl, IMPL_OFFSET>,
            BypassProxyOnLocal: BypassProxyOnLocal::<Impl, IMPL_OFFSET>,
            SetBypassProxyOnLocal: SetBypassProxyOnLocal::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
            PromptForCredentials: PromptForCredentials::<Impl, IMPL_OFFSET>,
            PromptForCredentialsFromHwnd: PromptForCredentialsFromHwnd::<Impl, IMPL_OFFSET>,
            AutoDetect: AutoDetect::<Impl, IMPL_OFFSET>,
            SetAutoDetect: SetAutoDetect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebProxy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl {
    fn DriverClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverHardwareID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverManufacturer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverModel(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverProvider(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverVerDate(&mut self) -> ::windows::core::Result<f64>;
    fn DeviceProblemNumber(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceStatus(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdate_Vtbl {
        unsafe extern "system" fn DriverClass<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverClass() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverHardwareID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverModel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverVerDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceProblemNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUpdate_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DriverClass: DriverClass::<Impl, IMPL_OFFSET>,
            DriverHardwareID: DriverHardwareID::<Impl, IMPL_OFFSET>,
            DriverManufacturer: DriverManufacturer::<Impl, IMPL_OFFSET>,
            DriverModel: DriverModel::<Impl, IMPL_OFFSET>,
            DriverProvider: DriverProvider::<Impl, IMPL_OFFSET>,
            DriverVerDate: DriverVerDate::<Impl, IMPL_OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Impl, IMPL_OFFSET>,
            DeviceStatus: DeviceStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate2_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl {
    fn RebootRequired(&mut self) -> ::windows::core::Result<i16>;
    fn IsPresent(&mut self) -> ::windows::core::Result<i16>;
    fn CveIDs(&mut self) -> ::windows::core::Result<IStringCollection>;
    fn CopyToCache(&mut self, pfiles: ::core::option::Option<IStringCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdate2_Vtbl {
        unsafe extern "system" fn RebootRequired<Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CveIDs() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiles: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyToCache(::core::mem::transmute(&pfiles)).into()
        }
        Self {
            base: IWindowsDriverUpdate_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RebootRequired: RebootRequired::<Impl, IMPL_OFFSET>,
            IsPresent: IsPresent::<Impl, IMPL_OFFSET>,
            CveIDs: CveIDs::<Impl, IMPL_OFFSET>,
            CopyToCache: CopyToCache::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate3_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl + IWindowsDriverUpdate2_Impl {
    fn BrowseOnly(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdate3_Vtbl {
        unsafe extern "system" fn BrowseOnly<Impl: IWindowsDriverUpdate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrowseOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWindowsDriverUpdate2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BrowseOnly: BrowseOnly::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate4_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl + IWindowsDriverUpdate2_Impl + IWindowsDriverUpdate3_Impl {
    fn WindowsDriverUpdateEntries(&mut self) -> ::windows::core::Result<IWindowsDriverUpdateEntryCollection>;
    fn PerUser(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdate4_Vtbl {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowsDriverUpdateEntries() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerUser<Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWindowsDriverUpdate3_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WindowsDriverUpdateEntries: WindowsDriverUpdateEntries::<Impl, IMPL_OFFSET>,
            PerUser: PerUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdate5_Impl: Sized + super::Com::IDispatch_Impl + IUpdate_Impl + IWindowsDriverUpdate_Impl + IWindowsDriverUpdate2_Impl + IWindowsDriverUpdate3_Impl + IWindowsDriverUpdate4_Impl {
    fn AutoSelection(&mut self) -> ::windows::core::Result<AutoSelectionMode>;
    fn AutoDownload(&mut self) -> ::windows::core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdate5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdate5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdate5_Vtbl {
        unsafe extern "system" fn AutoSelection<Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDownload() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWindowsDriverUpdate4_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AutoSelection: AutoSelection::<Impl, IMPL_OFFSET>,
            AutoDownload: AutoDownload::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdateEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn DriverClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverHardwareID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverManufacturer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverModel(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverProvider(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DriverVerDate(&mut self) -> ::windows::core::Result<f64>;
    fn DeviceProblemNumber(&mut self) -> ::windows::core::Result<i32>;
    fn DeviceStatus(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdateEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdateEntry_Vtbl {
        unsafe extern "system" fn DriverClass<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverClass() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverHardwareID() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverModel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DriverVerDate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceProblemNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DriverClass: DriverClass::<Impl, IMPL_OFFSET>,
            DriverHardwareID: DriverHardwareID::<Impl, IMPL_OFFSET>,
            DriverManufacturer: DriverManufacturer::<Impl, IMPL_OFFSET>,
            DriverModel: DriverModel::<Impl, IMPL_OFFSET>,
            DriverProvider: DriverProvider::<Impl, IMPL_OFFSET>,
            DriverVerDate: DriverVerDate::<Impl, IMPL_OFFSET>,
            DeviceProblemNumber: DeviceProblemNumber::<Impl, IMPL_OFFSET>,
            DeviceStatus: DeviceStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsDriverUpdateEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IWindowsDriverUpdateEntry>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsDriverUpdateEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsDriverUpdateEntryCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsDriverUpdateEntryCollection_Vtbl {
        unsafe extern "system" fn Item<Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntryCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWindowsUpdateAgentInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetInfo(&mut self, varinfoidentifier: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWindowsUpdateAgentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsUpdateAgentInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsUpdateAgentInfo_Vtbl {
        unsafe extern "system" fn GetInfo<Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinfoidentifier: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&varinfoidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetInfo: GetInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsUpdateAgentInfo as ::windows::core::Interface>::IID
    }
}

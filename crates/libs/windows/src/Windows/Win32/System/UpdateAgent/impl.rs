#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdates_Impl: Sized + super::Com::IDispatch_Impl {
    fn DetectNow(&self) -> ::windows_core::Result<()>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Resume(&self) -> ::windows_core::Result<()>;
    fn ShowSettingsDialog(&self) -> ::windows_core::Result<()>;
    fn Settings(&self) -> ::windows_core::Result<IAutomaticUpdatesSettings>;
    fn ServiceEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EnableService(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAutomaticUpdates {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAutomaticUpdates_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>() -> IAutomaticUpdates_Vtbl {
        unsafe extern "system" fn DetectNow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DetectNow().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn ShowSettingsDialog<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowSettingsDialog().into()
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Settings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableService().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DetectNow: DetectNow::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            ShowSettingsDialog: ShowSettingsDialog::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            ServiceEnabled: ServiceEnabled::<Identity, Impl, OFFSET>,
            EnableService: EnableService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdates2_Impl: Sized + IAutomaticUpdates_Impl {
    fn Results(&self) -> ::windows_core::Result<IAutomaticUpdatesResults>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAutomaticUpdates2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAutomaticUpdates2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates2_Impl, const OFFSET: isize>() -> IAutomaticUpdates2_Vtbl {
        unsafe extern "system" fn Results<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdates2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Results() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAutomaticUpdates_Vtbl::new::<Identity, Impl, OFFSET>(), Results: Results::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdates2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IAutomaticUpdates as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesResults_Impl: Sized + super::Com::IDispatch_Impl {
    fn LastSearchSuccessDate(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn LastInstallationSuccessDate(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAutomaticUpdatesResults {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAutomaticUpdatesResults_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>() -> IAutomaticUpdatesResults_Vtbl {
        unsafe extern "system" fn LastSearchSuccessDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastSearchSuccessDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastInstallationSuccessDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LastSearchSuccessDate: LastSearchSuccessDate::<Identity, Impl, OFFSET>,
            LastInstallationSuccessDate: LastInstallationSuccessDate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesResults as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesSettings_Impl: Sized + super::Com::IDispatch_Impl {
    fn NotificationLevel(&self) -> ::windows_core::Result<AutomaticUpdatesNotificationLevel>;
    fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::Result<()>;
    fn ReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Required(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ScheduledInstallationDay(&self) -> ::windows_core::Result<AutomaticUpdatesScheduledInstallationDay>;
    fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::Result<()>;
    fn ScheduledInstallationTime(&self) -> ::windows_core::Result<i32>;
    fn SetScheduledInstallationTime(&self, value: i32) -> ::windows_core::Result<()>;
    fn Refresh(&self) -> ::windows_core::Result<()>;
    fn Save(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAutomaticUpdatesSettings {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAutomaticUpdatesSettings_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>() -> IAutomaticUpdatesSettings_Vtbl {
        unsafe extern "system" fn NotificationLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NotificationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotificationLevel(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Required<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Required() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledInstallationDay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduledInstallationDay() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScheduledInstallationDay(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ScheduledInstallationTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScheduledInstallationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScheduledInstallationTime(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesSettings2_Impl: Sized + IAutomaticUpdatesSettings_Impl {
    fn IncludeRecommendedUpdates(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncludeRecommendedUpdates(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAutomaticUpdatesSettings2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAutomaticUpdatesSettings2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>() -> IAutomaticUpdatesSettings2_Vtbl {
        unsafe extern "system" fn IncludeRecommendedUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncludeRecommendedUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIncludeRecommendedUpdates(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CheckPermission<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckPermission(::core::mem::transmute_copy(&usertype), ::core::mem::transmute_copy(&permissiontype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(userhaspermission, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAutomaticUpdatesSettings_Vtbl::new::<Identity, Impl, OFFSET>(),
            IncludeRecommendedUpdates: IncludeRecommendedUpdates::<Identity, Impl, OFFSET>,
            SetIncludeRecommendedUpdates: SetIncludeRecommendedUpdates::<Identity, Impl, OFFSET>,
            CheckPermission: CheckPermission::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IAutomaticUpdatesSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAutomaticUpdatesSettings3_Impl: Sized + IAutomaticUpdatesSettings2_Impl {
    fn NonAdministratorsElevated(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNonAdministratorsElevated(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FeaturedUpdatesEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFeaturedUpdatesEnabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAutomaticUpdatesSettings3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAutomaticUpdatesSettings3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>() -> IAutomaticUpdatesSettings3_Vtbl {
        unsafe extern "system" fn NonAdministratorsElevated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonAdministratorsElevated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNonAdministratorsElevated(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FeaturedUpdatesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAutomaticUpdatesSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFeaturedUpdatesEnabled(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IAutomaticUpdatesSettings2_Vtbl::new::<Identity, Impl, OFFSET>(),
            NonAdministratorsElevated: NonAdministratorsElevated::<Identity, Impl, OFFSET>,
            SetNonAdministratorsElevated: SetNonAdministratorsElevated::<Identity, Impl, OFFSET>,
            FeaturedUpdatesEnabled: FeaturedUpdatesEnabled::<Identity, Impl, OFFSET>,
            SetFeaturedUpdatesEnabled: SetFeaturedUpdatesEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAutomaticUpdatesSettings3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IAutomaticUpdatesSettings as ::windows_core::ComInterface>::IID || iid == &<IAutomaticUpdatesSettings2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICategory_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CategoryID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Children(&self) -> ::windows_core::Result<ICategoryCollection>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Image(&self) -> ::windows_core::Result<IImageInformation>;
    fn Order(&self) -> ::windows_core::Result<i32>;
    fn Parent(&self) -> ::windows_core::Result<ICategory>;
    fn Type(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Updates(&self) -> ::windows_core::Result<IUpdateCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICategory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICategory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>() -> ICategory_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CategoryID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CategoryID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Image() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Order<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Order() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICategory as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICategoryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<ICategory>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICategoryCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICategoryCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: isize>() -> ICategoryCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICategoryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICategoryCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallback_Impl: Sized {
    fn Invoke(&self, downloadjob: ::core::option::Option<&IDownloadJob>, callbackargs: ::core::option::Option<&IDownloadCompletedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDownloadCompletedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadCompletedCallback_Impl, const OFFSET: isize>() -> IDownloadCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::windows_core::from_raw_borrowed(&downloadjob), ::windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDownloadCompletedCallbackArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadCompletedCallbackArgs_Impl, const OFFSET: isize>() -> IDownloadCompletedCallbackArgs_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadCompletedCallbackArgs as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsCompleted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Updates(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> ::windows_core::Result<()>;
    fn GetProgress(&self) -> ::windows_core::Result<IDownloadProgress>;
    fn RequestAbort(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDownloadJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadJob_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>() -> IDownloadJob_Vtbl {
        unsafe extern "system" fn AsyncState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CleanUp().into()
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAbort().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadJob as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateBytesDownloaded(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateBytesToDownload(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn CurrentUpdateIndex(&self) -> ::windows_core::Result<i32>;
    fn PercentComplete(&self) -> ::windows_core::Result<i32>;
    fn TotalBytesDownloaded(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn TotalBytesToDownload(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateDownloadResult>;
    fn CurrentUpdateDownloadPhase(&self) -> ::windows_core::Result<DownloadPhase>;
    fn CurrentUpdatePercentComplete(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDownloadProgress {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadProgress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>() -> IDownloadProgress_Vtbl {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdateBytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdateBytesToDownload() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdateIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesDownloaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalBytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesToDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalBytesToDownload() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPhase) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdateDownloadPhase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdatePercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadProgress as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallback_Impl: Sized {
    fn Invoke(&self, downloadjob: ::core::option::Option<&IDownloadJob>, callbackargs: ::core::option::Option<&IDownloadProgressChangedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IDownloadProgressChangedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>() -> IDownloadProgressChangedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::windows_core::from_raw_borrowed(&downloadjob), ::windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&self) -> ::windows_core::Result<IDownloadProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDownloadProgressChangedCallbackArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> IDownloadProgressChangedCallbackArgs_Vtbl {
        unsafe extern "system" fn Progress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Progress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Progress: Progress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadProgressChangedCallbackArgs as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows_core::Result<i32>;
    fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDownloadResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: isize>() -> IDownloadResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDownloadResult as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IImageInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn AltText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Height(&self) -> ::windows_core::Result<i32>;
    fn Source(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Width(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IImageInformation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IImageInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: isize>() -> IImageInformation_Vtbl {
        unsafe extern "system" fn AltText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AltText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Height() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Source() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IImageInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Width() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AltText: AltText::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Source: Source::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IImageInformation as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationAgent_Impl: Sized + super::Com::IDispatch_Impl {
    fn RecordInstallationResult(&self, installationresultcookie: &::windows_core::BSTR, hresult: i32, extendedreportingdata: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationAgent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationAgent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationAgent_Impl, const OFFSET: isize>() -> IInstallationAgent_Vtbl {
        unsafe extern "system" fn RecordInstallationResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationresultcookie: ::std::mem::MaybeUninit<::windows_core::BSTR>, hresult: i32, extendedreportingdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordInstallationResult(::core::mem::transmute(&installationresultcookie), ::core::mem::transmute_copy(&hresult), ::windows_core::from_raw_borrowed(&extendedreportingdata)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecordInstallationResult: RecordInstallationResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationAgent as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationBehavior_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanRequestUserInput(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Impact(&self) -> ::windows_core::Result<InstallationImpact>;
    fn RebootBehavior(&self) -> ::windows_core::Result<InstallationRebootBehavior>;
    fn RequiresNetworkConnectivity(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationBehavior {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationBehavior_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: isize>() -> IInstallationBehavior_Vtbl {
        unsafe extern "system" fn CanRequestUserInput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRequestUserInput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Impact<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationImpact) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Impact() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootBehavior<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequiresNetworkConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CanRequestUserInput: CanRequestUserInput::<Identity, Impl, OFFSET>,
            Impact: Impact::<Identity, Impl, OFFSET>,
            RebootBehavior: RebootBehavior::<Identity, Impl, OFFSET>,
            RequiresNetworkConnectivity: RequiresNetworkConnectivity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationBehavior as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallback_Impl: Sized {
    fn Invoke(&self, installationjob: ::core::option::Option<&IInstallationJob>, callbackargs: ::core::option::Option<&IInstallationCompletedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IInstallationCompletedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationCompletedCallback_Impl, const OFFSET: isize>() -> IInstallationCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::windows_core::from_raw_borrowed(&installationjob), ::windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationCompletedCallbackArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationCompletedCallbackArgs_Impl, const OFFSET: isize>() -> IInstallationCompletedCallbackArgs_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationCompletedCallbackArgs as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsCompleted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Updates(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn CleanUp(&self) -> ::windows_core::Result<()>;
    fn GetProgress(&self) -> ::windows_core::Result<IInstallationProgress>;
    fn RequestAbort(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationJob_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>() -> IInstallationJob_Vtbl {
        unsafe extern "system" fn AsyncState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CleanUp().into()
        }
        unsafe extern "system" fn GetProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProgress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAbort().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            GetProgress: GetProgress::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationJob as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationProgress_Impl: Sized + super::Com::IDispatch_Impl {
    fn CurrentUpdateIndex(&self) -> ::windows_core::Result<i32>;
    fn CurrentUpdatePercentComplete(&self) -> ::windows_core::Result<i32>;
    fn PercentComplete(&self) -> ::windows_core::Result<i32>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationProgress {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationProgress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: isize>() -> IInstallationProgress_Vtbl {
        unsafe extern "system" fn CurrentUpdateIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdateIndex() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentUpdatePercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentUpdateIndex: CurrentUpdateIndex::<Identity, Impl, OFFSET>,
            CurrentUpdatePercentComplete: CurrentUpdatePercentComplete::<Identity, Impl, OFFSET>,
            PercentComplete: PercentComplete::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationProgress as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallback_Impl: Sized {
    fn Invoke(&self, installationjob: ::core::option::Option<&IInstallationJob>, callbackargs: ::core::option::Option<&IInstallationProgressChangedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IInstallationProgressChangedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>() -> IInstallationProgressChangedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgressChangedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, installationjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::windows_core::from_raw_borrowed(&installationjob), ::windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationProgressChangedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {
    fn Progress(&self) -> ::windows_core::Result<IInstallationProgress>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationProgressChangedCallbackArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationProgressChangedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>() -> IInstallationProgressChangedCallbackArgs_Vtbl {
        unsafe extern "system" fn Progress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationProgressChangedCallbackArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Progress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Progress: Progress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationProgressChangedCallbackArgs as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows_core::Result<i32>;
    fn RebootRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode>;
    fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateInstallationResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInstallationResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInstallationResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: isize>() -> IInstallationResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpdateResult(::core::mem::transmute_copy(&updateindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            GetUpdateResult: GetUpdateResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInstallationResult as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IInvalidProductLicenseException_Impl: Sized + IUpdateException_Impl {
    fn Product(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IInvalidProductLicenseException {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IInvalidProductLicenseException_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInvalidProductLicenseException_Impl, const OFFSET: isize>() -> IInvalidProductLicenseException_Vtbl {
        unsafe extern "system" fn Product<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IInvalidProductLicenseException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Product() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateException_Vtbl::new::<Identity, Impl, OFFSET>(), Product: Product::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IInvalidProductLicenseException as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateException as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallback_Impl: Sized {
    fn Invoke(&self, searchjob: ::core::option::Option<&ISearchJob>, callbackargs: ::core::option::Option<&ISearchCompletedCallbackArgs>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for ISearchCompletedCallback {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchCompletedCallback_Impl, const OFFSET: isize>() -> ISearchCompletedCallback_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchCompletedCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchjob: *mut ::core::ffi::c_void, callbackargs: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::windows_core::from_raw_borrowed(&searchjob), ::windows_core::from_raw_borrowed(&callbackargs)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISearchCompletedCallbackArgs_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISearchCompletedCallbackArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISearchCompletedCallbackArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchCompletedCallbackArgs_Impl, const OFFSET: isize>() -> ISearchCompletedCallbackArgs_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISearchCompletedCallbackArgs as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISearchJob_Impl: Sized + super::Com::IDispatch_Impl {
    fn AsyncState(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsCompleted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CleanUp(&self) -> ::windows_core::Result<()>;
    fn RequestAbort(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISearchJob {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISearchJob_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: isize>() -> ISearchJob_Vtbl {
        unsafe extern "system" fn AsyncState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CleanUp().into()
        }
        unsafe extern "system" fn RequestAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchJob_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestAbort().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AsyncState: AsyncState::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            CleanUp: CleanUp::<Identity, Impl, OFFSET>,
            RequestAbort: RequestAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISearchJob as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISearchResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode>;
    fn RootCategories(&self) -> ::windows_core::Result<ICategoryCollection>;
    fn Updates(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn Warnings(&self) -> ::windows_core::Result<IUpdateExceptionCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISearchResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISearchResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: isize>() -> ISearchResult_Vtbl {
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootCategories<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RootCategories() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Warnings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISearchResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Warnings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
            RootCategories: RootCategories::<Identity, Impl, OFFSET>,
            Updates: Updates::<Identity, Impl, OFFSET>,
            Warnings: Warnings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISearchResult as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IStringCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_Item(&self, index: i32, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn ReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn Clear(&self) -> ::windows_core::Result<()>;
    fn Copy(&self) -> ::windows_core::Result<IStringCollection>;
    fn Insert(&self, index: i32, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IStringCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IStringCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>() -> IStringCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_Item(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Copy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Insert(::core::mem::transmute_copy(&index), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStringCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            put_Item: put_Item::<Identity, Impl, OFFSET>,
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStringCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISystemInformation_Impl: Sized + super::Com::IDispatch_Impl {
    fn OemHardwareSupportLink(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RebootRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISystemInformation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISystemInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemInformation_Impl, const OFFSET: isize>() -> ISystemInformation_Vtbl {
        unsafe extern "system" fn OemHardwareSupportLink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OemHardwareSupportLink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            OemHardwareSupportLink: OemHardwareSupportLink::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISystemInformation as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate_Impl: Sized + super::Com::IDispatch_Impl {
    fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn CanRequireSource(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Categories(&self) -> ::windows_core::Result<ICategoryCollection>;
    fn Deadline(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EulaAccepted(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EulaText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HandlerID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity>;
    fn Image(&self) -> ::windows_core::Result<IImageInformation>;
    fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior>;
    fn IsBeta(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsDownloaded(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsInstalled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsMandatory(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUninstallable(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Languages(&self) -> ::windows_core::Result<IStringCollection>;
    fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64>;
    fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL>;
    fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection>;
    fn MsrcSeverity(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32>;
    fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32>;
    fn RecommendedMemory(&self) -> ::windows_core::Result<i32>;
    fn ReleaseNotes(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection>;
    fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection>;
    fn SupportUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(&self) -> ::windows_core::Result<UpdateType>;
    fn UninstallationNotes(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior>;
    fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection>;
    fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection>;
    fn AcceptEula(&self) -> ::windows_core::Result<()>;
    fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction>;
    fn CopyFromCache(&self, path: &::windows_core::BSTR, toextractcabfiles: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority>;
    fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>() -> IUpdate_Vtbl {
        unsafe extern "system" fn Title<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoSelectOnWebSites() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BundledUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BundledUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequireSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRequireSource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Categories<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Categories() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeltaCompressedContentAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeltaCompressedContentPreferred() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaAccepted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EulaAccepted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EulaText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandlerID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HandlerID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Identity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Image() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallationBehavior<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstallationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBeta<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBeta() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsHidden(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn IsInstalled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInstalled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMandatory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMandatory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUninstallable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Languages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastDeploymentChangeTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDownloadSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDownloadSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinDownloadSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreInfoUrls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoreInfoUrls() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsrcSeverity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MsrcSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecommendedCpuSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecommendedHardDiskSpace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecommendedMemory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseNotes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReleaseNotes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityBulletinIDs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SecurityBulletinIDs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupersededUpdateIDs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupersededUpdateIDs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UninstallationNotes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationBehavior<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UninstallationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UninstallationSteps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KBArticleIDs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KBArticleIDs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptEula<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcceptEula().into()
        }
        unsafe extern "system" fn DeploymentAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DeploymentAction) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeploymentAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, toextractcabfiles: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyFromCache(::core::mem::transmute(&path), ::core::mem::transmute_copy(&toextractcabfiles)).into()
        }
        unsafe extern "system" fn DownloadPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadContents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadContents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate2_Impl: Sized + IUpdate_Impl {
    fn RebootRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPresent(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CveIDs(&self) -> ::windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdate2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdate2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: isize>() -> IUpdate2_Vtbl {
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPresent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CveIDs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiles: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyToCache(::windows_core::from_raw_borrowed(&pfiles)).into()
        }
        Self {
            base__: IUpdate_Vtbl::new::<Identity, Impl, OFFSET>(),
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            IsPresent: IsPresent::<Identity, Impl, OFFSET>,
            CveIDs: CveIDs::<Identity, Impl, OFFSET>,
            CopyToCache: CopyToCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdate2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate3_Impl: Sized + IUpdate2_Impl {
    fn BrowseOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdate3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdate3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate3_Impl, const OFFSET: isize>() -> IUpdate3_Vtbl {
        unsafe extern "system" fn BrowseOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BrowseOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUpdate2_Vtbl::new::<Identity, Impl, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdate3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IUpdate2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate4_Impl: Sized + IUpdate3_Impl {
    fn PerUser(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdate4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdate4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate4_Impl, const OFFSET: isize>() -> IUpdate4_Vtbl {
        unsafe extern "system" fn PerUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUpdate3_Vtbl::new::<Identity, Impl, OFFSET>(), PerUser: PerUser::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdate4 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IUpdate2 as ::windows_core::ComInterface>::IID || iid == &<IUpdate3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdate5_Impl: Sized + IUpdate4_Impl {
    fn AutoSelection(&self) -> ::windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> ::windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdate5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdate5_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate5_Impl, const OFFSET: isize>() -> IUpdate5_Vtbl {
        unsafe extern "system" fn AutoSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoDownload() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdate4_Vtbl::new::<Identity, Impl, OFFSET>(),
            AutoSelection: AutoSelection::<Identity, Impl, OFFSET>,
            AutoDownload: AutoDownload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdate5 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IUpdate2 as ::windows_core::ComInterface>::IID || iid == &<IUpdate3 as ::windows_core::ComInterface>::IID || iid == &<IUpdate4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdate>;
    fn put_Item(&self, index: i32, value: ::core::option::Option<&IUpdate>) -> ::windows_core::Result<()>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn ReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, value: ::core::option::Option<&IUpdate>) -> ::windows_core::Result<i32>;
    fn Clear(&self) -> ::windows_core::Result<()>;
    fn Copy(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn Insert(&self, index: i32, value: ::core::option::Option<&IUpdate>) -> ::windows_core::Result<()>;
    fn RemoveAt(&self, index: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>() -> IUpdateCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_Item(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Copy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Insert(::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAt(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            put_Item: put_Item::<Identity, Impl, OFFSET>,
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadContent_Impl: Sized + super::Com::IDispatch_Impl {
    fn DownloadUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateDownloadContent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateDownloadContent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContent_Impl, const OFFSET: isize>() -> IUpdateDownloadContent_Vtbl {
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DownloadUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadContent2_Impl: Sized + IUpdateDownloadContent_Impl {
    fn IsDeltaCompressedContent(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateDownloadContent2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateDownloadContent2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContent2_Impl, const OFFSET: isize>() -> IUpdateDownloadContent2_Vtbl {
        unsafe extern "system" fn IsDeltaCompressedContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDeltaCompressedContent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdateDownloadContent_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDeltaCompressedContent: IsDeltaCompressedContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContent2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateDownloadContent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadContentCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateDownloadContent>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateDownloadContentCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateDownloadContentCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>() -> IUpdateDownloadContentCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadContentCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadContentCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloadResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows_core::Result<i32>;
    fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateDownloadResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>() -> IUpdateDownloadResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateDownloadResult as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateDownloader_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsForced(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsForced(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Priority(&self) -> ::windows_core::Result<DownloadPriority>;
    fn SetPriority(&self, value: DownloadPriority) -> ::windows_core::Result<()>;
    fn Updates(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: ::core::option::Option<&IUpdateCollection>) -> ::windows_core::Result<()>;
    fn BeginDownload(&self, onprogresschanged: ::core::option::Option<&::windows_core::IUnknown>, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<IDownloadJob>;
    fn Download(&self) -> ::windows_core::Result<IDownloadResult>;
    fn EndDownload(&self, value: ::core::option::Option<&IDownloadJob>) -> ::windows_core::Result<IDownloadResult>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateDownloader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateDownloader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>() -> IUpdateDownloader_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsForced<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsForced() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsForced(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DownloadPriority) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUpdates(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginDownload(::windows_core::from_raw_borrowed(&onprogresschanged), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Download() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateDownloader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndDownload(::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateDownloader as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateException_Impl: Sized + super::Com::IDispatch_Impl {
    fn Message(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HResult(&self) -> ::windows_core::Result<i32>;
    fn Context(&self) -> ::windows_core::Result<UpdateExceptionContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateException {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateException_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: isize>() -> IUpdateException_Vtbl {
        unsafe extern "system" fn Message<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Message() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateException_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateExceptionContext) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Context() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Message: Message::<Identity, Impl, OFFSET>,
            HResult: HResult::<Identity, Impl, OFFSET>,
            Context: Context::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateException as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateExceptionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateException>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateExceptionCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateExceptionCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>() -> IUpdateExceptionCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateExceptionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateExceptionCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateHistoryEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn Operation(&self) -> ::windows_core::Result<UpdateOperation>;
    fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode>;
    fn HResult(&self) -> ::windows_core::Result<i32>;
    fn Date(&self) -> ::windows_core::Result<f64>;
    fn UpdateIdentity(&self) -> ::windows_core::Result<IUpdateIdentity>;
    fn Title(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UnmappedResultCode(&self) -> ::windows_core::Result<i32>;
    fn ClientApplicationID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection>;
    fn ServiceID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection>;
    fn UninstallationNotes(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SupportUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateHistoryEntry {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateHistoryEntry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>() -> IUpdateHistoryEntry_Vtbl {
        unsafe extern "system" fn Operation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateOperation) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Date() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UpdateIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmappedResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnmappedResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UninstallationSteps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UninstallationNotes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateHistoryEntry2_Impl: Sized + IUpdateHistoryEntry_Impl {
    fn Categories(&self) -> ::windows_core::Result<ICategoryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateHistoryEntry2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateHistoryEntry2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry2_Impl, const OFFSET: isize>() -> IUpdateHistoryEntry2_Vtbl {
        unsafe extern "system" fn Categories<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntry2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Categories() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateHistoryEntry_Vtbl::new::<Identity, Impl, OFFSET>(), Categories: Categories::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntry2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateHistoryEntry as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateHistoryEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateHistoryEntry>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateHistoryEntryCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateHistoryEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>() -> IUpdateHistoryEntryCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateHistoryEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateHistoryEntryCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateIdentity_Impl: Sized + super::Com::IDispatch_Impl {
    fn RevisionNumber(&self) -> ::windows_core::Result<i32>;
    fn UpdateID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateIdentity {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateIdentity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateIdentity_Impl, const OFFSET: isize>() -> IUpdateIdentity_Vtbl {
        unsafe extern "system" fn RevisionNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RevisionNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UpdateID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RevisionNumber: RevisionNumber::<Identity, Impl, OFFSET>,
            UpdateID: UpdateID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateIdentity as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstallationResult_Impl: Sized + super::Com::IDispatch_Impl {
    fn HResult(&self) -> ::windows_core::Result<i32>;
    fn RebootRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateInstallationResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateInstallationResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>() -> IUpdateInstallationResult_Vtbl {
        unsafe extern "system" fn HResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstallationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            HResult: HResult::<Identity, Impl, OFFSET>,
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            ResultCode: ResultCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateInstallationResult as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsForced(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsForced(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ParentHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn SetParentHwnd(&self, value: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn SetParentWindow(&self, value: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ParentWindow(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Updates(&self) -> ::windows_core::Result<IUpdateCollection>;
    fn SetUpdates(&self, value: ::core::option::Option<&IUpdateCollection>) -> ::windows_core::Result<()>;
    fn BeginInstall(&self, onprogresschanged: ::core::option::Option<&::windows_core::IUnknown>, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<IInstallationJob>;
    fn BeginUninstall(&self, onprogresschanged: ::core::option::Option<&::windows_core::IUnknown>, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<IInstallationJob>;
    fn EndInstall(&self, value: ::core::option::Option<&IInstallationJob>) -> ::windows_core::Result<IInstallationResult>;
    fn EndUninstall(&self, value: ::core::option::Option<&IInstallationJob>) -> ::windows_core::Result<IInstallationResult>;
    fn Install(&self) -> ::windows_core::Result<IInstallationResult>;
    fn RunWizard(&self, dialogtitle: &::windows_core::BSTR) -> ::windows_core::Result<IInstallationResult>;
    fn IsBusy(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Uninstall(&self) -> ::windows_core::Result<IInstallationResult>;
    fn AllowSourcePrompts(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowSourcePrompts(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn RebootRequiredBeforeInstallation(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateInstaller {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>() -> IUpdateInstaller_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IsForced<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsForced() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsForced(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentHwnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParentHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentHwnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParentHwnd(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetParentWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetParentWindow(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ParentWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUpdates(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn BeginInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginInstall(::windows_core::from_raw_borrowed(&onprogresschanged), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUninstall(::windows_core::from_raw_borrowed(&onprogresschanged), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndInstall(::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUninstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndUninstall(::windows_core::from_raw_borrowed(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Install() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWizard<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dialogtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunWizard(::core::mem::transmute(&dialogtitle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBusy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsBusy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Uninstall() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSourcePrompts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowSourcePrompts() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowSourcePrompts(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootRequiredBeforeInstallation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller2_Impl: Sized + IUpdateInstaller_Impl {
    fn ForceQuiet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetForceQuiet(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateInstaller2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateInstaller2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller2_Impl, const OFFSET: isize>() -> IUpdateInstaller2_Vtbl {
        unsafe extern "system" fn ForceQuiet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForceQuiet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceQuiet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForceQuiet(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateInstaller_Vtbl::new::<Identity, Impl, OFFSET>(),
            ForceQuiet: ForceQuiet::<Identity, Impl, OFFSET>,
            SetForceQuiet: SetForceQuiet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateInstaller as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller3_Impl: Sized + IUpdateInstaller2_Impl {
    fn AttemptCloseAppsIfNecessary(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAttemptCloseAppsIfNecessary(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateInstaller3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateInstaller3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller3_Impl, const OFFSET: isize>() -> IUpdateInstaller3_Vtbl {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AttemptCloseAppsIfNecessary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttemptCloseAppsIfNecessary(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateInstaller2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AttemptCloseAppsIfNecessary: AttemptCloseAppsIfNecessary::<Identity, Impl, OFFSET>,
            SetAttemptCloseAppsIfNecessary: SetAttemptCloseAppsIfNecessary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateInstaller as ::windows_core::ComInterface>::IID || iid == &<IUpdateInstaller2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateInstaller4_Impl: Sized + IUpdateInstaller3_Impl {
    fn Commit(&self, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateInstaller4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateInstaller4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller4_Impl, const OFFSET: isize>() -> IUpdateInstaller4_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateInstaller4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: IUpdateInstaller3_Vtbl::new::<Identity, Impl, OFFSET>(), Commit: Commit::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller4 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateInstaller as ::windows_core::ComInterface>::IID || iid == &<IUpdateInstaller2 as ::windows_core::ComInterface>::IID || iid == &<IUpdateInstaller3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"implement\"`*"]
pub trait IUpdateLockdown_Impl: Sized {
    fn LockDown(&self, flags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUpdateLockdown {}
impl IUpdateLockdown_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateLockdown_Impl, const OFFSET: isize>() -> IUpdateLockdown_Vtbl {
        unsafe extern "system" fn LockDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateLockdown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockDown(::core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LockDown: LockDown::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateLockdown as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSearcher_Impl: Sized + super::Com::IDispatch_Impl {
    fn CanAutomaticallyUpgradeService(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCanAutomaticallyUpgradeService(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ClientApplicationID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IncludePotentiallySupersededUpdates(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIncludePotentiallySupersededUpdates(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection>;
    fn SetServerSelection(&self, value: ServerSelection) -> ::windows_core::Result<()>;
    fn BeginSearch(&self, criteria: &::windows_core::BSTR, oncompleted: ::core::option::Option<&::windows_core::IUnknown>, state: &super::Variant::VARIANT) -> ::windows_core::Result<ISearchJob>;
    fn EndSearch(&self, searchjob: ::core::option::Option<&ISearchJob>) -> ::windows_core::Result<ISearchResult>;
    fn EscapeString(&self, unescaped: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn QueryHistory(&self, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection>;
    fn Search(&self, criteria: &::windows_core::BSTR) -> ::windows_core::Result<ISearchResult>;
    fn Online(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetOnline(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetTotalHistoryCount(&self) -> ::windows_core::Result<i32>;
    fn ServiceID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceID(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateSearcher {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateSearcher_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>() -> IUpdateSearcher_Vtbl {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanAutomaticallyUpgradeService() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCanAutomaticallyUpgradeService(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IncludePotentiallySupersededUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIncludePotentiallySupersededUpdates(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ServerSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServerSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ServerSelection) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServerSelection(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BeginSearch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: super::Variant::VARIANT, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSearch(::core::mem::transmute(&criteria), ::windows_core::from_raw_borrowed(&oncompleted), ::core::mem::transmute(&state)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSearch<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchjob: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndSearch(::windows_core::from_raw_borrowed(&searchjob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unescaped: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EscapeString(::core::mem::transmute(&unescaped)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startindex: i32, count: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHistory(::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Search<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Search(::core::mem::transmute(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Online() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOnline<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOnline(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTotalHistoryCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTotalHistoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceID(::core::mem::transmute(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSearcher2_Impl: Sized + IUpdateSearcher_Impl {
    fn IgnoreDownloadPriority(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIgnoreDownloadPriority(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateSearcher2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateSearcher2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher2_Impl, const OFFSET: isize>() -> IUpdateSearcher2_Vtbl {
        unsafe extern "system" fn IgnoreDownloadPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IgnoreDownloadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIgnoreDownloadPriority(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateSearcher_Vtbl::new::<Identity, Impl, OFFSET>(),
            IgnoreDownloadPriority: IgnoreDownloadPriority::<Identity, Impl, OFFSET>,
            SetIgnoreDownloadPriority: SetIgnoreDownloadPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateSearcher as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSearcher3_Impl: Sized + IUpdateSearcher2_Impl {
    fn SearchScope(&self) -> ::windows_core::Result<SearchScope>;
    fn SetSearchScope(&self, value: SearchScope) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateSearcher3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateSearcher3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher3_Impl, const OFFSET: isize>() -> IUpdateSearcher3_Vtbl {
        unsafe extern "system" fn SearchScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut SearchScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SearchScope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSearcher3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SearchScope) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSearchScope(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IUpdateSearcher2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SearchScope: SearchScope::<Identity, Impl, OFFSET>,
            SetSearchScope: SetSearchScope::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateSearcher3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateSearcher as ::windows_core::ComInterface>::IID || iid == &<IUpdateSearcher2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateService_Impl: Sized + super::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ContentValidationCert(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ExpirationDate(&self) -> ::windows_core::Result<f64>;
    fn IsManaged(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsRegisteredWithAU(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IssueDate(&self) -> ::windows_core::Result<f64>;
    fn OffersWindowsUpdates(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RedirectUrls(&self) -> ::windows_core::Result<IStringCollection>;
    fn ServiceID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsScanPackageService(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CanRegisterWithAU(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ServiceUrl(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetupPrefix(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateService {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>() -> IUpdateService_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentValidationCert<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentValidationCert() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpirationDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsManaged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsManaged() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredWithAU<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRegisteredWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IssueDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffersWindowsUpdates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OffersWindowsUpdates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectUrls<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RedirectUrls() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanPackageService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsScanPackageService() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRegisterWithAU<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanRegisterWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceUrl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupPrefix<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetupPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateService as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateService2_Impl: Sized + IUpdateService_Impl {
    fn IsDefaultAUService(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateService2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateService2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService2_Impl, const OFFSET: isize>() -> IUpdateService2_Vtbl {
        unsafe extern "system" fn IsDefaultAUService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateService2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDefaultAUService() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IUpdateService_Vtbl::new::<Identity, Impl, OFFSET>(), IsDefaultAUService: IsDefaultAUService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateService2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateService as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateService>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateServiceCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateServiceCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>() -> IUpdateServiceCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateServiceCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Services(&self) -> ::windows_core::Result<IUpdateServiceCollection>;
    fn AddService(&self, serviceid: &::windows_core::BSTR, authorizationcabpath: &::windows_core::BSTR) -> ::windows_core::Result<IUpdateService>;
    fn RegisterServiceWithAU(&self, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveService(&self, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UnregisterServiceWithAU(&self, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddScanPackageService(&self, servicename: &::windows_core::BSTR, scanfilelocation: &::windows_core::BSTR, flags: i32) -> ::windows_core::Result<IUpdateService>;
    fn SetOption(&self, optionname: &::windows_core::BSTR, optionvalue: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateServiceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateServiceManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>() -> IUpdateServiceManager_Vtbl {
        unsafe extern "system" fn Services<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Services() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, authorizationcabpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddService(::core::mem::transmute(&serviceid), ::core::mem::transmute(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServiceWithAU<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterServiceWithAU(::core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn RemoveService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveService(::core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterServiceWithAU(::core::mem::transmute(&serviceid)).into()
        }
        unsafe extern "system" fn AddScanPackageService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, scanfilelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddScanPackageService(::core::mem::transmute(&servicename), ::core::mem::transmute(&scanfilelocation), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionvalue: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOption(::core::mem::transmute(&optionname), ::core::mem::transmute(&optionvalue)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Services: Services::<Identity, Impl, OFFSET>,
            AddService: AddService::<Identity, Impl, OFFSET>,
            RegisterServiceWithAU: RegisterServiceWithAU::<Identity, Impl, OFFSET>,
            RemoveService: RemoveService::<Identity, Impl, OFFSET>,
            UnregisterServiceWithAU: UnregisterServiceWithAU::<Identity, Impl, OFFSET>,
            AddScanPackageService: AddScanPackageService::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceManager2_Impl: Sized + IUpdateServiceManager_Impl {
    fn ClientApplicationID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryServiceRegistration(&self, serviceid: &::windows_core::BSTR) -> ::windows_core::Result<IUpdateServiceRegistration>;
    fn AddService2(&self, serviceid: &::windows_core::BSTR, flags: i32, authorizationcabpath: &::windows_core::BSTR) -> ::windows_core::Result<IUpdateServiceRegistration>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateServiceManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateServiceManager2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>() -> IUpdateServiceManager2_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn QueryServiceRegistration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryServiceRegistration(::core::mem::transmute(&serviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: i32, authorizationcabpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddService2(::core::mem::transmute(&serviceid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&authorizationcabpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdateServiceManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            ClientApplicationID: ClientApplicationID::<Identity, Impl, OFFSET>,
            SetClientApplicationID: SetClientApplicationID::<Identity, Impl, OFFSET>,
            QueryServiceRegistration: QueryServiceRegistration::<Identity, Impl, OFFSET>,
            AddService2: AddService2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateServiceManager2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateServiceManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateServiceRegistration_Impl: Sized + super::Com::IDispatch_Impl {
    fn RegistrationState(&self) -> ::windows_core::Result<UpdateServiceRegistrationState>;
    fn ServiceID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsPendingRegistrationWithAU(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Service(&self) -> ::windows_core::Result<IUpdateService2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateServiceRegistration {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateServiceRegistration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>() -> IUpdateServiceRegistration_Vtbl {
        unsafe extern "system" fn RegistrationState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPendingRegistrationWithAU() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateServiceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Service() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegistrationState: RegistrationState::<Identity, Impl, OFFSET>,
            ServiceID: ServiceID::<Identity, Impl, OFFSET>,
            IsPendingRegistrationWithAU: IsPendingRegistrationWithAU::<Identity, Impl, OFFSET>,
            Service: Service::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateServiceRegistration as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn ClientApplicationID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetClientApplicationID(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WebProxy(&self) -> ::windows_core::Result<IWebProxy>;
    fn SetWebProxy(&self, value: ::core::option::Option<&IWebProxy>) -> ::windows_core::Result<()>;
    fn CreateUpdateSearcher(&self) -> ::windows_core::Result<IUpdateSearcher>;
    fn CreateUpdateDownloader(&self) -> ::windows_core::Result<IUpdateDownloader>;
    fn CreateUpdateInstaller(&self) -> ::windows_core::Result<IUpdateInstaller>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateSession {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateSession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>() -> IUpdateSession_Vtbl {
        unsafe extern "system" fn ClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientApplicationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientApplicationID(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WebProxy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWebProxy(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn CreateUpdateSearcher<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUpdateSearcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateDownloader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUpdateDownloader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUpdateInstaller() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateSession as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSession2_Impl: Sized + IUpdateSession_Impl {
    fn UserLocale(&self) -> ::windows_core::Result<u32>;
    fn SetUserLocale(&self, lcid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateSession2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateSession2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession2_Impl, const OFFSET: isize>() -> IUpdateSession2_Vtbl {
        unsafe extern "system" fn UserLocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserLocale() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserLocale<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserLocale(::core::mem::transmute_copy(&lcid)).into()
        }
        Self {
            base__: IUpdateSession_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserLocale: UserLocale::<Identity, Impl, OFFSET>,
            SetUserLocale: SetUserLocale::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateSession2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateSession as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUpdateSession3_Impl: Sized + IUpdateSession2_Impl {
    fn CreateUpdateServiceManager(&self) -> ::windows_core::Result<IUpdateServiceManager2>;
    fn QueryHistory(&self, criteria: &::windows_core::BSTR, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IUpdateSession3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUpdateSession3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession3_Impl, const OFFSET: isize>() -> IUpdateSession3_Vtbl {
        unsafe extern "system" fn CreateUpdateServiceManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUpdateServiceManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUpdateSession3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, criteria: ::std::mem::MaybeUninit<::windows_core::BSTR>, startindex: i32, count: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHistory(::core::mem::transmute(&criteria), ::core::mem::transmute_copy(&startindex), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdateSession2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateUpdateServiceManager: CreateUpdateServiceManager::<Identity, Impl, OFFSET>,
            QueryHistory: QueryHistory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUpdateSession3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdateSession as ::windows_core::ComInterface>::IID || iid == &<IUpdateSession2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWebProxy_Impl: Sized + super::Com::IDispatch_Impl {
    fn Address(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAddress(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BypassList(&self) -> ::windows_core::Result<IStringCollection>;
    fn SetBypassList(&self, value: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
    fn BypassProxyOnLocal(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBypassProxyOnLocal(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ReadOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UserName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetUserName(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetPassword(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PromptForCredentials(&self, parentwindow: ::core::option::Option<&::windows_core::IUnknown>, title: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PromptForCredentialsFromHwnd(&self, parentwindow: super::super::Foundation::HWND, title: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AutoDetect(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoDetect(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWebProxy {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWebProxy_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>() -> IWebProxy_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Address() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddress(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn BypassList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BypassList() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassList<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBypassList(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn BypassProxyOnLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BypassProxyOnLocal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBypassProxyOnLocal(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PromptForCredentials<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwindow: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromptForCredentials(::windows_core::from_raw_borrowed(&parentwindow), ::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromptForCredentialsFromHwnd(::core::mem::transmute_copy(&parentwindow), ::core::mem::transmute(&title)).into()
        }
        unsafe extern "system" fn AutoDetect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoDetect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoDetect(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWebProxy as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate_Impl: Sized + IUpdate_Impl {
    fn DriverClass(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverHardwareID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverManufacturer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverModel(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverProvider(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverVerDate(&self) -> ::windows_core::Result<f64>;
    fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32>;
    fn DeviceStatus(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate_Vtbl {
        unsafe extern "system" fn DriverClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverHardwareID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverModel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverVerDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceProblemNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IUpdate_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate2_Impl: Sized + IWindowsDriverUpdate_Impl {
    fn RebootRequired(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPresent(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CveIDs(&self) -> ::windows_core::Result<IStringCollection>;
    fn CopyToCache(&self, pfiles: ::core::option::Option<&IStringCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdate2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdate2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate2_Vtbl {
        unsafe extern "system" fn RebootRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RebootRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPresent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CveIDs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfiles: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyToCache(::windows_core::from_raw_borrowed(&pfiles)).into()
        }
        Self {
            base__: IWindowsDriverUpdate_Vtbl::new::<Identity, Impl, OFFSET>(),
            RebootRequired: RebootRequired::<Identity, Impl, OFFSET>,
            IsPresent: IsPresent::<Identity, Impl, OFFSET>,
            CveIDs: CveIDs::<Identity, Impl, OFFSET>,
            CopyToCache: CopyToCache::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate3_Impl: Sized + IWindowsDriverUpdate2_Impl {
    fn BrowseOnly(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdate3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdate3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate3_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate3_Vtbl {
        unsafe extern "system" fn BrowseOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BrowseOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWindowsDriverUpdate2_Vtbl::new::<Identity, Impl, OFFSET>(), BrowseOnly: BrowseOnly::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate3 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate4_Impl: Sized + IWindowsDriverUpdate3_Impl {
    fn WindowsDriverUpdateEntries(&self) -> ::windows_core::Result<IWindowsDriverUpdateEntryCollection>;
    fn PerUser(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdate4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdate4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate4_Vtbl {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowsDriverUpdateEntries() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWindowsDriverUpdate3_Vtbl::new::<Identity, Impl, OFFSET>(),
            WindowsDriverUpdateEntries: WindowsDriverUpdateEntries::<Identity, Impl, OFFSET>,
            PerUser: PerUser::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate4 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate2 as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdate5_Impl: Sized + IWindowsDriverUpdate4_Impl {
    fn AutoSelection(&self) -> ::windows_core::Result<AutoSelectionMode>;
    fn AutoDownload(&self) -> ::windows_core::Result<AutoDownloadMode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdate5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdate5_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>() -> IWindowsDriverUpdate5_Vtbl {
        unsafe extern "system" fn AutoSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdate5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutoDownload() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWindowsDriverUpdate4_Vtbl::new::<Identity, Impl, OFFSET>(),
            AutoSelection: AutoSelection::<Identity, Impl, OFFSET>,
            AutoDownload: AutoDownload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdate5 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate2 as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate3 as ::windows_core::ComInterface>::IID || iid == &<IWindowsDriverUpdate4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdateEntry_Impl: Sized + super::Com::IDispatch_Impl {
    fn DriverClass(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverHardwareID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverManufacturer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverModel(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverProvider(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DriverVerDate(&self) -> ::windows_core::Result<f64>;
    fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32>;
    fn DeviceStatus(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdateEntry {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdateEntry_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>() -> IWindowsDriverUpdateEntry_Vtbl {
        unsafe extern "system" fn DriverClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverHardwareID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverModel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DriverVerDate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceProblemNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntry as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsDriverUpdateEntryCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IWindowsDriverUpdateEntry>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsDriverUpdateEntryCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsDriverUpdateEntryCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>() -> IWindowsDriverUpdateEntryCollection_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsDriverUpdateEntryCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsDriverUpdateEntryCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsUpdateAgentInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetInfo(&self, varinfoidentifier: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWindowsUpdateAgentInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWindowsUpdateAgentInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>() -> IWindowsUpdateAgentInfo_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWindowsUpdateAgentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinfoidentifier: super::Variant::VARIANT, retval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInfo(::core::mem::transmute(&varinfoidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetInfo: GetInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWindowsUpdateAgentInfo as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}

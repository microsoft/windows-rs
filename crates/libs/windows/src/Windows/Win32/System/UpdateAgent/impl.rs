#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesImpl: Sized + IDispatchImpl {
    fn DetectNow();
    fn Pause();
    fn Resume();
    fn ShowSettingsDialog();
    fn Settings();
    fn ServiceEnabled();
    fn EnableService();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAutomaticUpdates {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IAutomaticUpdates";
}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesVtbl {
    pub const fn new<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomaticUpdatesVtbl {
        unsafe extern "system" fn DetectNow<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectNow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowSettingsDialog<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowSettingsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Settings(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceEnabled<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceEnabled(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableService<Impl: IAutomaticUpdatesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomaticUpdates>, base.5, DetectNow::<Impl, OFFSET>, Pause::<Impl, OFFSET>, Resume::<Impl, OFFSET>, ShowSettingsDialog::<Impl, OFFSET>, Settings::<Impl, OFFSET>, ServiceEnabled::<Impl, OFFSET>, EnableService::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdates2Impl: Sized + IAutomaticUpdatesImpl + IDispatchImpl {
    fn Results();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAutomaticUpdates2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IAutomaticUpdates2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates2Vtbl {
    pub const fn new<Impl: IAutomaticUpdates2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomaticUpdates2Vtbl {
        unsafe extern "system" fn Results<Impl: IAutomaticUpdates2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Results(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomaticUpdates2>, base.5, Results::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesResultsImpl: Sized + IDispatchImpl {
    fn LastSearchSuccessDate();
    fn LastInstallationSuccessDate();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAutomaticUpdatesResults {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IAutomaticUpdatesResults";
}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesResultsVtbl {
    pub const fn new<Impl: IAutomaticUpdatesResultsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomaticUpdatesResultsVtbl {
        unsafe extern "system" fn LastSearchSuccessDate<Impl: IAutomaticUpdatesResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastSearchSuccessDate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastInstallationSuccessDate<Impl: IAutomaticUpdatesResultsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastInstallationSuccessDate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomaticUpdatesResults>, base.5, LastSearchSuccessDate::<Impl, OFFSET>, LastInstallationSuccessDate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettingsImpl: Sized + IDispatchImpl {
    fn NotificationLevel();
    fn SetNotificationLevel();
    fn ReadOnly();
    fn Required();
    fn ScheduledInstallationDay();
    fn SetScheduledInstallationDay();
    fn ScheduledInstallationTime();
    fn SetScheduledInstallationTime();
    fn Refresh();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAutomaticUpdatesSettings {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IAutomaticUpdatesSettings";
}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettingsVtbl {
    pub const fn new<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomaticUpdatesSettingsVtbl {
        unsafe extern "system" fn NotificationLevel<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotificationLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationLevel<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotificationLevel(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Required<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Required(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledInstallationDay<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScheduledInstallationDay(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationDay<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScheduledInstallationDay(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScheduledInstallationTime<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScheduledInstallationTime(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheduledInstallationTime<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScheduledInstallationTime(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IAutomaticUpdatesSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Save() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomaticUpdatesSettings>, base.5, NotificationLevel::<Impl, OFFSET>, SetNotificationLevel::<Impl, OFFSET>, ReadOnly::<Impl, OFFSET>, Required::<Impl, OFFSET>, ScheduledInstallationDay::<Impl, OFFSET>, SetScheduledInstallationDay::<Impl, OFFSET>, ScheduledInstallationTime::<Impl, OFFSET>, SetScheduledInstallationTime::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Save::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings2Impl: Sized + IAutomaticUpdatesSettingsImpl + IDispatchImpl {
    fn IncludeRecommendedUpdates();
    fn SetIncludeRecommendedUpdates();
    fn CheckPermission();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAutomaticUpdatesSettings2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IAutomaticUpdatesSettings2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings2Vtbl {
    pub const fn new<Impl: IAutomaticUpdatesSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomaticUpdatesSettings2Vtbl {
        unsafe extern "system" fn IncludeRecommendedUpdates<Impl: IAutomaticUpdatesSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludeRecommendedUpdates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeRecommendedUpdates<Impl: IAutomaticUpdatesSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIncludeRecommendedUpdates(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckPermission<Impl: IAutomaticUpdatesSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckPermission(usertype, permissiontype, ::core::mem::transmute_copy(&userhaspermission)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomaticUpdatesSettings2>, base.5, IncludeRecommendedUpdates::<Impl, OFFSET>, SetIncludeRecommendedUpdates::<Impl, OFFSET>, CheckPermission::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAutomaticUpdatesSettings3Impl: Sized + IAutomaticUpdatesSettings2Impl + IAutomaticUpdatesSettingsImpl + IDispatchImpl {
    fn NonAdministratorsElevated();
    fn SetNonAdministratorsElevated();
    fn FeaturedUpdatesEnabled();
    fn SetFeaturedUpdatesEnabled();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAutomaticUpdatesSettings3 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IAutomaticUpdatesSettings3";
}
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings3Vtbl {
    pub const fn new<Impl: IAutomaticUpdatesSettings3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomaticUpdatesSettings3Vtbl {
        unsafe extern "system" fn NonAdministratorsElevated<Impl: IAutomaticUpdatesSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NonAdministratorsElevated(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonAdministratorsElevated<Impl: IAutomaticUpdatesSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNonAdministratorsElevated(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FeaturedUpdatesEnabled<Impl: IAutomaticUpdatesSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FeaturedUpdatesEnabled(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFeaturedUpdatesEnabled<Impl: IAutomaticUpdatesSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFeaturedUpdatesEnabled(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomaticUpdatesSettings3>, base.5, NonAdministratorsElevated::<Impl, OFFSET>, SetNonAdministratorsElevated::<Impl, OFFSET>, FeaturedUpdatesEnabled::<Impl, OFFSET>, SetFeaturedUpdatesEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICategoryImpl: Sized + IDispatchImpl {
    fn Name();
    fn CategoryID();
    fn Children();
    fn Description();
    fn Image();
    fn Order();
    fn Parent();
    fn Type();
    fn Updates();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICategory {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ICategory";
}
#[cfg(feature = "Win32_System_Com")]
impl ICategoryVtbl {
    pub const fn new<Impl: ICategoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICategoryVtbl {
        unsafe extern "system" fn Name<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CategoryID<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CategoryID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Children(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Image(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Order<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Order(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: ICategoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICategory>, base.5, Name::<Impl, OFFSET>, CategoryID::<Impl, OFFSET>, Children::<Impl, OFFSET>, Description::<Impl, OFFSET>, Image::<Impl, OFFSET>, Order::<Impl, OFFSET>, Parent::<Impl, OFFSET>, Type::<Impl, OFFSET>, Updates::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICategoryCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICategoryCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ICategoryCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl ICategoryCollectionVtbl {
    pub const fn new<Impl: ICategoryCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICategoryCollectionVtbl {
        unsafe extern "system" fn Item<Impl: ICategoryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICategoryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICategoryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICategoryCollection>, base.5, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
pub trait IDownloadCompletedCallbackImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IDownloadCompletedCallback {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadCompletedCallback";
}
impl IDownloadCompletedCallbackVtbl {
    pub const fn new<Impl: IDownloadCompletedCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadCompletedCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IDownloadCompletedCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&downloadjob as *const <IDownloadJob as ::windows::core::Abi>::Abi as *const <IDownloadJob as ::windows::core::DefaultType>::DefaultType), &*(&callbackargs as *const <IDownloadCompletedCallbackArgs as ::windows::core::Abi>::Abi as *const <IDownloadCompletedCallbackArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadCompletedCallback>, base.5, Invoke::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadCompletedCallbackArgsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDownloadCompletedCallbackArgs {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadCompletedCallbackArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallbackArgsVtbl {
    pub const fn new<Impl: IDownloadCompletedCallbackArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadCompletedCallbackArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadCompletedCallbackArgs>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadJobImpl: Sized + IDispatchImpl {
    fn AsyncState();
    fn IsCompleted();
    fn Updates();
    fn CleanUp();
    fn GetProgress();
    fn RequestAbort();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDownloadJob {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadJob";
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadJobVtbl {
    pub const fn new<Impl: IDownloadJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadJobVtbl {
        unsafe extern "system" fn AsyncState<Impl: IDownloadJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Impl: IDownloadJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCompleted(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IDownloadJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Impl: IDownloadJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CleanUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IDownloadJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Impl: IDownloadJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadJob>, base.5, AsyncState::<Impl, OFFSET>, IsCompleted::<Impl, OFFSET>, Updates::<Impl, OFFSET>, CleanUp::<Impl, OFFSET>, GetProgress::<Impl, OFFSET>, RequestAbort::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressImpl: Sized + IDispatchImpl {
    fn CurrentUpdateBytesDownloaded();
    fn CurrentUpdateBytesToDownload();
    fn CurrentUpdateIndex();
    fn PercentComplete();
    fn TotalBytesDownloaded();
    fn TotalBytesToDownload();
    fn GetUpdateResult();
    fn CurrentUpdateDownloadPhase();
    fn CurrentUpdatePercentComplete();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDownloadProgress {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadProgress";
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressVtbl {
    pub const fn new<Impl: IDownloadProgressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadProgressVtbl {
        unsafe extern "system" fn CurrentUpdateBytesDownloaded<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateBytesDownloaded(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateBytesToDownload<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateBytesToDownload(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateIndex<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateIndex(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PercentComplete(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesDownloaded<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalBytesDownloaded(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBytesToDownload<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalBytesToDownload(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(updateindex, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdateDownloadPhase<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPhase) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateDownloadPhase(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Impl: IDownloadProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdatePercentComplete(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadProgress>, base.5, CurrentUpdateBytesDownloaded::<Impl, OFFSET>, CurrentUpdateBytesToDownload::<Impl, OFFSET>, CurrentUpdateIndex::<Impl, OFFSET>, PercentComplete::<Impl, OFFSET>, TotalBytesDownloaded::<Impl, OFFSET>, TotalBytesToDownload::<Impl, OFFSET>, GetUpdateResult::<Impl, OFFSET>, CurrentUpdateDownloadPhase::<Impl, OFFSET>, CurrentUpdatePercentComplete::<Impl, OFFSET>)
    }
}
pub trait IDownloadProgressChangedCallbackImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IDownloadProgressChangedCallback {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadProgressChangedCallback";
}
impl IDownloadProgressChangedCallbackVtbl {
    pub const fn new<Impl: IDownloadProgressChangedCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadProgressChangedCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IDownloadProgressChangedCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&downloadjob as *const <IDownloadJob as ::windows::core::Abi>::Abi as *const <IDownloadJob as ::windows::core::DefaultType>::DefaultType), &*(&callbackargs as *const <IDownloadProgressChangedCallbackArgs as ::windows::core::Abi>::Abi as *const <IDownloadProgressChangedCallbackArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadProgressChangedCallback>, base.5, Invoke::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadProgressChangedCallbackArgsImpl: Sized + IDispatchImpl {
    fn Progress();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDownloadProgressChangedCallbackArgs {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadProgressChangedCallbackArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallbackArgsVtbl {
    pub const fn new<Impl: IDownloadProgressChangedCallbackArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadProgressChangedCallbackArgsVtbl {
        unsafe extern "system" fn Progress<Impl: IDownloadProgressChangedCallbackArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Progress(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadProgressChangedCallbackArgs>, base.5, Progress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDownloadResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn ResultCode();
    fn GetUpdateResult();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDownloadResult {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IDownloadResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IDownloadResultVtbl {
    pub const fn new<Impl: IDownloadResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDownloadResultVtbl {
        unsafe extern "system" fn HResult<Impl: IDownloadResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HResult(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IDownloadResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IDownloadResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(updateindex, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDownloadResult>, base.5, HResult::<Impl, OFFSET>, ResultCode::<Impl, OFFSET>, GetUpdateResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImageInformationImpl: Sized + IDispatchImpl {
    fn AltText();
    fn Height();
    fn Source();
    fn Width();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IImageInformation {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IImageInformation";
}
#[cfg(feature = "Win32_System_Com")]
impl IImageInformationVtbl {
    pub const fn new<Impl: IImageInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageInformationVtbl {
        unsafe extern "system" fn AltText<Impl: IImageInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltText(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IImageInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Impl: IImageInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IImageInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageInformation>, base.5, AltText::<Impl, OFFSET>, Height::<Impl, OFFSET>, Source::<Impl, OFFSET>, Width::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationAgentImpl: Sized + IDispatchImpl {
    fn RecordInstallationResult();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationAgent {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationAgent";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationAgentVtbl {
    pub const fn new<Impl: IInstallationAgentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationAgentVtbl {
        unsafe extern "system" fn RecordInstallationResult<Impl: IInstallationAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installationresultcookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hresult: i32, extendedreportingdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecordInstallationResult(&*(&installationresultcookie as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), hresult, &*(&extendedreportingdata as *const <IStringCollection as ::windows::core::Abi>::Abi as *const <IStringCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationAgent>, base.5, RecordInstallationResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationBehaviorImpl: Sized + IDispatchImpl {
    fn CanRequestUserInput();
    fn Impact();
    fn RebootBehavior();
    fn RequiresNetworkConnectivity();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationBehavior {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationBehavior";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationBehaviorVtbl {
    pub const fn new<Impl: IInstallationBehaviorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationBehaviorVtbl {
        unsafe extern "system" fn CanRequestUserInput<Impl: IInstallationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanRequestUserInput(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Impact<Impl: IInstallationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationImpact) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Impact(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootBehavior<Impl: IInstallationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootBehavior(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiresNetworkConnectivity<Impl: IInstallationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequiresNetworkConnectivity(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationBehavior>, base.5, CanRequestUserInput::<Impl, OFFSET>, Impact::<Impl, OFFSET>, RebootBehavior::<Impl, OFFSET>, RequiresNetworkConnectivity::<Impl, OFFSET>)
    }
}
pub trait IInstallationCompletedCallbackImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IInstallationCompletedCallback {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationCompletedCallback";
}
impl IInstallationCompletedCallbackVtbl {
    pub const fn new<Impl: IInstallationCompletedCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationCompletedCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IInstallationCompletedCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installationjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&installationjob as *const <IInstallationJob as ::windows::core::Abi>::Abi as *const <IInstallationJob as ::windows::core::DefaultType>::DefaultType), &*(&callbackargs as *const <IInstallationCompletedCallbackArgs as ::windows::core::Abi>::Abi as *const <IInstallationCompletedCallbackArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationCompletedCallback>, base.5, Invoke::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationCompletedCallbackArgsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationCompletedCallbackArgs {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationCompletedCallbackArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallbackArgsVtbl {
    pub const fn new<Impl: IInstallationCompletedCallbackArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationCompletedCallbackArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationCompletedCallbackArgs>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationJobImpl: Sized + IDispatchImpl {
    fn AsyncState();
    fn IsCompleted();
    fn Updates();
    fn CleanUp();
    fn GetProgress();
    fn RequestAbort();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationJob {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationJob";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationJobVtbl {
    pub const fn new<Impl: IInstallationJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationJobVtbl {
        unsafe extern "system" fn AsyncState<Impl: IInstallationJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Impl: IInstallationJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCompleted(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IInstallationJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Impl: IInstallationJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CleanUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgress<Impl: IInstallationJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgress(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Impl: IInstallationJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationJob>, base.5, AsyncState::<Impl, OFFSET>, IsCompleted::<Impl, OFFSET>, Updates::<Impl, OFFSET>, CleanUp::<Impl, OFFSET>, GetProgress::<Impl, OFFSET>, RequestAbort::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressImpl: Sized + IDispatchImpl {
    fn CurrentUpdateIndex();
    fn CurrentUpdatePercentComplete();
    fn PercentComplete();
    fn GetUpdateResult();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationProgress {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationProgress";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressVtbl {
    pub const fn new<Impl: IInstallationProgressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationProgressVtbl {
        unsafe extern "system" fn CurrentUpdateIndex<Impl: IInstallationProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdateIndex(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentUpdatePercentComplete<Impl: IInstallationProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentUpdatePercentComplete(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Impl: IInstallationProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PercentComplete(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IInstallationProgressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(updateindex, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationProgress>, base.5, CurrentUpdateIndex::<Impl, OFFSET>, CurrentUpdatePercentComplete::<Impl, OFFSET>, PercentComplete::<Impl, OFFSET>, GetUpdateResult::<Impl, OFFSET>)
    }
}
pub trait IInstallationProgressChangedCallbackImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IInstallationProgressChangedCallback {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationProgressChangedCallback";
}
impl IInstallationProgressChangedCallbackVtbl {
    pub const fn new<Impl: IInstallationProgressChangedCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationProgressChangedCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: IInstallationProgressChangedCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, installationjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&installationjob as *const <IInstallationJob as ::windows::core::Abi>::Abi as *const <IInstallationJob as ::windows::core::DefaultType>::DefaultType), &*(&callbackargs as *const <IInstallationProgressChangedCallbackArgs as ::windows::core::Abi>::Abi as *const <IInstallationProgressChangedCallbackArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationProgressChangedCallback>, base.5, Invoke::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationProgressChangedCallbackArgsImpl: Sized + IDispatchImpl {
    fn Progress();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationProgressChangedCallbackArgs {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationProgressChangedCallbackArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallbackArgsVtbl {
    pub const fn new<Impl: IInstallationProgressChangedCallbackArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationProgressChangedCallbackArgsVtbl {
        unsafe extern "system" fn Progress<Impl: IInstallationProgressChangedCallbackArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Progress(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationProgressChangedCallbackArgs>, base.5, Progress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInstallationResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn RebootRequired();
    fn ResultCode();
    fn GetUpdateResult();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInstallationResult {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInstallationResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IInstallationResultVtbl {
    pub const fn new<Impl: IInstallationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInstallationResultVtbl {
        unsafe extern "system" fn HResult<Impl: IInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HResult(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Impl: IInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateResult<Impl: IInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateResult(updateindex, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInstallationResult>, base.5, HResult::<Impl, OFFSET>, RebootRequired::<Impl, OFFSET>, ResultCode::<Impl, OFFSET>, GetUpdateResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInvalidProductLicenseExceptionImpl: Sized + IUpdateExceptionImpl + IDispatchImpl {
    fn Product();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IInvalidProductLicenseException {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IInvalidProductLicenseException";
}
#[cfg(feature = "Win32_System_Com")]
impl IInvalidProductLicenseExceptionVtbl {
    pub const fn new<Impl: IInvalidProductLicenseExceptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInvalidProductLicenseExceptionVtbl {
        unsafe extern "system" fn Product<Impl: IInvalidProductLicenseExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Product(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInvalidProductLicenseException>, base.5, Product::<Impl, OFFSET>)
    }
}
pub trait ISearchCompletedCallbackImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for ISearchCompletedCallback {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ISearchCompletedCallback";
}
impl ISearchCompletedCallbackVtbl {
    pub const fn new<Impl: ISearchCompletedCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchCompletedCallbackVtbl {
        unsafe extern "system" fn Invoke<Impl: ISearchCompletedCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchjob: ::windows::core::RawPtr, callbackargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(&*(&searchjob as *const <ISearchJob as ::windows::core::Abi>::Abi as *const <ISearchJob as ::windows::core::DefaultType>::DefaultType), &*(&callbackargs as *const <ISearchCompletedCallbackArgs as ::windows::core::Abi>::Abi as *const <ISearchCompletedCallbackArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchCompletedCallback>, base.5, Invoke::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCompletedCallbackArgsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISearchCompletedCallbackArgs {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ISearchCompletedCallbackArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallbackArgsVtbl {
    pub const fn new<Impl: ISearchCompletedCallbackArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchCompletedCallbackArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchCompletedCallbackArgs>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchJobImpl: Sized + IDispatchImpl {
    fn AsyncState();
    fn IsCompleted();
    fn CleanUp();
    fn RequestAbort();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISearchJob {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ISearchJob";
}
#[cfg(feature = "Win32_System_Com")]
impl ISearchJobVtbl {
    pub const fn new<Impl: ISearchJobImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchJobVtbl {
        unsafe extern "system" fn AsyncState<Impl: ISearchJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AsyncState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCompleted<Impl: ISearchJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCompleted(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanUp<Impl: ISearchJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CleanUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAbort<Impl: ISearchJobImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchJob>, base.5, AsyncState::<Impl, OFFSET>, IsCompleted::<Impl, OFFSET>, CleanUp::<Impl, OFFSET>, RequestAbort::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchResultImpl: Sized + IDispatchImpl {
    fn ResultCode();
    fn RootCategories();
    fn Updates();
    fn Warnings();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISearchResult {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ISearchResult";
}
#[cfg(feature = "Win32_System_Com")]
impl ISearchResultVtbl {
    pub const fn new<Impl: ISearchResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchResultVtbl {
        unsafe extern "system" fn ResultCode<Impl: ISearchResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootCategories<Impl: ISearchResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RootCategories(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: ISearchResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Warnings<Impl: ISearchResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Warnings(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchResult>, base.5, ResultCode::<Impl, OFFSET>, RootCategories::<Impl, OFFSET>, Updates::<Impl, OFFSET>, Warnings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStringCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn SetItem();
    fn _NewEnum();
    fn Count();
    fn ReadOnly();
    fn Add();
    fn Clear();
    fn Copy();
    fn Insert();
    fn RemoveAt();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStringCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IStringCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IStringCollectionVtbl {
    pub const fn new<Impl: IStringCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStringCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetItem(index, &*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Insert(index, &*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStringCollection>, base.5, Item::<Impl, OFFSET>, SetItem::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>, ReadOnly::<Impl, OFFSET>, Add::<Impl, OFFSET>, Clear::<Impl, OFFSET>, Copy::<Impl, OFFSET>, Insert::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISystemInformationImpl: Sized + IDispatchImpl {
    fn OemHardwareSupportLink();
    fn RebootRequired();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISystemInformation {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.ISystemInformation";
}
#[cfg(feature = "Win32_System_Com")]
impl ISystemInformationVtbl {
    pub const fn new<Impl: ISystemInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemInformationVtbl {
        unsafe extern "system" fn OemHardwareSupportLink<Impl: ISystemInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OemHardwareSupportLink(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Impl: ISystemInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemInformation>, base.5, OemHardwareSupportLink::<Impl, OFFSET>, RebootRequired::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateImpl: Sized + IDispatchImpl {
    fn Title();
    fn AutoSelectOnWebSites();
    fn BundledUpdates();
    fn CanRequireSource();
    fn Categories();
    fn Deadline();
    fn DeltaCompressedContentAvailable();
    fn DeltaCompressedContentPreferred();
    fn Description();
    fn EulaAccepted();
    fn EulaText();
    fn HandlerID();
    fn Identity();
    fn Image();
    fn InstallationBehavior();
    fn IsBeta();
    fn IsDownloaded();
    fn IsHidden();
    fn SetIsHidden();
    fn IsInstalled();
    fn IsMandatory();
    fn IsUninstallable();
    fn Languages();
    fn LastDeploymentChangeTime();
    fn MaxDownloadSize();
    fn MinDownloadSize();
    fn MoreInfoUrls();
    fn MsrcSeverity();
    fn RecommendedCpuSpeed();
    fn RecommendedHardDiskSpace();
    fn RecommendedMemory();
    fn ReleaseNotes();
    fn SecurityBulletinIDs();
    fn SupersededUpdateIDs();
    fn SupportUrl();
    fn Type();
    fn UninstallationNotes();
    fn UninstallationBehavior();
    fn UninstallationSteps();
    fn KBArticleIDs();
    fn AcceptEula();
    fn DeploymentAction();
    fn CopyFromCache();
    fn DownloadPriority();
    fn DownloadContents();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdate {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdate";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateVtbl {
    pub const fn new<Impl: IUpdateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateVtbl {
        unsafe extern "system" fn Title<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoSelectOnWebSites<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoSelectOnWebSites(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BundledUpdates<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BundledUpdates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRequireSource<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanRequireSource(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Categories<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Categories(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deadline<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Deadline(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentAvailable<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeltaCompressedContentAvailable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeltaCompressedContentPreferred<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeltaCompressedContentPreferred(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaAccepted<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EulaAccepted(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EulaText<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EulaText(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandlerID<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandlerID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identity<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Identity(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Image(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallationBehavior<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallationBehavior(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBeta<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBeta(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDownloaded<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDownloaded(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHidden(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsHidden(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInstalled<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInstalled(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMandatory<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMandatory(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUninstallable<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUninstallable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Languages<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Languages(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDeploymentChangeTime<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastDeploymentChangeTime(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDownloadSize<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxDownloadSize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinDownloadSize<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinDownloadSize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoreInfoUrls<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MoreInfoUrls(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MsrcSeverity<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MsrcSeverity(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedCpuSpeed<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecommendedCpuSpeed(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedHardDiskSpace<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecommendedHardDiskSpace(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecommendedMemory<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecommendedMemory(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseNotes<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseNotes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityBulletinIDs<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SecurityBulletinIDs(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupersededUpdateIDs<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupersededUpdateIDs(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportUrl(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UninstallationNotes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationBehavior<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UninstallationBehavior(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UninstallationSteps(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KBArticleIDs<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KBArticleIDs(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptEula<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcceptEula() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentAction<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DeploymentAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeploymentAction(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyFromCache<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, toextractcabfiles: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyFromCache(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), toextractcabfiles) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadPriority<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadPriority(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadContents<Impl: IUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadContents(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUpdate>,
            base.5,
            Title::<Impl, OFFSET>,
            AutoSelectOnWebSites::<Impl, OFFSET>,
            BundledUpdates::<Impl, OFFSET>,
            CanRequireSource::<Impl, OFFSET>,
            Categories::<Impl, OFFSET>,
            Deadline::<Impl, OFFSET>,
            DeltaCompressedContentAvailable::<Impl, OFFSET>,
            DeltaCompressedContentPreferred::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            EulaAccepted::<Impl, OFFSET>,
            EulaText::<Impl, OFFSET>,
            HandlerID::<Impl, OFFSET>,
            Identity::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
            InstallationBehavior::<Impl, OFFSET>,
            IsBeta::<Impl, OFFSET>,
            IsDownloaded::<Impl, OFFSET>,
            IsHidden::<Impl, OFFSET>,
            SetIsHidden::<Impl, OFFSET>,
            IsInstalled::<Impl, OFFSET>,
            IsMandatory::<Impl, OFFSET>,
            IsUninstallable::<Impl, OFFSET>,
            Languages::<Impl, OFFSET>,
            LastDeploymentChangeTime::<Impl, OFFSET>,
            MaxDownloadSize::<Impl, OFFSET>,
            MinDownloadSize::<Impl, OFFSET>,
            MoreInfoUrls::<Impl, OFFSET>,
            MsrcSeverity::<Impl, OFFSET>,
            RecommendedCpuSpeed::<Impl, OFFSET>,
            RecommendedHardDiskSpace::<Impl, OFFSET>,
            RecommendedMemory::<Impl, OFFSET>,
            ReleaseNotes::<Impl, OFFSET>,
            SecurityBulletinIDs::<Impl, OFFSET>,
            SupersededUpdateIDs::<Impl, OFFSET>,
            SupportUrl::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            UninstallationNotes::<Impl, OFFSET>,
            UninstallationBehavior::<Impl, OFFSET>,
            UninstallationSteps::<Impl, OFFSET>,
            KBArticleIDs::<Impl, OFFSET>,
            AcceptEula::<Impl, OFFSET>,
            DeploymentAction::<Impl, OFFSET>,
            CopyFromCache::<Impl, OFFSET>,
            DownloadPriority::<Impl, OFFSET>,
            DownloadContents::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate2Impl: Sized + IUpdateImpl + IDispatchImpl {
    fn RebootRequired();
    fn IsPresent();
    fn CveIDs();
    fn CopyToCache();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdate2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdate2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate2Vtbl {
    pub const fn new<Impl: IUpdate2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdate2Vtbl {
        unsafe extern "system" fn RebootRequired<Impl: IUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Impl: IUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPresent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Impl: IUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CveIDs(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Impl: IUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiles: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyToCache(&*(&pfiles as *const <IStringCollection as ::windows::core::Abi>::Abi as *const <IStringCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdate2>, base.5, RebootRequired::<Impl, OFFSET>, IsPresent::<Impl, OFFSET>, CveIDs::<Impl, OFFSET>, CopyToCache::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate3Impl: Sized + IUpdate2Impl + IUpdateImpl + IDispatchImpl {
    fn BrowseOnly();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdate3 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdate3";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate3Vtbl {
    pub const fn new<Impl: IUpdate3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdate3Vtbl {
        unsafe extern "system" fn BrowseOnly<Impl: IUpdate3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrowseOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdate3>, base.5, BrowseOnly::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate4Impl: Sized + IUpdate3Impl + IUpdate2Impl + IUpdateImpl + IDispatchImpl {
    fn PerUser();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdate4 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdate4";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate4Vtbl {
    pub const fn new<Impl: IUpdate4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdate4Vtbl {
        unsafe extern "system" fn PerUser<Impl: IUpdate4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PerUser(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdate4>, base.5, PerUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdate5Impl: Sized + IUpdate4Impl + IUpdate3Impl + IUpdate2Impl + IUpdateImpl + IDispatchImpl {
    fn AutoSelection();
    fn AutoDownload();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdate5 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdate5";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdate5Vtbl {
    pub const fn new<Impl: IUpdate5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdate5Vtbl {
        unsafe extern "system" fn AutoSelection<Impl: IUpdate5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoSelection(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Impl: IUpdate5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoDownload(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdate5>, base.5, AutoSelection::<Impl, OFFSET>, AutoDownload::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn SetItem();
    fn _NewEnum();
    fn Count();
    fn ReadOnly();
    fn Add();
    fn Clear();
    fn Copy();
    fn Insert();
    fn RemoveAt();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateCollectionVtbl {
    pub const fn new<Impl: IUpdateCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItem<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetItem(index, &*(&value as *const <IUpdate as ::windows::core::Abi>::Abi as *const <IUpdate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&value as *const <IUpdate as ::windows::core::Abi>::Abi as *const <IUpdate as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Insert(index, &*(&value as *const <IUpdate as ::windows::core::Abi>::Abi as *const <IUpdate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: IUpdateCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAt(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateCollection>, base.5, Item::<Impl, OFFSET>, SetItem::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>, ReadOnly::<Impl, OFFSET>, Add::<Impl, OFFSET>, Clear::<Impl, OFFSET>, Copy::<Impl, OFFSET>, Insert::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContentImpl: Sized + IDispatchImpl {
    fn DownloadUrl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateDownloadContent {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateDownloadContent";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContentVtbl {
    pub const fn new<Impl: IUpdateDownloadContentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateDownloadContentVtbl {
        unsafe extern "system" fn DownloadUrl<Impl: IUpdateDownloadContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadUrl(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateDownloadContent>, base.5, DownloadUrl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContent2Impl: Sized + IUpdateDownloadContentImpl + IDispatchImpl {
    fn IsDeltaCompressedContent();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateDownloadContent2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateDownloadContent2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent2Vtbl {
    pub const fn new<Impl: IUpdateDownloadContent2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateDownloadContent2Vtbl {
        unsafe extern "system" fn IsDeltaCompressedContent<Impl: IUpdateDownloadContent2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDeltaCompressedContent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateDownloadContent2>, base.5, IsDeltaCompressedContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadContentCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateDownloadContentCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateDownloadContentCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContentCollectionVtbl {
    pub const fn new<Impl: IUpdateDownloadContentCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateDownloadContentCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IUpdateDownloadContentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateDownloadContentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateDownloadContentCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateDownloadContentCollection>, base.5, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloadResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn ResultCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateDownloadResult {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateDownloadResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadResultVtbl {
    pub const fn new<Impl: IUpdateDownloadResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateDownloadResultVtbl {
        unsafe extern "system" fn HResult<Impl: IUpdateDownloadResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HResult(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IUpdateDownloadResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateDownloadResult>, base.5, HResult::<Impl, OFFSET>, ResultCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateDownloaderImpl: Sized + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn IsForced();
    fn SetIsForced();
    fn Priority();
    fn SetPriority();
    fn Updates();
    fn SetUpdates();
    fn BeginDownload();
    fn Download();
    fn EndDownload();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateDownloader {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateDownloader";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloaderVtbl {
    pub const fn new<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateDownloaderVtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientApplicationID(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForced<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsForced(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsForced(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Priority(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: DownloadPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPriority(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpdates(&*(&value as *const <IUpdateCollection as ::windows::core::Abi>::Abi as *const <IUpdateCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDownload<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDownload(
                &*(&onprogresschanged as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&oncompleted as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&state as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&retval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Download(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDownload<Impl: IUpdateDownloaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndDownload(&*(&value as *const <IDownloadJob as ::windows::core::Abi>::Abi as *const <IDownloadJob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateDownloader>, base.5, ClientApplicationID::<Impl, OFFSET>, SetClientApplicationID::<Impl, OFFSET>, IsForced::<Impl, OFFSET>, SetIsForced::<Impl, OFFSET>, Priority::<Impl, OFFSET>, SetPriority::<Impl, OFFSET>, Updates::<Impl, OFFSET>, SetUpdates::<Impl, OFFSET>, BeginDownload::<Impl, OFFSET>, Download::<Impl, OFFSET>, EndDownload::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateExceptionImpl: Sized + IDispatchImpl {
    fn Message();
    fn HResult();
    fn Context();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateException {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateException";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateExceptionVtbl {
    pub const fn new<Impl: IUpdateExceptionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateExceptionVtbl {
        unsafe extern "system" fn Message<Impl: IUpdateExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Impl: IUpdateExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HResult(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: IUpdateExceptionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateExceptionContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Context(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateException>, base.5, Message::<Impl, OFFSET>, HResult::<Impl, OFFSET>, Context::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateExceptionCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateExceptionCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateExceptionCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateExceptionCollectionVtbl {
    pub const fn new<Impl: IUpdateExceptionCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateExceptionCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IUpdateExceptionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateExceptionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateExceptionCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateExceptionCollection>, base.5, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntryImpl: Sized + IDispatchImpl {
    fn Operation();
    fn ResultCode();
    fn HResult();
    fn Date();
    fn UpdateIdentity();
    fn Title();
    fn Description();
    fn UnmappedResultCode();
    fn ClientApplicationID();
    fn ServerSelection();
    fn ServiceID();
    fn UninstallationSteps();
    fn UninstallationNotes();
    fn SupportUrl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateHistoryEntry {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateHistoryEntry";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntryVtbl {
    pub const fn new<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateHistoryEntryVtbl {
        unsafe extern "system" fn Operation<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Operation(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HResult<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HResult(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Date<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Date(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateIdentity<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateIdentity(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnmappedResultCode<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnmappedResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerSelection<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerSelection(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationSteps<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UninstallationSteps(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallationNotes<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UninstallationNotes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUrl<Impl: IUpdateHistoryEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportUrl(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUpdateHistoryEntry>,
            base.5,
            Operation::<Impl, OFFSET>,
            ResultCode::<Impl, OFFSET>,
            HResult::<Impl, OFFSET>,
            Date::<Impl, OFFSET>,
            UpdateIdentity::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            UnmappedResultCode::<Impl, OFFSET>,
            ClientApplicationID::<Impl, OFFSET>,
            ServerSelection::<Impl, OFFSET>,
            ServiceID::<Impl, OFFSET>,
            UninstallationSteps::<Impl, OFFSET>,
            UninstallationNotes::<Impl, OFFSET>,
            SupportUrl::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntry2Impl: Sized + IUpdateHistoryEntryImpl + IDispatchImpl {
    fn Categories();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateHistoryEntry2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateHistoryEntry2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry2Vtbl {
    pub const fn new<Impl: IUpdateHistoryEntry2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateHistoryEntry2Vtbl {
        unsafe extern "system" fn Categories<Impl: IUpdateHistoryEntry2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Categories(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateHistoryEntry2>, base.5, Categories::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateHistoryEntryCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateHistoryEntryCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateHistoryEntryCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntryCollectionVtbl {
    pub const fn new<Impl: IUpdateHistoryEntryCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateHistoryEntryCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IUpdateHistoryEntryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateHistoryEntryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateHistoryEntryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateHistoryEntryCollection>, base.5, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateIdentityImpl: Sized + IDispatchImpl {
    fn RevisionNumber();
    fn UpdateID();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateIdentity {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateIdentity";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateIdentityVtbl {
    pub const fn new<Impl: IUpdateIdentityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateIdentityVtbl {
        unsafe extern "system" fn RevisionNumber<Impl: IUpdateIdentityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevisionNumber(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateID<Impl: IUpdateIdentityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateIdentity>, base.5, RevisionNumber::<Impl, OFFSET>, UpdateID::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstallationResultImpl: Sized + IDispatchImpl {
    fn HResult();
    fn RebootRequired();
    fn ResultCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateInstallationResult {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateInstallationResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstallationResultVtbl {
    pub const fn new<Impl: IUpdateInstallationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateInstallationResultVtbl {
        unsafe extern "system" fn HResult<Impl: IUpdateInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HResult(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequired<Impl: IUpdateInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultCode<Impl: IUpdateInstallationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultCode(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateInstallationResult>, base.5, HResult::<Impl, OFFSET>, RebootRequired::<Impl, OFFSET>, ResultCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstallerImpl: Sized + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn IsForced();
    fn SetIsForced();
    fn ParentHwnd();
    fn SetParentHwnd();
    fn SetParentWindow();
    fn ParentWindow();
    fn Updates();
    fn SetUpdates();
    fn BeginInstall();
    fn BeginUninstall();
    fn EndInstall();
    fn EndUninstall();
    fn Install();
    fn RunWizard();
    fn IsBusy();
    fn Uninstall();
    fn AllowSourcePrompts();
    fn SetAllowSourcePrompts();
    fn RebootRequiredBeforeInstallation();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateInstaller {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateInstaller";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstallerVtbl {
    pub const fn new<Impl: IUpdateInstallerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateInstallerVtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientApplicationID(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsForced<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsForced(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsForced<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsForced(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentHwnd<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParentHwnd(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentHwnd<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParentHwnd(&*(&value as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParentWindow(&*(&value as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentWindow<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParentWindow(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Updates<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdates<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpdates(&*(&value as *const <IUpdateCollection as ::windows::core::Abi>::Abi as *const <IUpdateCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginInstall<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginInstall(
                &*(&onprogresschanged as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&oncompleted as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&state as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&retval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUninstall<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginUninstall(
                &*(&onprogresschanged as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&oncompleted as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&state as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&retval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndInstall<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndInstall(&*(&value as *const <IInstallationJob as ::windows::core::Abi>::Abi as *const <IInstallationJob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUninstall<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndUninstall(&*(&value as *const <IInstallationJob as ::windows::core::Abi>::Abi as *const <IInstallationJob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Install(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWizard<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dialogtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunWizard(&*(&dialogtitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBusy<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBusy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninstall<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uninstall(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSourcePrompts<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowSourcePrompts(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowSourcePrompts<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllowSourcePrompts(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RebootRequiredBeforeInstallation<Impl: IUpdateInstallerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootRequiredBeforeInstallation(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUpdateInstaller>,
            base.5,
            ClientApplicationID::<Impl, OFFSET>,
            SetClientApplicationID::<Impl, OFFSET>,
            IsForced::<Impl, OFFSET>,
            SetIsForced::<Impl, OFFSET>,
            ParentHwnd::<Impl, OFFSET>,
            SetParentHwnd::<Impl, OFFSET>,
            SetParentWindow::<Impl, OFFSET>,
            ParentWindow::<Impl, OFFSET>,
            Updates::<Impl, OFFSET>,
            SetUpdates::<Impl, OFFSET>,
            BeginInstall::<Impl, OFFSET>,
            BeginUninstall::<Impl, OFFSET>,
            EndInstall::<Impl, OFFSET>,
            EndUninstall::<Impl, OFFSET>,
            Install::<Impl, OFFSET>,
            RunWizard::<Impl, OFFSET>,
            IsBusy::<Impl, OFFSET>,
            Uninstall::<Impl, OFFSET>,
            AllowSourcePrompts::<Impl, OFFSET>,
            SetAllowSourcePrompts::<Impl, OFFSET>,
            RebootRequiredBeforeInstallation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller2Impl: Sized + IUpdateInstallerImpl + IDispatchImpl {
    fn ForceQuiet();
    fn SetForceQuiet();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateInstaller2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateInstaller2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller2Vtbl {
    pub const fn new<Impl: IUpdateInstaller2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateInstaller2Vtbl {
        unsafe extern "system" fn ForceQuiet<Impl: IUpdateInstaller2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForceQuiet(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceQuiet<Impl: IUpdateInstaller2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetForceQuiet(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateInstaller2>, base.5, ForceQuiet::<Impl, OFFSET>, SetForceQuiet::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller3Impl: Sized + IUpdateInstaller2Impl + IUpdateInstallerImpl + IDispatchImpl {
    fn AttemptCloseAppsIfNecessary();
    fn SetAttemptCloseAppsIfNecessary();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateInstaller3 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateInstaller3";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller3Vtbl {
    pub const fn new<Impl: IUpdateInstaller3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateInstaller3Vtbl {
        unsafe extern "system" fn AttemptCloseAppsIfNecessary<Impl: IUpdateInstaller3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttemptCloseAppsIfNecessary(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttemptCloseAppsIfNecessary<Impl: IUpdateInstaller3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAttemptCloseAppsIfNecessary(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateInstaller3>, base.5, AttemptCloseAppsIfNecessary::<Impl, OFFSET>, SetAttemptCloseAppsIfNecessary::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateInstaller4Impl: Sized + IUpdateInstaller3Impl + IUpdateInstaller2Impl + IUpdateInstallerImpl + IDispatchImpl {
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateInstaller4 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateInstaller4";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller4Vtbl {
    pub const fn new<Impl: IUpdateInstaller4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateInstaller4Vtbl {
        unsafe extern "system" fn Commit<Impl: IUpdateInstaller4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commit(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateInstaller4>, base.5, Commit::<Impl, OFFSET>)
    }
}
pub trait IUpdateLockdownImpl: Sized {
    fn LockDown();
}
impl ::windows::core::RuntimeName for IUpdateLockdown {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateLockdown";
}
impl IUpdateLockdownVtbl {
    pub const fn new<Impl: IUpdateLockdownImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateLockdownVtbl {
        unsafe extern "system" fn LockDown<Impl: IUpdateLockdownImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LockDown(flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateLockdown>, base.5, LockDown::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcherImpl: Sized + IDispatchImpl {
    fn CanAutomaticallyUpgradeService();
    fn SetCanAutomaticallyUpgradeService();
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn IncludePotentiallySupersededUpdates();
    fn SetIncludePotentiallySupersededUpdates();
    fn ServerSelection();
    fn SetServerSelection();
    fn BeginSearch();
    fn EndSearch();
    fn EscapeString();
    fn QueryHistory();
    fn Search();
    fn Online();
    fn SetOnline();
    fn GetTotalHistoryCount();
    fn ServiceID();
    fn SetServiceID();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateSearcher {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateSearcher";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcherVtbl {
    pub const fn new<Impl: IUpdateSearcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateSearcherVtbl {
        unsafe extern "system" fn CanAutomaticallyUpgradeService<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanAutomaticallyUpgradeService(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanAutomaticallyUpgradeService<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCanAutomaticallyUpgradeService(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientApplicationID(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludePotentiallySupersededUpdates<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IncludePotentiallySupersededUpdates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludePotentiallySupersededUpdates<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIncludePotentiallySupersededUpdates(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerSelection<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerSelection(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerSelection<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ServerSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServerSelection(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSearch<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginSearch(
                &*(&criteria as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&oncompleted as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&state as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&retval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSearch<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchjob: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndSearch(&*(&searchjob as *const <ISearchJob as ::windows::core::Abi>::Abi as *const <ISearchJob as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeString<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unescaped: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EscapeString(&*(&unescaped as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startindex: i32, count: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryHistory(startindex, count, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Search<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Search(&*(&criteria as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Online<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Online(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOnline<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOnline(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalHistoryCount<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTotalHistoryCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceID<Impl: IUpdateSearcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetServiceID(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUpdateSearcher>,
            base.5,
            CanAutomaticallyUpgradeService::<Impl, OFFSET>,
            SetCanAutomaticallyUpgradeService::<Impl, OFFSET>,
            ClientApplicationID::<Impl, OFFSET>,
            SetClientApplicationID::<Impl, OFFSET>,
            IncludePotentiallySupersededUpdates::<Impl, OFFSET>,
            SetIncludePotentiallySupersededUpdates::<Impl, OFFSET>,
            ServerSelection::<Impl, OFFSET>,
            SetServerSelection::<Impl, OFFSET>,
            BeginSearch::<Impl, OFFSET>,
            EndSearch::<Impl, OFFSET>,
            EscapeString::<Impl, OFFSET>,
            QueryHistory::<Impl, OFFSET>,
            Search::<Impl, OFFSET>,
            Online::<Impl, OFFSET>,
            SetOnline::<Impl, OFFSET>,
            GetTotalHistoryCount::<Impl, OFFSET>,
            ServiceID::<Impl, OFFSET>,
            SetServiceID::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher2Impl: Sized + IUpdateSearcherImpl + IDispatchImpl {
    fn IgnoreDownloadPriority();
    fn SetIgnoreDownloadPriority();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateSearcher2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateSearcher2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher2Vtbl {
    pub const fn new<Impl: IUpdateSearcher2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateSearcher2Vtbl {
        unsafe extern "system" fn IgnoreDownloadPriority<Impl: IUpdateSearcher2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IgnoreDownloadPriority(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreDownloadPriority<Impl: IUpdateSearcher2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIgnoreDownloadPriority(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateSearcher2>, base.5, IgnoreDownloadPriority::<Impl, OFFSET>, SetIgnoreDownloadPriority::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSearcher3Impl: Sized + IUpdateSearcher2Impl + IUpdateSearcherImpl + IDispatchImpl {
    fn SearchScope();
    fn SetSearchScope();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateSearcher3 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateSearcher3";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher3Vtbl {
    pub const fn new<Impl: IUpdateSearcher3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateSearcher3Vtbl {
        unsafe extern "system" fn SearchScope<Impl: IUpdateSearcher3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut SearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchScope(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchScope<Impl: IUpdateSearcher3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SearchScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSearchScope(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateSearcher3>, base.5, SearchScope::<Impl, OFFSET>, SetSearchScope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceImpl: Sized + IDispatchImpl {
    fn Name();
    fn ContentValidationCert();
    fn ExpirationDate();
    fn IsManaged();
    fn IsRegisteredWithAU();
    fn IssueDate();
    fn OffersWindowsUpdates();
    fn RedirectUrls();
    fn ServiceID();
    fn IsScanPackageService();
    fn CanRegisterWithAU();
    fn ServiceUrl();
    fn SetupPrefix();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateService {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateService";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceVtbl {
    pub const fn new<Impl: IUpdateServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateServiceVtbl {
        unsafe extern "system" fn Name<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentValidationCert<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentValidationCert(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationDate<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpirationDate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsManaged<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsManaged(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRegisteredWithAU<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRegisteredWithAU(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueDate<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IssueDate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffersWindowsUpdates<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OffersWindowsUpdates(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectUrls<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedirectUrls(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanPackageService<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScanPackageService(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRegisterWithAU<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanRegisterWithAU(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceUrl<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceUrl(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetupPrefix<Impl: IUpdateServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetupPrefix(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUpdateService>,
            base.5,
            Name::<Impl, OFFSET>,
            ContentValidationCert::<Impl, OFFSET>,
            ExpirationDate::<Impl, OFFSET>,
            IsManaged::<Impl, OFFSET>,
            IsRegisteredWithAU::<Impl, OFFSET>,
            IssueDate::<Impl, OFFSET>,
            OffersWindowsUpdates::<Impl, OFFSET>,
            RedirectUrls::<Impl, OFFSET>,
            ServiceID::<Impl, OFFSET>,
            IsScanPackageService::<Impl, OFFSET>,
            CanRegisterWithAU::<Impl, OFFSET>,
            ServiceUrl::<Impl, OFFSET>,
            SetupPrefix::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateService2Impl: Sized + IUpdateServiceImpl + IDispatchImpl {
    fn IsDefaultAUService();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateService2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateService2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService2Vtbl {
    pub const fn new<Impl: IUpdateService2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateService2Vtbl {
        unsafe extern "system" fn IsDefaultAUService<Impl: IUpdateService2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDefaultAUService(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateService2>, base.5, IsDefaultAUService::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateServiceCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateServiceCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceCollectionVtbl {
    pub const fn new<Impl: IUpdateServiceCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateServiceCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IUpdateServiceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IUpdateServiceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IUpdateServiceCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateServiceCollection>, base.5, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceManagerImpl: Sized + IDispatchImpl {
    fn Services();
    fn AddService();
    fn RegisterServiceWithAU();
    fn RemoveService();
    fn UnregisterServiceWithAU();
    fn AddScanPackageService();
    fn SetOption();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateServiceManager {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateServiceManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManagerVtbl {
    pub const fn new<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateServiceManagerVtbl {
        unsafe extern "system" fn Services<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Services(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddService(&*(&serviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&authorizationcabpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterServiceWithAU<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterServiceWithAU(&*(&serviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveService<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveService(&*(&serviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterServiceWithAU<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterServiceWithAU(&*(&serviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddScanPackageService<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scanfilelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, ppservice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddScanPackageService(&*(&servicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&scanfilelocation as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&ppservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Impl: IUpdateServiceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOption(&*(&optionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&optionvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateServiceManager>, base.5, Services::<Impl, OFFSET>, AddService::<Impl, OFFSET>, RegisterServiceWithAU::<Impl, OFFSET>, RemoveService::<Impl, OFFSET>, UnregisterServiceWithAU::<Impl, OFFSET>, AddScanPackageService::<Impl, OFFSET>, SetOption::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceManager2Impl: Sized + IUpdateServiceManagerImpl + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn QueryServiceRegistration();
    fn AddService2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateServiceManager2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateServiceManager2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager2Vtbl {
    pub const fn new<Impl: IUpdateServiceManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateServiceManager2Vtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateServiceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateServiceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientApplicationID(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryServiceRegistration<Impl: IUpdateServiceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryServiceRegistration(&*(&serviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddService2<Impl: IUpdateServiceManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddService2(&*(&serviceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), flags, &*(&authorizationcabpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateServiceManager2>, base.5, ClientApplicationID::<Impl, OFFSET>, SetClientApplicationID::<Impl, OFFSET>, QueryServiceRegistration::<Impl, OFFSET>, AddService2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateServiceRegistrationImpl: Sized + IDispatchImpl {
    fn RegistrationState();
    fn ServiceID();
    fn IsPendingRegistrationWithAU();
    fn Service();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateServiceRegistration {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateServiceRegistration";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceRegistrationVtbl {
    pub const fn new<Impl: IUpdateServiceRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateServiceRegistrationVtbl {
        unsafe extern "system" fn RegistrationState<Impl: IUpdateServiceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegistrationState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceID<Impl: IUpdateServiceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPendingRegistrationWithAU<Impl: IUpdateServiceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPendingRegistrationWithAU(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Service<Impl: IUpdateServiceRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Service(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateServiceRegistration>, base.5, RegistrationState::<Impl, OFFSET>, ServiceID::<Impl, OFFSET>, IsPendingRegistrationWithAU::<Impl, OFFSET>, Service::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSessionImpl: Sized + IDispatchImpl {
    fn ClientApplicationID();
    fn SetClientApplicationID();
    fn ReadOnly();
    fn WebProxy();
    fn SetWebProxy();
    fn CreateUpdateSearcher();
    fn CreateUpdateDownloader();
    fn CreateUpdateInstaller();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateSession {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateSession";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSessionVtbl {
    pub const fn new<Impl: IUpdateSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateSessionVtbl {
        unsafe extern "system" fn ClientApplicationID<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClientApplicationID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientApplicationID<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientApplicationID(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxy<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebProxy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxy<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetWebProxy(&*(&value as *const <IWebProxy as ::windows::core::Abi>::Abi as *const <IWebProxy as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateSearcher<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUpdateSearcher(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateDownloader<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUpdateDownloader(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Impl: IUpdateSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUpdateInstaller(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateSession>, base.5, ClientApplicationID::<Impl, OFFSET>, SetClientApplicationID::<Impl, OFFSET>, ReadOnly::<Impl, OFFSET>, WebProxy::<Impl, OFFSET>, SetWebProxy::<Impl, OFFSET>, CreateUpdateSearcher::<Impl, OFFSET>, CreateUpdateDownloader::<Impl, OFFSET>, CreateUpdateInstaller::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession2Impl: Sized + IUpdateSessionImpl + IDispatchImpl {
    fn UserLocale();
    fn SetUserLocale();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateSession2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateSession2";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession2Vtbl {
    pub const fn new<Impl: IUpdateSession2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateSession2Vtbl {
        unsafe extern "system" fn UserLocale<Impl: IUpdateSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserLocale(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserLocale<Impl: IUpdateSession2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUserLocale(lcid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateSession2>, base.5, UserLocale::<Impl, OFFSET>, SetUserLocale::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUpdateSession3Impl: Sized + IUpdateSession2Impl + IUpdateSessionImpl + IDispatchImpl {
    fn CreateUpdateServiceManager();
    fn QueryHistory();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IUpdateSession3 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IUpdateSession3";
}
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession3Vtbl {
    pub const fn new<Impl: IUpdateSession3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUpdateSession3Vtbl {
        unsafe extern "system" fn CreateUpdateServiceManager<Impl: IUpdateSession3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUpdateServiceManager(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHistory<Impl: IUpdateSession3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, startindex: i32, count: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryHistory(&*(&criteria as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), startindex, count, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUpdateSession3>, base.5, CreateUpdateServiceManager::<Impl, OFFSET>, QueryHistory::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebProxyImpl: Sized + IDispatchImpl {
    fn Address();
    fn SetAddress();
    fn BypassList();
    fn SetBypassList();
    fn BypassProxyOnLocal();
    fn SetBypassProxyOnLocal();
    fn ReadOnly();
    fn UserName();
    fn SetUserName();
    fn SetPassword();
    fn PromptForCredentials();
    fn PromptForCredentialsFromHwnd();
    fn AutoDetect();
    fn SetAutoDetect();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWebProxy {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWebProxy";
}
#[cfg(feature = "Win32_System_Com")]
impl IWebProxyVtbl {
    pub const fn new<Impl: IWebProxyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebProxyVtbl {
        unsafe extern "system" fn Address<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAddress<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAddress(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BypassList<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BypassList(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassList<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBypassList(&*(&value as *const <IStringCollection as ::windows::core::Abi>::Abi as *const <IStringCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BypassProxyOnLocal<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BypassProxyOnLocal(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBypassProxyOnLocal<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBypassProxyOnLocal(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUserName(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPassword(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptForCredentials<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentwindow: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptForCredentials(&*(&parentwindow as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&title as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptForCredentialsFromHwnd<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PromptForCredentialsFromHwnd(&*(&parentwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&title as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDetect<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoDetect(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDetect<Impl: IWebProxyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoDetect(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWebProxy>,
            base.5,
            Address::<Impl, OFFSET>,
            SetAddress::<Impl, OFFSET>,
            BypassList::<Impl, OFFSET>,
            SetBypassList::<Impl, OFFSET>,
            BypassProxyOnLocal::<Impl, OFFSET>,
            SetBypassProxyOnLocal::<Impl, OFFSET>,
            ReadOnly::<Impl, OFFSET>,
            UserName::<Impl, OFFSET>,
            SetUserName::<Impl, OFFSET>,
            SetPassword::<Impl, OFFSET>,
            PromptForCredentials::<Impl, OFFSET>,
            PromptForCredentialsFromHwnd::<Impl, OFFSET>,
            AutoDetect::<Impl, OFFSET>,
            SetAutoDetect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateImpl: Sized + IUpdateImpl + IDispatchImpl {
    fn DriverClass();
    fn DriverHardwareID();
    fn DriverManufacturer();
    fn DriverModel();
    fn DriverProvider();
    fn DriverVerDate();
    fn DeviceProblemNumber();
    fn DeviceStatus();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdate {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdate";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateVtbl {
    pub const fn new<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdateVtbl {
        unsafe extern "system" fn DriverClass<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverClass(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverHardwareID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverManufacturer(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverModel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverProvider(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverVerDate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceProblemNumber(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Impl: IWindowsDriverUpdateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceStatus(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdate>, base.5, DriverClass::<Impl, OFFSET>, DriverHardwareID::<Impl, OFFSET>, DriverManufacturer::<Impl, OFFSET>, DriverModel::<Impl, OFFSET>, DriverProvider::<Impl, OFFSET>, DriverVerDate::<Impl, OFFSET>, DeviceProblemNumber::<Impl, OFFSET>, DeviceStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate2Impl: Sized + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn RebootRequired();
    fn IsPresent();
    fn CveIDs();
    fn CopyToCache();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdate2 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdate2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate2Vtbl {
    pub const fn new<Impl: IWindowsDriverUpdate2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdate2Vtbl {
        unsafe extern "system" fn RebootRequired<Impl: IWindowsDriverUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RebootRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresent<Impl: IWindowsDriverUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPresent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CveIDs<Impl: IWindowsDriverUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CveIDs(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyToCache<Impl: IWindowsDriverUpdate2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiles: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CopyToCache(&*(&pfiles as *const <IStringCollection as ::windows::core::Abi>::Abi as *const <IStringCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdate2>, base.5, RebootRequired::<Impl, OFFSET>, IsPresent::<Impl, OFFSET>, CveIDs::<Impl, OFFSET>, CopyToCache::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate3Impl: Sized + IWindowsDriverUpdate2Impl + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn BrowseOnly();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdate3 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdate3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate3Vtbl {
    pub const fn new<Impl: IWindowsDriverUpdate3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdate3Vtbl {
        unsafe extern "system" fn BrowseOnly<Impl: IWindowsDriverUpdate3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BrowseOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdate3>, base.5, BrowseOnly::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate4Impl: Sized + IWindowsDriverUpdate3Impl + IWindowsDriverUpdate2Impl + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn WindowsDriverUpdateEntries();
    fn PerUser();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdate4 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdate4";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate4Vtbl {
    pub const fn new<Impl: IWindowsDriverUpdate4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdate4Vtbl {
        unsafe extern "system" fn WindowsDriverUpdateEntries<Impl: IWindowsDriverUpdate4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowsDriverUpdateEntries(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PerUser<Impl: IWindowsDriverUpdate4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PerUser(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdate4>, base.5, WindowsDriverUpdateEntries::<Impl, OFFSET>, PerUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdate5Impl: Sized + IWindowsDriverUpdate4Impl + IWindowsDriverUpdate3Impl + IWindowsDriverUpdate2Impl + IWindowsDriverUpdateImpl + IUpdateImpl + IDispatchImpl {
    fn AutoSelection();
    fn AutoDownload();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdate5 {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdate5";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate5Vtbl {
    pub const fn new<Impl: IWindowsDriverUpdate5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdate5Vtbl {
        unsafe extern "system" fn AutoSelection<Impl: IWindowsDriverUpdate5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoSelection(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDownload<Impl: IWindowsDriverUpdate5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoDownload(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdate5>, base.5, AutoSelection::<Impl, OFFSET>, AutoDownload::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateEntryImpl: Sized + IDispatchImpl {
    fn DriverClass();
    fn DriverHardwareID();
    fn DriverManufacturer();
    fn DriverModel();
    fn DriverProvider();
    fn DriverVerDate();
    fn DeviceProblemNumber();
    fn DeviceStatus();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdateEntry {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdateEntry";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntryVtbl {
    pub const fn new<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdateEntryVtbl {
        unsafe extern "system" fn DriverClass<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverClass(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverHardwareID<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverHardwareID(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverManufacturer<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverManufacturer(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverModel<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverModel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverProvider<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverProvider(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DriverVerDate<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DriverVerDate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceProblemNumber<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceProblemNumber(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceStatus<Impl: IWindowsDriverUpdateEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceStatus(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdateEntry>, base.5, DriverClass::<Impl, OFFSET>, DriverHardwareID::<Impl, OFFSET>, DriverManufacturer::<Impl, OFFSET>, DriverModel::<Impl, OFFSET>, DriverProvider::<Impl, OFFSET>, DriverVerDate::<Impl, OFFSET>, DeviceProblemNumber::<Impl, OFFSET>, DeviceStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsDriverUpdateEntryCollectionImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsDriverUpdateEntryCollection {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsDriverUpdateEntryCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntryCollectionVtbl {
    pub const fn new<Impl: IWindowsDriverUpdateEntryCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsDriverUpdateEntryCollectionVtbl {
        unsafe extern "system" fn Item<Impl: IWindowsDriverUpdateEntryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IWindowsDriverUpdateEntryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWindowsDriverUpdateEntryCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsDriverUpdateEntryCollection>, base.5, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsUpdateAgentInfoImpl: Sized + IDispatchImpl {
    fn GetInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWindowsUpdateAgentInfo {
    const NAME: &'static str = "Windows.Win32.System.UpdateAgent.IWindowsUpdateAgentInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IWindowsUpdateAgentInfoVtbl {
    pub const fn new<Impl: IWindowsUpdateAgentInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowsUpdateAgentInfoVtbl {
        unsafe extern "system" fn GetInfo<Impl: IWindowsUpdateAgentInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinfoidentifier: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInfo(&*(&varinfoidentifier as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowsUpdateAgentInfo>, base.5, GetInfo::<Impl, OFFSET>)
    }
}

#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDiskQuotaControl_Impl: Sized + super::super::System::Com::IConnectionPointContainer_Impl {
    fn Initialize(&self, pszpath: &::windows::core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetQuotaState(&self, dwstate: u32) -> ::windows::core::Result<()>;
    fn GetQuotaState(&self, pdwstate: *mut u32) -> ::windows::core::Result<()>;
    fn SetQuotaLogFlags(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetQuotaLogFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn SetDefaultQuotaThreshold(&self, llthreshold: i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaThreshold(&self, pllthreshold: *mut i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaThresholdText(&self, psztext: &::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn SetDefaultQuotaLimit(&self, lllimit: i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaLimit(&self, plllimit: *mut i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaLimitText(&self, psztext: &::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn AddUserSid(&self, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>;
    fn AddUserName(&self, pszlogonname: &::windows::core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>;
    fn DeleteUser(&self, puser: &::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn FindUserSid(&self, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>;
    fn FindUserName(&self, pszlogonname: &::windows::core::PCWSTR) -> ::windows::core::Result<IDiskQuotaUser>;
    fn CreateEnumUsers(&self, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::core::option::Option<IEnumDiskQuotaUsers>) -> ::windows::core::Result<()>;
    fn CreateUserBatch(&self) -> ::windows::core::Result<IDiskQuotaUserBatch>;
    fn InvalidateSidNameCache(&self) -> ::windows::core::Result<()>;
    fn GiveUserNameResolutionPriority(&self, puser: &::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn ShutdownNameResolution(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IDiskQuotaControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDiskQuotaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>() -> IDiskQuotaControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows::core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&breadwrite)).into()
        }
        unsafe extern "system" fn SetQuotaState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuotaState(::core::mem::transmute_copy(&dwstate)).into()
        }
        unsafe extern "system" fn GetQuotaState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaState(::core::mem::transmute_copy(&pdwstate)).into()
        }
        unsafe extern "system" fn SetQuotaLogFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuotaLogFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetQuotaLogFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaLogFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetDefaultQuotaThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llthreshold: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultQuotaThreshold(::core::mem::transmute_copy(&llthreshold)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultQuotaThreshold(::core::mem::transmute_copy(&pllthreshold)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaThresholdText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultQuotaThresholdText(::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn SetDefaultQuotaLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lllimit: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultQuotaLimit(::core::mem::transmute_copy(&lllimit)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultQuotaLimit(::core::mem::transmute_copy(&plllimit)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaLimitText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultQuotaLimitText(::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn AddUserSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddUserSid(::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlogonname: ::windows::core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddUserName(::core::mem::transmute(&pszlogonname), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteUser(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn FindUserSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindUserSid(::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlogonname: ::windows::core::PCWSTR, ppuser: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindUserName(::core::mem::transmute(&pszlogonname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnumUsers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateEnumUsers(::core::mem::transmute_copy(&rgpusersids), ::core::mem::transmute_copy(&cpsids), ::core::mem::transmute_copy(&fnameresolution), ::core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn CreateUserBatch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUserBatch() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbatch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateSidNameCache<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateSidNameCache().into()
        }
        unsafe extern "system" fn GiveUserNameResolutionPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GiveUserNameResolutionPriority(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn ShutdownNameResolution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownNameResolution().into()
        }
        Self {
            base__: super::super::System::Com::IConnectionPointContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetQuotaState: SetQuotaState::<Identity, Impl, OFFSET>,
            GetQuotaState: GetQuotaState::<Identity, Impl, OFFSET>,
            SetQuotaLogFlags: SetQuotaLogFlags::<Identity, Impl, OFFSET>,
            GetQuotaLogFlags: GetQuotaLogFlags::<Identity, Impl, OFFSET>,
            SetDefaultQuotaThreshold: SetDefaultQuotaThreshold::<Identity, Impl, OFFSET>,
            GetDefaultQuotaThreshold: GetDefaultQuotaThreshold::<Identity, Impl, OFFSET>,
            GetDefaultQuotaThresholdText: GetDefaultQuotaThresholdText::<Identity, Impl, OFFSET>,
            SetDefaultQuotaLimit: SetDefaultQuotaLimit::<Identity, Impl, OFFSET>,
            GetDefaultQuotaLimit: GetDefaultQuotaLimit::<Identity, Impl, OFFSET>,
            GetDefaultQuotaLimitText: GetDefaultQuotaLimitText::<Identity, Impl, OFFSET>,
            AddUserSid: AddUserSid::<Identity, Impl, OFFSET>,
            AddUserName: AddUserName::<Identity, Impl, OFFSET>,
            DeleteUser: DeleteUser::<Identity, Impl, OFFSET>,
            FindUserSid: FindUserSid::<Identity, Impl, OFFSET>,
            FindUserName: FindUserName::<Identity, Impl, OFFSET>,
            CreateEnumUsers: CreateEnumUsers::<Identity, Impl, OFFSET>,
            CreateUserBatch: CreateUserBatch::<Identity, Impl, OFFSET>,
            InvalidateSidNameCache: InvalidateSidNameCache::<Identity, Impl, OFFSET>,
            GiveUserNameResolutionPriority: GiveUserNameResolutionPriority::<Identity, Impl, OFFSET>,
            ShutdownNameResolution: ShutdownNameResolution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IConnectionPointContainer as ::windows::core::Interface>::IID
    }
}
pub trait IDiskQuotaEvents_Impl: Sized {
    fn OnUserNameChanged(&self, puser: &::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiskQuotaEvents {}
impl IDiskQuotaEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaEvents_Impl, const OFFSET: isize>() -> IDiskQuotaEvents_Vtbl {
        unsafe extern "system" fn OnUserNameChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUserNameChanged(::core::mem::transmute(&puser)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnUserNameChanged: OnUserNameChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDiskQuotaUser_Impl: Sized {
    fn GetID(&self, pulid: *mut u32) -> ::windows::core::Result<()>;
    fn GetName(&self, pszaccountcontainer: &::windows::core::PCWSTR, cchaccountcontainer: u32, pszlogonname: &::windows::core::PCWSTR, cchlogonname: u32, pszdisplayname: &::windows::core::PCWSTR, cchdisplayname: u32) -> ::windows::core::Result<()>;
    fn GetSidLength(&self, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSid(&self, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::Result<()>;
    fn GetQuotaThreshold(&self, pllthreshold: *mut i64) -> ::windows::core::Result<()>;
    fn GetQuotaThresholdText(&self, psztext: &::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetQuotaLimit(&self, plllimit: *mut i64) -> ::windows::core::Result<()>;
    fn GetQuotaLimitText(&self, psztext: &::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetQuotaUsed(&self, pllused: *mut i64) -> ::windows::core::Result<()>;
    fn GetQuotaUsedText(&self, psztext: &::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetQuotaInformation(&self, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::Result<()>;
    fn SetQuotaThreshold(&self, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetQuotaLimit(&self, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Invalidate(&self) -> ::windows::core::Result<()>;
    fn GetAccountStatus(&self, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDiskQuotaUser {}
#[cfg(feature = "Win32_Foundation")]
impl IDiskQuotaUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>() -> IDiskQuotaUser_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetID(::core::mem::transmute_copy(&pulid)).into()
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaccountcontainer: ::windows::core::PCWSTR, cchaccountcontainer: u32, pszlogonname: ::windows::core::PCWSTR, cchlogonname: u32, pszdisplayname: ::windows::core::PCWSTR, cchdisplayname: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute(&pszaccountcontainer), ::core::mem::transmute_copy(&cchaccountcontainer), ::core::mem::transmute(&pszlogonname), ::core::mem::transmute_copy(&cchlogonname), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&cchdisplayname)).into()
        }
        unsafe extern "system" fn GetSidLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSidLength(::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSid(::core::mem::transmute_copy(&pbsidbuffer), ::core::mem::transmute_copy(&cbsidbuffer)).into()
        }
        unsafe extern "system" fn GetQuotaThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaThreshold(::core::mem::transmute_copy(&pllthreshold)).into()
        }
        unsafe extern "system" fn GetQuotaThresholdText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaThresholdText(::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaLimit(::core::mem::transmute_copy(&plllimit)).into()
        }
        unsafe extern "system" fn GetQuotaLimitText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaLimitText(::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaUsed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllused: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaUsed(::core::mem::transmute_copy(&pllused)).into()
        }
        unsafe extern "system" fn GetQuotaUsedText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: ::windows::core::PCWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaUsedText(::core::mem::transmute(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuotaInformation(::core::mem::transmute_copy(&pbquotainfo), ::core::mem::transmute_copy(&cbquotainfo)).into()
        }
        unsafe extern "system" fn SetQuotaThreshold<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuotaThreshold(::core::mem::transmute_copy(&llthreshold), ::core::mem::transmute_copy(&fwritethrough)).into()
        }
        unsafe extern "system" fn SetQuotaLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuotaLimit(::core::mem::transmute_copy(&lllimit), ::core::mem::transmute_copy(&fwritethrough)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invalidate().into()
        }
        unsafe extern "system" fn GetAccountStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAccountStatus(::core::mem::transmute_copy(&pdwstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSidLength: GetSidLength::<Identity, Impl, OFFSET>,
            GetSid: GetSid::<Identity, Impl, OFFSET>,
            GetQuotaThreshold: GetQuotaThreshold::<Identity, Impl, OFFSET>,
            GetQuotaThresholdText: GetQuotaThresholdText::<Identity, Impl, OFFSET>,
            GetQuotaLimit: GetQuotaLimit::<Identity, Impl, OFFSET>,
            GetQuotaLimitText: GetQuotaLimitText::<Identity, Impl, OFFSET>,
            GetQuotaUsed: GetQuotaUsed::<Identity, Impl, OFFSET>,
            GetQuotaUsedText: GetQuotaUsedText::<Identity, Impl, OFFSET>,
            GetQuotaInformation: GetQuotaInformation::<Identity, Impl, OFFSET>,
            SetQuotaThreshold: SetQuotaThreshold::<Identity, Impl, OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetAccountStatus: GetAccountStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaUser as ::windows::core::Interface>::IID
    }
}
pub trait IDiskQuotaUserBatch_Impl: Sized {
    fn Add(&self, puser: &::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn Remove(&self, puser: &::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
    fn FlushToDisk(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDiskQuotaUserBatch {}
impl IDiskQuotaUserBatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>() -> IDiskQuotaUserBatch_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn FlushToDisk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushToDisk().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            FlushToDisk: FlushToDisk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaUserBatch as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDiskQuotaUsers_Impl: Sized {
    fn Next(&self, cusers: u32, rgusers: *mut ::core::option::Option<IDiskQuotaUser>, pcusersfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cusers: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDiskQuotaUsers>;
}
impl ::windows::core::RuntimeName for IEnumDiskQuotaUsers {}
impl IEnumDiskQuotaUsers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>() -> IEnumDiskQuotaUsers_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusers: u32, rgusers: *mut *mut ::core::ffi::c_void, pcusersfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cusers), ::core::mem::transmute_copy(&rgusers), ::core::mem::transmute_copy(&pcusersfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusers: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cusers)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDiskQuotaUsers as ::windows::core::Interface>::IID
    }
}

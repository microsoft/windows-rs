#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDiskQuotaControlImpl: Sized + IConnectionPointContainerImpl {
    fn Initialize(&mut self, pszpath: super::super::Foundation::PWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetQuotaState(&mut self, dwstate: u32) -> ::windows::core::Result<()>;
    fn GetQuotaState(&mut self, pdwstate: *mut u32) -> ::windows::core::Result<()>;
    fn SetQuotaLogFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetQuotaLogFlags(&mut self, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn SetDefaultQuotaThreshold(&mut self, llthreshold: i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaThreshold(&mut self, pllthreshold: *mut i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaThresholdText(&mut self, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn SetDefaultQuotaLimit(&mut self, lllimit: i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaLimit(&mut self, plllimit: *mut i64) -> ::windows::core::Result<()>;
    fn GetDefaultQuotaLimitText(&mut self, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn AddUserSid(&mut self, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>;
    fn AddUserName(&mut self, pszlogonname: super::super::Foundation::PWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>;
    fn DeleteUser(&mut self, puser: ::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn FindUserSid(&mut self, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> ::windows::core::Result<IDiskQuotaUser>;
    fn FindUserName(&mut self, pszlogonname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IDiskQuotaUser>;
    fn CreateEnumUsers(&mut self, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::core::option::Option<IEnumDiskQuotaUsers>) -> ::windows::core::Result<()>;
    fn CreateUserBatch(&mut self) -> ::windows::core::Result<IDiskQuotaUserBatch>;
    fn InvalidateSidNameCache(&mut self) -> ::windows::core::Result<()>;
    fn GiveUserNameResolutionPriority(&mut self, puser: ::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn ShutdownNameResolution(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDiskQuotaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaControlVtbl {
        unsafe extern "system" fn Initialize<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&breadwrite)).into()
        }
        unsafe extern "system" fn SetQuotaState<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuotaState(::core::mem::transmute_copy(&dwstate)).into()
        }
        unsafe extern "system" fn GetQuotaState<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaState(::core::mem::transmute_copy(&pdwstate)).into()
        }
        unsafe extern "system" fn SetQuotaLogFlags<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuotaLogFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetQuotaLogFlags<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaLogFlags(::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetDefaultQuotaThreshold<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llthreshold: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultQuotaThreshold(::core::mem::transmute_copy(&llthreshold)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaThreshold<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultQuotaThreshold(::core::mem::transmute_copy(&pllthreshold)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaThresholdText<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultQuotaThresholdText(::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn SetDefaultQuotaLimit<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lllimit: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultQuotaLimit(::core::mem::transmute_copy(&lllimit)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaLimit<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultQuotaLimit(::core::mem::transmute_copy(&plllimit)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaLimitText<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDefaultQuotaLimitText(::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn AddUserSid<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddUserSid(::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddUserName<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlogonname: super::super::Foundation::PWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddUserName(::core::mem::transmute_copy(&pszlogonname), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUser<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteUser(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn FindUserSid<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUserSid(::core::mem::transmute_copy(&pusersid), ::core::mem::transmute_copy(&fnameresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUserName<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlogonname: super::super::Foundation::PWSTR, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUserName(::core::mem::transmute_copy(&pszlogonname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnumUsers<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateEnumUsers(::core::mem::transmute_copy(&rgpusersids), ::core::mem::transmute_copy(&cpsids), ::core::mem::transmute_copy(&fnameresolution), ::core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn CreateUserBatch<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUserBatch() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateSidNameCache<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateSidNameCache().into()
        }
        unsafe extern "system" fn GiveUserNameResolutionPriority<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GiveUserNameResolutionPriority(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn ShutdownNameResolution<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownNameResolution().into()
        }
        Self {
            base: IConnectionPointContainerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SetQuotaState: SetQuotaState::<Impl, IMPL_OFFSET>,
            GetQuotaState: GetQuotaState::<Impl, IMPL_OFFSET>,
            SetQuotaLogFlags: SetQuotaLogFlags::<Impl, IMPL_OFFSET>,
            GetQuotaLogFlags: GetQuotaLogFlags::<Impl, IMPL_OFFSET>,
            SetDefaultQuotaThreshold: SetDefaultQuotaThreshold::<Impl, IMPL_OFFSET>,
            GetDefaultQuotaThreshold: GetDefaultQuotaThreshold::<Impl, IMPL_OFFSET>,
            GetDefaultQuotaThresholdText: GetDefaultQuotaThresholdText::<Impl, IMPL_OFFSET>,
            SetDefaultQuotaLimit: SetDefaultQuotaLimit::<Impl, IMPL_OFFSET>,
            GetDefaultQuotaLimit: GetDefaultQuotaLimit::<Impl, IMPL_OFFSET>,
            GetDefaultQuotaLimitText: GetDefaultQuotaLimitText::<Impl, IMPL_OFFSET>,
            AddUserSid: AddUserSid::<Impl, IMPL_OFFSET>,
            AddUserName: AddUserName::<Impl, IMPL_OFFSET>,
            DeleteUser: DeleteUser::<Impl, IMPL_OFFSET>,
            FindUserSid: FindUserSid::<Impl, IMPL_OFFSET>,
            FindUserName: FindUserName::<Impl, IMPL_OFFSET>,
            CreateEnumUsers: CreateEnumUsers::<Impl, IMPL_OFFSET>,
            CreateUserBatch: CreateUserBatch::<Impl, IMPL_OFFSET>,
            InvalidateSidNameCache: InvalidateSidNameCache::<Impl, IMPL_OFFSET>,
            GiveUserNameResolutionPriority: GiveUserNameResolutionPriority::<Impl, IMPL_OFFSET>,
            ShutdownNameResolution: ShutdownNameResolution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaControl as ::windows::core::Interface>::IID
    }
}
pub trait IDiskQuotaEventsImpl: Sized {
    fn OnUserNameChanged(&mut self, puser: ::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
}
impl IDiskQuotaEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaEventsVtbl {
        unsafe extern "system" fn OnUserNameChanged<Impl: IDiskQuotaEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUserNameChanged(::core::mem::transmute(&puser)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUserNameChanged: OnUserNameChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDiskQuotaUserImpl: Sized {
    fn GetID(&mut self, pulid: *mut u32) -> ::windows::core::Result<()>;
    fn GetName(&mut self, pszaccountcontainer: super::super::Foundation::PWSTR, cchaccountcontainer: u32, pszlogonname: super::super::Foundation::PWSTR, cchlogonname: u32, pszdisplayname: super::super::Foundation::PWSTR, cchdisplayname: u32) -> ::windows::core::Result<()>;
    fn GetSidLength(&mut self, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSid(&mut self, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::Result<()>;
    fn GetQuotaThreshold(&mut self, pllthreshold: *mut i64) -> ::windows::core::Result<()>;
    fn GetQuotaThresholdText(&mut self, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetQuotaLimit(&mut self, plllimit: *mut i64) -> ::windows::core::Result<()>;
    fn GetQuotaLimitText(&mut self, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetQuotaUsed(&mut self, pllused: *mut i64) -> ::windows::core::Result<()>;
    fn GetQuotaUsedText(&mut self, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::Result<()>;
    fn GetQuotaInformation(&mut self, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::Result<()>;
    fn SetQuotaThreshold(&mut self, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetQuotaLimit(&mut self, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Invalidate(&mut self) -> ::windows::core::Result<()>;
    fn GetAccountStatus(&mut self, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDiskQuotaUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaUserVtbl {
        unsafe extern "system" fn GetID<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pulid)).into()
        }
        unsafe extern "system" fn GetName<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaccountcontainer: super::super::Foundation::PWSTR, cchaccountcontainer: u32, pszlogonname: super::super::Foundation::PWSTR, cchlogonname: u32, pszdisplayname: super::super::Foundation::PWSTR, cchdisplayname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pszaccountcontainer), ::core::mem::transmute_copy(&cchaccountcontainer), ::core::mem::transmute_copy(&pszlogonname), ::core::mem::transmute_copy(&cchlogonname), ::core::mem::transmute_copy(&pszdisplayname), ::core::mem::transmute_copy(&cchdisplayname)).into()
        }
        unsafe extern "system" fn GetSidLength<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSidLength(::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSid<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSid(::core::mem::transmute_copy(&pbsidbuffer), ::core::mem::transmute_copy(&cbsidbuffer)).into()
        }
        unsafe extern "system" fn GetQuotaThreshold<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaThreshold(::core::mem::transmute_copy(&pllthreshold)).into()
        }
        unsafe extern "system" fn GetQuotaThresholdText<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaThresholdText(::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaLimit<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaLimit(::core::mem::transmute_copy(&plllimit)).into()
        }
        unsafe extern "system" fn GetQuotaLimitText<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaLimitText(::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaUsed<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllused: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaUsed(::core::mem::transmute_copy(&pllused)).into()
        }
        unsafe extern "system" fn GetQuotaUsedText<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaUsedText(::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaInformation<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetQuotaInformation(::core::mem::transmute_copy(&pbquotainfo), ::core::mem::transmute_copy(&cbquotainfo)).into()
        }
        unsafe extern "system" fn SetQuotaThreshold<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuotaThreshold(::core::mem::transmute_copy(&llthreshold), ::core::mem::transmute_copy(&fwritethrough)).into()
        }
        unsafe extern "system" fn SetQuotaLimit<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuotaLimit(::core::mem::transmute_copy(&lllimit), ::core::mem::transmute_copy(&fwritethrough)).into()
        }
        unsafe extern "system" fn Invalidate<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invalidate().into()
        }
        unsafe extern "system" fn GetAccountStatus<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAccountStatus(::core::mem::transmute_copy(&pdwstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetID: GetID::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetSidLength: GetSidLength::<Impl, IMPL_OFFSET>,
            GetSid: GetSid::<Impl, IMPL_OFFSET>,
            GetQuotaThreshold: GetQuotaThreshold::<Impl, IMPL_OFFSET>,
            GetQuotaThresholdText: GetQuotaThresholdText::<Impl, IMPL_OFFSET>,
            GetQuotaLimit: GetQuotaLimit::<Impl, IMPL_OFFSET>,
            GetQuotaLimitText: GetQuotaLimitText::<Impl, IMPL_OFFSET>,
            GetQuotaUsed: GetQuotaUsed::<Impl, IMPL_OFFSET>,
            GetQuotaUsedText: GetQuotaUsedText::<Impl, IMPL_OFFSET>,
            GetQuotaInformation: GetQuotaInformation::<Impl, IMPL_OFFSET>,
            SetQuotaThreshold: SetQuotaThreshold::<Impl, IMPL_OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Impl, IMPL_OFFSET>,
            Invalidate: Invalidate::<Impl, IMPL_OFFSET>,
            GetAccountStatus: GetAccountStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaUser as ::windows::core::Interface>::IID
    }
}
pub trait IDiskQuotaUserBatchImpl: Sized {
    fn Add(&mut self, puser: ::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, puser: ::core::option::Option<IDiskQuotaUser>) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
    fn FlushToDisk(&mut self) -> ::windows::core::Result<()>;
}
impl IDiskQuotaUserBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaUserBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaUserBatchVtbl {
        unsafe extern "system" fn Add<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn Remove<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute(&puser)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        unsafe extern "system" fn FlushToDisk<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushToDisk().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
            FlushToDisk: FlushToDisk::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaUserBatch as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDiskQuotaUsersImpl: Sized {
    fn Next(&mut self, cusers: u32, rgusers: *mut ::core::option::Option<IDiskQuotaUser>, pcusersfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cusers: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDiskQuotaUsers>;
}
impl IEnumDiskQuotaUsersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiskQuotaUsersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDiskQuotaUsersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusers: u32, rgusers: *mut ::windows::core::RawPtr, pcusersfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cusers), ::core::mem::transmute_copy(&rgusers), ::core::mem::transmute_copy(&pcusersfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cusers)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDiskQuotaUsers as ::windows::core::Interface>::IID
    }
}

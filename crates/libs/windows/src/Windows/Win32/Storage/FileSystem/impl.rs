#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDiskQuotaControlImpl: Sized + IConnectionPointContainerImpl {
    fn Initialize();
    fn SetQuotaState();
    fn GetQuotaState();
    fn SetQuotaLogFlags();
    fn GetQuotaLogFlags();
    fn SetDefaultQuotaThreshold();
    fn GetDefaultQuotaThreshold();
    fn GetDefaultQuotaThresholdText();
    fn SetDefaultQuotaLimit();
    fn GetDefaultQuotaLimit();
    fn GetDefaultQuotaLimitText();
    fn AddUserSid();
    fn AddUserName();
    fn DeleteUser();
    fn FindUserSid();
    fn FindUserName();
    fn CreateEnumUsers();
    fn CreateUserBatch();
    fn InvalidateSidNameCache();
    fn GiveUserNameResolutionPriority();
    fn ShutdownNameResolution();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDiskQuotaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaControlVtbl {
        unsafe extern "system" fn Initialize<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, breadwrite: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuotaState<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaState<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuotaLogFlags<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaLogFlags<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultQuotaThreshold<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llthreshold: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultQuotaThreshold<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultQuotaThresholdText<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultQuotaLimit<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lllimit: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultQuotaLimit<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultQuotaLimitText<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddUserSid<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddUserName<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlogonname: super::super::Foundation::PWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteUser<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindUserSid<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindUserName<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlogonname: super::super::Foundation::PWSTR, ppuser: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEnumUsers<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateUserBatch<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidateSidNameCache<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GiveUserNameResolutionPriority<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownNameResolution<Impl: IDiskQuotaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OnUserNameChanged();
}
impl IDiskQuotaEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaEventsVtbl {
        unsafe extern "system" fn OnUserNameChanged<Impl: IDiskQuotaEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUserNameChanged: OnUserNameChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiskQuotaEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDiskQuotaUserImpl: Sized {
    fn GetID();
    fn GetName();
    fn GetSidLength();
    fn GetSid();
    fn GetQuotaThreshold();
    fn GetQuotaThresholdText();
    fn GetQuotaLimit();
    fn GetQuotaLimitText();
    fn GetQuotaUsed();
    fn GetQuotaUsedText();
    fn GetQuotaInformation();
    fn SetQuotaThreshold();
    fn SetQuotaLimit();
    fn Invalidate();
    fn GetAccountStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IDiskQuotaUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaUserVtbl {
        unsafe extern "system" fn GetID<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaccountcontainer: super::super::Foundation::PWSTR, cchaccountcontainer: u32, pszlogonname: super::super::Foundation::PWSTR, cchlogonname: u32, pszdisplayname: super::super::Foundation::PWSTR, cchdisplayname: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSidLength<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSid<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaThreshold<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllthreshold: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaThresholdText<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaLimit<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plllimit: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaLimitText<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaUsed<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllused: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaUsedText<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuotaInformation<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbquotainfo: *mut ::core::ffi::c_void, cbquotainfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuotaThreshold<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuotaLimit<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invalidate<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccountStatus<Impl: IDiskQuotaUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn FlushToDisk();
}
impl IDiskQuotaUserBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiskQuotaUserBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiskQuotaUserBatchVtbl {
        unsafe extern "system" fn Add<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAll<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushToDisk<Impl: IDiskQuotaUserBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumDiskQuotaUsersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiskQuotaUsersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDiskQuotaUsersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusers: u32, rgusers: *mut ::windows::core::RawPtr, pcusersfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cusers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiskQuotaUsersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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

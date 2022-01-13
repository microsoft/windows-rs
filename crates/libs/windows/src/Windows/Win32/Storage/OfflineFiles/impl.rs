pub trait IEnumOfflineFilesItemsImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IOfflineFilesItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOfflineFilesItems>;
}
impl IEnumOfflineFilesItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOfflineFilesItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        iid == &<IEnumOfflineFilesItems as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOfflineFilesSettingsImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IOfflineFilesSetting>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOfflineFilesSettings>;
}
impl IEnumOfflineFilesSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOfflineFilesSettingsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        iid == &<IEnumOfflineFilesSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCacheImpl: Sized {
    fn Synchronize(&mut self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::core::option::Option<IOfflineFilesSyncConflictHandler>, piprogress: ::core::option::Option<IOfflineFilesSyncProgress>, psyncid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DeleteItems(&mut self, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::core::option::Option<IOfflineFilesSimpleProgress>) -> ::windows::core::Result<()>;
    fn DeleteItemsForUser(&mut self, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::core::option::Option<IOfflineFilesSimpleProgress>) -> ::windows::core::Result<()>;
    fn Pin(&mut self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn Unpin(&mut self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn GetEncryptionStatus(&mut self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Encrypt(&mut self, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn FindItem(&mut self, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32) -> ::windows::core::Result<IOfflineFilesItem>;
    fn FindItemEx(&mut self, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::core::option::Option<IOfflineFilesItemFilter>, pincludedirfilter: ::core::option::Option<IOfflineFilesItemFilter>, pexcludefilefilter: ::core::option::Option<IOfflineFilesItemFilter>, pexcludedirfilter: ::core::option::Option<IOfflineFilesItemFilter>, dwqueryflags: u32) -> ::windows::core::Result<IOfflineFilesItem>;
    fn RenameItem(&mut self, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDiskSpaceInformation(&mut self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::Result<()>;
    fn SetDiskSpaceLimits(&mut self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::Result<()>;
    fn ProcessAdminPinPolicy(&mut self, ppinprogress: ::core::option::Option<IOfflineFilesSyncProgress>, punpinprogress: ::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn GetSettingObject(&mut self, pszsettingname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IOfflineFilesSetting>;
    fn EnumSettingObjects(&mut self) -> ::windows::core::Result<IEnumOfflineFilesSettings>;
    fn IsPathCacheable(&mut self, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCacheImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesCacheVtbl {
        unsafe extern "system" fn Synchronize<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::windows::core::RawPtr, piprogress: ::windows::core::RawPtr, psyncid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Synchronize(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwsynccontrol), ::core::mem::transmute(&pisyncconflicthandler), ::core::mem::transmute(&piprogress), ::core::mem::transmute_copy(&psyncid)).into()
        }
        unsafe extern "system" fn DeleteItems<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItems(::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&basync), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn DeleteItemsForUser<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteItemsForUser(::core::mem::transmute_copy(&pszuser), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&basync), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn Pin<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pin(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&bdeep), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwpincontrolflags), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn Unpin<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unpin(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&bdeep), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwpincontrolflags), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn GetEncryptionStatus<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEncryptionStatus(::core::mem::transmute_copy(&pbencrypted), ::core::mem::transmute_copy(&pbpartial)).into()
        }
        unsafe extern "system" fn Encrypt<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encrypt(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bencrypt), ::core::mem::transmute_copy(&dwencryptioncontrolflags), ::core::mem::transmute_copy(&basync), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn FindItem<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItem(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemEx<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemEx(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute(&pincludefilefilter), ::core::mem::transmute(&pincludedirfilter), ::core::mem::transmute(&pexcludefilefilter), ::core::mem::transmute(&pexcludedirfilter), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameItem<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameItem(::core::mem::transmute_copy(&pszpathoriginal), ::core::mem::transmute_copy(&pszpathnew), ::core::mem::transmute_copy(&breplaceifexists)).into()
        }
        unsafe extern "system" fn GetLocation<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDiskSpaceInformation(::core::mem::transmute_copy(&pcbvolumetotal), ::core::mem::transmute_copy(&pcblimit), ::core::mem::transmute_copy(&pcbused), ::core::mem::transmute_copy(&pcbunpinnedlimit), ::core::mem::transmute_copy(&pcbunpinnedused)).into()
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiskSpaceLimits(::core::mem::transmute_copy(&cblimit), ::core::mem::transmute_copy(&cbunpinnedlimit)).into()
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinprogress: ::windows::core::RawPtr, punpinprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessAdminPinPolicy(::core::mem::transmute(&ppinprogress), ::core::mem::transmute(&punpinprogress)).into()
        }
        unsafe extern "system" fn GetSettingObject<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsettingname: super::super::Foundation::PWSTR, ppsetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettingObject(::core::mem::transmute_copy(&pszsettingname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSettingObjects<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumSettingObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathCacheable<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPathCacheable(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pbcacheable), ::core::mem::transmute_copy(&psharecachingmode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Synchronize: Synchronize::<Impl, IMPL_OFFSET>,
            DeleteItems: DeleteItems::<Impl, IMPL_OFFSET>,
            DeleteItemsForUser: DeleteItemsForUser::<Impl, IMPL_OFFSET>,
            Pin: Pin::<Impl, IMPL_OFFSET>,
            Unpin: Unpin::<Impl, IMPL_OFFSET>,
            GetEncryptionStatus: GetEncryptionStatus::<Impl, IMPL_OFFSET>,
            Encrypt: Encrypt::<Impl, IMPL_OFFSET>,
            FindItem: FindItem::<Impl, IMPL_OFFSET>,
            FindItemEx: FindItemEx::<Impl, IMPL_OFFSET>,
            RenameItem: RenameItem::<Impl, IMPL_OFFSET>,
            GetLocation: GetLocation::<Impl, IMPL_OFFSET>,
            GetDiskSpaceInformation: GetDiskSpaceInformation::<Impl, IMPL_OFFSET>,
            SetDiskSpaceLimits: SetDiskSpaceLimits::<Impl, IMPL_OFFSET>,
            ProcessAdminPinPolicy: ProcessAdminPinPolicy::<Impl, IMPL_OFFSET>,
            GetSettingObject: GetSettingObject::<Impl, IMPL_OFFSET>,
            EnumSettingObjects: EnumSettingObjects::<Impl, IMPL_OFFSET>,
            IsPathCacheable: IsPathCacheable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCache2Impl: Sized + IOfflineFilesCacheImpl {
    fn RenameItemEx(&mut self, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesCache2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesCache2Vtbl {
        unsafe extern "system" fn RenameItemEx<Impl: IOfflineFilesCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenameItemEx(::core::mem::transmute_copy(&pszpathoriginal), ::core::mem::transmute_copy(&pszpathnew), ::core::mem::transmute_copy(&breplaceifexists)).into()
        }
        Self { base: IOfflineFilesCacheVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RenameItemEx: RenameItemEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesCache2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesChangeInfoImpl: Sized {
    fn IsDirty(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDeletedOffline(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsCreatedOffline(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedData(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedAttributes(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesChangeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesChangeInfoVtbl {
        unsafe extern "system" fn IsDirty<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeletedOffline<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeletedOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdeletedoffline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCreatedOffline<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCreatedOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *pbcreatedoffline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedData<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocallyModifiedData() {
                ::core::result::Result::Ok(ok__) => {
                    *pblocallymodifieddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocallyModifiedAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pblocallymodifiedattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocallyModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pblocallymodifiedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsDirty: IsDirty::<Impl, IMPL_OFFSET>,
            IsDeletedOffline: IsDeletedOffline::<Impl, IMPL_OFFSET>,
            IsCreatedOffline: IsCreatedOffline::<Impl, IMPL_OFFSET>,
            IsLocallyModifiedData: IsLocallyModifiedData::<Impl, IMPL_OFFSET>,
            IsLocallyModifiedAttributes: IsLocallyModifiedAttributes::<Impl, IMPL_OFFSET>,
            IsLocallyModifiedTime: IsLocallyModifiedTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesChangeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesConnectionInfoImpl: Sized {
    fn GetConnectState(&mut self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::core::Result<()>;
    fn SetConnectState(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::core::Result<()>;
    fn TransitionOnline(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::Result<()>;
    fn TransitionOffline(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesConnectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesConnectionInfoVtbl {
        unsafe extern "system" fn GetConnectState<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConnectState(::core::mem::transmute_copy(&pconnectstate), ::core::mem::transmute_copy(&pofflinereason)).into()
        }
        unsafe extern "system" fn SetConnectState<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectState(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&connectstate)).into()
        }
        unsafe extern "system" fn TransitionOnline<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransitionOnline(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn TransitionOffline<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitionOffline(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&bforceopenfilesclosed)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbopenfilespreventedtransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConnectState: GetConnectState::<Impl, IMPL_OFFSET>,
            SetConnectState: SetConnectState::<Impl, IMPL_OFFSET>,
            TransitionOnline: TransitionOnline::<Impl, IMPL_OFFSET>,
            TransitionOffline: TransitionOffline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesConnectionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesDirectoryItemImpl: Sized + IOfflineFilesItemImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesDirectoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirectoryItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesDirectoryItemVtbl {
        Self { base: IOfflineFilesItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesDirectoryItem as ::windows::core::Interface>::IID
    }
}
pub trait IOfflineFilesDirtyInfoImpl: Sized {
    fn LocalDirtyByteCount(&mut self) -> ::windows::core::Result<i64>;
    fn RemoteDirtyByteCount(&mut self) -> ::windows::core::Result<i64>;
}
impl IOfflineFilesDirtyInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirtyInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesDirtyInfoVtbl {
        unsafe extern "system" fn LocalDirtyByteCount<Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalDirtyByteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirtybytecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteDirtyByteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirtybytecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LocalDirtyByteCount: LocalDirtyByteCount::<Impl, IMPL_OFFSET>,
            RemoteDirtyByteCount: RemoteDirtyByteCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesDirtyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesErrorInfoImpl: Sized {
    fn GetRawData(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::BYTE_BLOB>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOfflineFilesErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesErrorInfoVtbl {
        unsafe extern "system" fn GetRawData<Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRawData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppblob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetRawData: GetRawData::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEventsImpl: Sized {
    fn CacheMoved(&mut self, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CacheIsFull(&mut self) -> ::windows::core::Result<()>;
    fn CacheIsCorrupted(&mut self) -> ::windows::core::Result<()>;
    fn Enabled(&mut self, benabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EncryptionChanged(&mut self, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SyncBegin(&mut self, rsyncid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SyncFileResult(&mut self, rsyncid: *const ::windows::core::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn SyncConflictRecAdded(&mut self, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>;
    fn SyncConflictRecUpdated(&mut self, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>;
    fn SyncConflictRecRemoved(&mut self, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>;
    fn SyncEnd(&mut self, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn NetTransportArrived(&mut self) -> ::windows::core::Result<()>;
    fn NoNetTransports(&mut self) -> ::windows::core::Result<()>;
    fn ItemDisconnected(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemReconnected(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemAvailableOffline(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemNotAvailableOffline(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemPinned(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemNotPinned(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemModified(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ItemAddedToCache(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemDeletedFromCache(&mut self, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn ItemRenamed(&mut self, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>;
    fn DataLost(&mut self) -> ::windows::core::Result<()>;
    fn Ping(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEventsVtbl {
        unsafe extern "system" fn CacheMoved<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CacheMoved(::core::mem::transmute_copy(&pszoldpath), ::core::mem::transmute_copy(&psznewpath)).into()
        }
        unsafe extern "system" fn CacheIsFull<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CacheIsFull().into()
        }
        unsafe extern "system" fn CacheIsCorrupted<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CacheIsCorrupted().into()
        }
        unsafe extern "system" fn Enabled<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn EncryptionChanged<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncryptionChanged(::core::mem::transmute_copy(&bwasencrypted), ::core::mem::transmute_copy(&bwaspartial), ::core::mem::transmute_copy(&bisencrypted), ::core::mem::transmute_copy(&bispartial)).into()
        }
        unsafe extern "system" fn SyncBegin<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncBegin(::core::mem::transmute_copy(&rsyncid)).into()
        }
        unsafe extern "system" fn SyncFileResult<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncFileResult(::core::mem::transmute_copy(&rsyncid), ::core::mem::transmute_copy(&pszfile), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn SyncConflictRecAdded<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncConflictRecAdded(::core::mem::transmute_copy(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncConflictRecUpdated(::core::mem::transmute_copy(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncConflictRecRemoved(::core::mem::transmute_copy(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncEnd<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncEnd(::core::mem::transmute_copy(&rsyncid), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn NetTransportArrived<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NetTransportArrived().into()
        }
        unsafe extern "system" fn NoNetTransports<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NoNetTransports().into()
        }
        unsafe extern "system" fn ItemDisconnected<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemDisconnected(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemReconnected<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemReconnected(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemAvailableOffline<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemAvailableOffline(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemNotAvailableOffline(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemPinned<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemPinned(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemNotPinned<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemNotPinned(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemModified<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemModified(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype), ::core::mem::transmute_copy(&bmodifieddata), ::core::mem::transmute_copy(&bmodifiedattributes)).into()
        }
        unsafe extern "system" fn ItemAddedToCache<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemAddedToCache(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemDeletedFromCache<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemDeletedFromCache(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemRenamed<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemRenamed(::core::mem::transmute_copy(&pszoldpath), ::core::mem::transmute_copy(&psznewpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn DataLost<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DataLost().into()
        }
        unsafe extern "system" fn Ping<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Ping().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CacheMoved: CacheMoved::<Impl, IMPL_OFFSET>,
            CacheIsFull: CacheIsFull::<Impl, IMPL_OFFSET>,
            CacheIsCorrupted: CacheIsCorrupted::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            EncryptionChanged: EncryptionChanged::<Impl, IMPL_OFFSET>,
            SyncBegin: SyncBegin::<Impl, IMPL_OFFSET>,
            SyncFileResult: SyncFileResult::<Impl, IMPL_OFFSET>,
            SyncConflictRecAdded: SyncConflictRecAdded::<Impl, IMPL_OFFSET>,
            SyncConflictRecUpdated: SyncConflictRecUpdated::<Impl, IMPL_OFFSET>,
            SyncConflictRecRemoved: SyncConflictRecRemoved::<Impl, IMPL_OFFSET>,
            SyncEnd: SyncEnd::<Impl, IMPL_OFFSET>,
            NetTransportArrived: NetTransportArrived::<Impl, IMPL_OFFSET>,
            NoNetTransports: NoNetTransports::<Impl, IMPL_OFFSET>,
            ItemDisconnected: ItemDisconnected::<Impl, IMPL_OFFSET>,
            ItemReconnected: ItemReconnected::<Impl, IMPL_OFFSET>,
            ItemAvailableOffline: ItemAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemNotAvailableOffline: ItemNotAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemPinned: ItemPinned::<Impl, IMPL_OFFSET>,
            ItemNotPinned: ItemNotPinned::<Impl, IMPL_OFFSET>,
            ItemModified: ItemModified::<Impl, IMPL_OFFSET>,
            ItemAddedToCache: ItemAddedToCache::<Impl, IMPL_OFFSET>,
            ItemDeletedFromCache: ItemDeletedFromCache::<Impl, IMPL_OFFSET>,
            ItemRenamed: ItemRenamed::<Impl, IMPL_OFFSET>,
            DataLost: DataLost::<Impl, IMPL_OFFSET>,
            Ping: Ping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents2Impl: Sized + IOfflineFilesEventsImpl {
    fn ItemReconnectBegin(&mut self) -> ::windows::core::Result<()>;
    fn ItemReconnectEnd(&mut self) -> ::windows::core::Result<()>;
    fn CacheEvictBegin(&mut self) -> ::windows::core::Result<()>;
    fn CacheEvictEnd(&mut self) -> ::windows::core::Result<()>;
    fn BackgroundSyncBegin(&mut self, dwsynccontrolflags: u32) -> ::windows::core::Result<()>;
    fn BackgroundSyncEnd(&mut self, dwsynccontrolflags: u32) -> ::windows::core::Result<()>;
    fn PolicyChangeDetected(&mut self) -> ::windows::core::Result<()>;
    fn PreferenceChangeDetected(&mut self) -> ::windows::core::Result<()>;
    fn SettingsChangesApplied(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEvents2Vtbl {
        unsafe extern "system" fn ItemReconnectBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemReconnectBegin().into()
        }
        unsafe extern "system" fn ItemReconnectEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ItemReconnectEnd().into()
        }
        unsafe extern "system" fn CacheEvictBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CacheEvictBegin().into()
        }
        unsafe extern "system" fn CacheEvictEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CacheEvictEnd().into()
        }
        unsafe extern "system" fn BackgroundSyncBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackgroundSyncBegin(::core::mem::transmute_copy(&dwsynccontrolflags)).into()
        }
        unsafe extern "system" fn BackgroundSyncEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackgroundSyncEnd(::core::mem::transmute_copy(&dwsynccontrolflags)).into()
        }
        unsafe extern "system" fn PolicyChangeDetected<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PolicyChangeDetected().into()
        }
        unsafe extern "system" fn PreferenceChangeDetected<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreferenceChangeDetected().into()
        }
        unsafe extern "system" fn SettingsChangesApplied<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SettingsChangesApplied().into()
        }
        Self {
            base: IOfflineFilesEventsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemReconnectBegin: ItemReconnectBegin::<Impl, IMPL_OFFSET>,
            ItemReconnectEnd: ItemReconnectEnd::<Impl, IMPL_OFFSET>,
            CacheEvictBegin: CacheEvictBegin::<Impl, IMPL_OFFSET>,
            CacheEvictEnd: CacheEvictEnd::<Impl, IMPL_OFFSET>,
            BackgroundSyncBegin: BackgroundSyncBegin::<Impl, IMPL_OFFSET>,
            BackgroundSyncEnd: BackgroundSyncEnd::<Impl, IMPL_OFFSET>,
            PolicyChangeDetected: PolicyChangeDetected::<Impl, IMPL_OFFSET>,
            PreferenceChangeDetected: PreferenceChangeDetected::<Impl, IMPL_OFFSET>,
            SettingsChangesApplied: SettingsChangesApplied::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents3Impl: Sized + IOfflineFilesEventsImpl + IOfflineFilesEvents2Impl {
    fn TransparentCacheItemNotify(&mut self, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn PrefetchFileBegin(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn PrefetchFileEnd(&mut self, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEvents3Vtbl {
        unsafe extern "system" fn TransparentCacheItemNotify<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransparentCacheItemNotify(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&eventtype), ::core::mem::transmute_copy(&itemtype), ::core::mem::transmute_copy(&bmodifieddata), ::core::mem::transmute_copy(&bmodifiedattributes), ::core::mem::transmute_copy(&pzsoldpath)).into()
        }
        unsafe extern "system" fn PrefetchFileBegin<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrefetchFileBegin(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn PrefetchFileEnd<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrefetchFileEnd(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base: IOfflineFilesEvents2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TransparentCacheItemNotify: TransparentCacheItemNotify::<Impl, IMPL_OFFSET>,
            PrefetchFileBegin: PrefetchFileBegin::<Impl, IMPL_OFFSET>,
            PrefetchFileEnd: PrefetchFileEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents4Impl: Sized + IOfflineFilesEventsImpl + IOfflineFilesEvents2Impl + IOfflineFilesEvents3Impl {
    fn PrefetchCloseHandleBegin(&mut self) -> ::windows::core::Result<()>;
    fn PrefetchCloseHandleEnd(&mut self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEvents4Vtbl {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrefetchCloseHandleBegin().into()
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrefetchCloseHandleEnd(::core::mem::transmute_copy(&dwclosedhandlecount), ::core::mem::transmute_copy(&dwopenhandlecount), ::core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base: IOfflineFilesEvents3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PrefetchCloseHandleBegin: PrefetchCloseHandleBegin::<Impl, IMPL_OFFSET>,
            PrefetchCloseHandleEnd: PrefetchCloseHandleEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEventsFilterImpl: Sized {
    fn GetPathFilter(&mut self, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::core::Result<()>;
    fn GetIncludedEvents(&mut self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::Result<()>;
    fn GetExcludedEvents(&mut self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEventsFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEventsFilterVtbl {
        unsafe extern "system" fn GetPathFilter<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPathFilter(::core::mem::transmute_copy(&ppszfilter), ::core::mem::transmute_copy(&pmatch)).into()
        }
        unsafe extern "system" fn GetIncludedEvents<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIncludedEvents(::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)).into()
        }
        unsafe extern "system" fn GetExcludedEvents<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExcludedEvents(::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPathFilter: GetPathFilter::<Impl, IMPL_OFFSET>,
            GetIncludedEvents: GetIncludedEvents::<Impl, IMPL_OFFSET>,
            GetExcludedEvents: GetExcludedEvents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEventsFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileItemImpl: Sized + IOfflineFilesItemImpl {
    fn IsSparse(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsEncrypted(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesFileItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesFileItemVtbl {
        unsafe extern "system" fn IsSparse<Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSparse() {
                ::core::result::Result::Ok(ok__) => {
                    *pbissparse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEncrypted<Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEncrypted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisencrypted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsSparse: IsSparse::<Impl, IMPL_OFFSET>,
            IsEncrypted: IsEncrypted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesFileItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileSysInfoImpl: Sized {
    fn GetAttributes(&mut self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows::core::Result<u32>;
    fn GetTimes(&mut self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetFileSize(&mut self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesFileSysInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesFileSysInfoVtbl {
        unsafe extern "system" fn GetAttributes<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimes(::core::mem::transmute_copy(&copy), ::core::mem::transmute_copy(&pftcreationtime), ::core::mem::transmute_copy(&pftlastwritetime), ::core::mem::transmute_copy(&pftchangetime), ::core::mem::transmute_copy(&pftlastaccesstime)).into()
        }
        unsafe extern "system" fn GetFileSize<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAttributes: GetAttributes::<Impl, IMPL_OFFSET>,
            GetTimes: GetTimes::<Impl, IMPL_OFFSET>,
            GetFileSize: GetFileSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesFileSysInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesGhostInfoImpl: Sized {
    fn IsGhosted(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesGhostInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesGhostInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesGhostInfoVtbl {
        unsafe extern "system" fn IsGhosted<Impl: IOfflineFilesGhostInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGhosted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbghosted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsGhosted: IsGhosted::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesGhostInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItemImpl: Sized {
    fn GetItemType(&mut self) -> ::windows::core::Result<OFFLINEFILES_ITEM_TYPE>;
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetParentItem(&mut self) -> ::windows::core::Result<IOfflineFilesItem>;
    fn Refresh(&mut self, dwqueryflags: u32) -> ::windows::core::Result<()>;
    fn IsMarkedForDeletion(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesItemVtbl {
        unsafe extern "system" fn GetItemType<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *pitemtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&dwqueryflags)).into()
        }
        unsafe extern "system" fn IsMarkedForDeletion<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMarkedForDeletion() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmarkedfordeletion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetItemType: GetItemType::<Impl, IMPL_OFFSET>,
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            GetParentItem: GetParentItem::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            IsMarkedForDeletion: IsMarkedForDeletion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
pub trait IOfflineFilesItemContainerImpl: Sized {
    fn EnumItems(&mut self, dwqueryflags: u32) -> ::windows::core::Result<IEnumOfflineFilesItems>;
    fn EnumItemsEx(&mut self, pincludefilefilter: ::core::option::Option<IOfflineFilesItemFilter>, pincludedirfilter: ::core::option::Option<IOfflineFilesItemFilter>, pexcludefilefilter: ::core::option::Option<IOfflineFilesItemFilter>, pexcludedirfilter: ::core::option::Option<IOfflineFilesItemFilter>, dwenumflags: u32, dwqueryflags: u32) -> ::windows::core::Result<IEnumOfflineFilesItems>;
}
impl IOfflineFilesItemContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesItemContainerVtbl {
        unsafe extern "system" fn EnumItems<Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItems(::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsEx<Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItemsEx(::core::mem::transmute(&pincludefilefilter), ::core::mem::transmute(&pincludedirfilter), ::core::mem::transmute(&pexcludefilefilter), ::core::mem::transmute(&pexcludedirfilter), ::core::mem::transmute_copy(&dwenumflags), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumItems: EnumItems::<Impl, IMPL_OFFSET>,
            EnumItemsEx: EnumItemsEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItemContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItemFilterImpl: Sized {
    fn GetFilterFlags(&mut self, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::core::Result<()>;
    fn GetTimeFilter(&mut self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::core::Result<()>;
    fn GetPatternFilter(&mut self, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesItemFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesItemFilterVtbl {
        unsafe extern "system" fn GetFilterFlags<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFilterFlags(::core::mem::transmute_copy(&pullflags), ::core::mem::transmute_copy(&pullmask)).into()
        }
        unsafe extern "system" fn GetTimeFilter<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimeFilter(::core::mem::transmute_copy(&pfttime), ::core::mem::transmute_copy(&pbevaltimeofday), ::core::mem::transmute_copy(&ptimetype), ::core::mem::transmute_copy(&pcompare)).into()
        }
        unsafe extern "system" fn GetPatternFilter<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPatternFilter(::core::mem::transmute_copy(&pszpattern), ::core::mem::transmute_copy(&cchpattern)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFilterFlags: GetFilterFlags::<Impl, IMPL_OFFSET>,
            GetTimeFilter: GetTimeFilter::<Impl, IMPL_OFFSET>,
            GetPatternFilter: GetPatternFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItemFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfoImpl: Sized {
    fn IsPinned(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsPinnedForUser(&mut self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinnedForUserByPolicy(&mut self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinnedForComputer(&mut self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinnedForFolderRedirection(&mut self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesPinInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesPinInfoVtbl {
        unsafe extern "system" fn IsPinned<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbpinned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForUser<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPinnedForUser(::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPinnedForUserByPolicy(::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForComputer<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPinnedForComputer(::core::mem::transmute_copy(&pbpinnedforcomputer), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPinnedForFolderRedirection(::core::mem::transmute_copy(&pbpinnedforfolderredirection), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsPinned: IsPinned::<Impl, IMPL_OFFSET>,
            IsPinnedForUser: IsPinnedForUser::<Impl, IMPL_OFFSET>,
            IsPinnedForUserByPolicy: IsPinnedForUserByPolicy::<Impl, IMPL_OFFSET>,
            IsPinnedForComputer: IsPinnedForComputer::<Impl, IMPL_OFFSET>,
            IsPinnedForFolderRedirection: IsPinnedForFolderRedirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfo2Impl: Sized + IOfflineFilesPinInfoImpl {
    fn IsPartlyPinned(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesPinInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesPinInfo2Vtbl {
        unsafe extern "system" fn IsPartlyPinned<Impl: IOfflineFilesPinInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPartlyPinned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbpartlypinned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IOfflineFilesPinInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), IsPartlyPinned: IsPartlyPinned::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesProgressImpl: Sized {
    fn Begin(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn QueryAbort(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn End(&mut self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesProgressVtbl {
        unsafe extern "system" fn Begin<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin() {
                ::core::result::Result::Ok(ok__) => {
                    *pbabort = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAbort<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *pbabort = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).End(::core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Begin: Begin::<Impl, IMPL_OFFSET>,
            QueryAbort: QueryAbort::<Impl, IMPL_OFFSET>,
            End: End::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesServerItemImpl: Sized + IOfflineFilesItemImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesServerItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesServerItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesServerItemVtbl {
        Self { base: IOfflineFilesItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesServerItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOfflineFilesSettingImpl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetValueType(&mut self) -> ::windows::core::Result<OFFLINEFILES_SETTING_VALUE_TYPE>;
    fn GetPreference(&mut self, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::Result<()>;
    fn GetPreferenceScope(&mut self) -> ::windows::core::Result<u32>;
    fn SetPreference(&mut self, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::Result<()>;
    fn DeletePreference(&mut self, dwscope: u32) -> ::windows::core::Result<()>;
    fn GetPolicy(&mut self, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::Result<()>;
    fn GetPolicyScope(&mut self) -> ::windows::core::Result<u32>;
    fn GetValue(&mut self, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOfflineFilesSettingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSettingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSettingVtbl {
        unsafe extern "system" fn GetName<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueType<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPreference(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPreferenceScope<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferenceScope() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreference(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn DeletePreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePreference(::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPolicy<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPolicy(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPolicyScope<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicyScope() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pbsetbypolicy)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetValueType: GetValueType::<Impl, IMPL_OFFSET>,
            GetPreference: GetPreference::<Impl, IMPL_OFFSET>,
            GetPreferenceScope: GetPreferenceScope::<Impl, IMPL_OFFSET>,
            SetPreference: SetPreference::<Impl, IMPL_OFFSET>,
            DeletePreference: DeletePreference::<Impl, IMPL_OFFSET>,
            GetPolicy: GetPolicy::<Impl, IMPL_OFFSET>,
            GetPolicyScope: GetPolicyScope::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSetting as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareInfoImpl: Sized {
    fn GetShareItem(&mut self) -> ::windows::core::Result<IOfflineFilesShareItem>;
    fn GetShareCachingMode(&mut self) -> ::windows::core::Result<OFFLINEFILES_CACHING_MODE>;
    fn IsShareDfsJunction(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesShareInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesShareInfoVtbl {
        unsafe extern "system" fn GetShareItem<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshareitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShareItem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppshareitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShareCachingMode<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShareCachingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pcachingmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShareDfsJunction<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShareDfsJunction() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisdfsjunction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetShareItem: GetShareItem::<Impl, IMPL_OFFSET>,
            GetShareCachingMode: GetShareCachingMode::<Impl, IMPL_OFFSET>,
            IsShareDfsJunction: IsShareDfsJunction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesShareInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareItemImpl: Sized + IOfflineFilesItemImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesShareItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesShareItemVtbl {
        Self { base: IOfflineFilesItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesShareItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSimpleProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn ItemBegin(&mut self, pszfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn ItemResult(&mut self, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSimpleProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSimpleProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSimpleProgressVtbl {
        unsafe extern "system" fn ItemBegin<Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemBegin(::core::mem::transmute_copy(&pszfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemResult<Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemResult(::core::mem::transmute_copy(&pszfile), ::core::mem::transmute_copy(&hrresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesProgressVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ItemBegin: ItemBegin::<Impl, IMPL_OFFSET>,
            ItemResult: ItemResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSimpleProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspendImpl: Sized {
    fn SuspendRoot(&mut self, bsuspend: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSuspendVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSuspendVtbl {
        unsafe extern "system" fn SuspendRoot<Impl: IOfflineFilesSuspendImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendRoot(::core::mem::transmute_copy(&bsuspend)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SuspendRoot: SuspendRoot::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSuspend as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspendInfoImpl: Sized {
    fn IsSuspended(&mut self, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSuspendInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSuspendInfoVtbl {
        unsafe extern "system" fn IsSuspended<Impl: IOfflineFilesSuspendInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSuspended(::core::mem::transmute_copy(&pbsuspended), ::core::mem::transmute_copy(&pbsuspendedroot)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsSuspended: IsSuspended::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSuspendInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncConflictHandlerImpl: Sized {
    fn ResolveConflict(&mut self, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncConflictHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncConflictHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncConflictHandlerVtbl {
        unsafe extern "system" fn ResolveConflict<Impl: IOfflineFilesSyncConflictHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveConflict(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&fstateknown), ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&fchangedetails), ::core::mem::transmute_copy(&pconflictresolution), ::core::mem::transmute_copy(&ppsznewname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ResolveConflict: ResolveConflict::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncConflictHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesSyncErrorInfoImpl: Sized + IOfflineFilesErrorInfoImpl {
    fn GetSyncOperation(&mut self) -> ::windows::core::Result<OFFLINEFILES_SYNC_OPERATION>;
    fn GetItemChangeFlags(&mut self) -> ::windows::core::Result<u32>;
    fn InfoEnumerated(&mut self, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InfoAvailable(&mut self, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocalInfo(&mut self) -> ::windows::core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetRemoteInfo(&mut self) -> ::windows::core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetOriginalInfo(&mut self) -> ::windows::core::Result<IOfflineFilesSyncErrorItemInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOfflineFilesSyncErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncErrorInfoVtbl {
        unsafe extern "system" fn GetSyncOperation<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *psyncop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemChangeFlags<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitemchangeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemChangeFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwitemchangeflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoEnumerated<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InfoEnumerated(::core::mem::transmute_copy(&pblocalenumerated), ::core::mem::transmute_copy(&pbremoteenumerated), ::core::mem::transmute_copy(&pboriginalenumerated)).into()
        }
        unsafe extern "system" fn InfoAvailable<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InfoAvailable(::core::mem::transmute_copy(&pblocalinfo), ::core::mem::transmute_copy(&pbremoteinfo), ::core::mem::transmute_copy(&pboriginalinfo)).into()
        }
        unsafe extern "system" fn GetLocalInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesErrorInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSyncOperation: GetSyncOperation::<Impl, IMPL_OFFSET>,
            GetItemChangeFlags: GetItemChangeFlags::<Impl, IMPL_OFFSET>,
            InfoEnumerated: InfoEnumerated::<Impl, IMPL_OFFSET>,
            InfoAvailable: InfoAvailable::<Impl, IMPL_OFFSET>,
            GetLocalInfo: GetLocalInfo::<Impl, IMPL_OFFSET>,
            GetRemoteInfo: GetRemoteInfo::<Impl, IMPL_OFFSET>,
            GetOriginalInfo: GetOriginalInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncErrorItemInfoImpl: Sized {
    fn GetFileAttributes(&mut self) -> ::windows::core::Result<u32>;
    fn GetFileTimes(&mut self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncErrorItemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncErrorItemInfoVtbl {
        unsafe extern "system" fn GetFileAttributes<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileTimes<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFileTimes(::core::mem::transmute_copy(&pftlastwrite), ::core::mem::transmute_copy(&pftchange)).into()
        }
        unsafe extern "system" fn GetFileSize<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFileAttributes: GetFileAttributes::<Impl, IMPL_OFFSET>,
            GetFileTimes: GetFileTimes::<Impl, IMPL_OFFSET>,
            GetFileSize: GetFileSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorItemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn SyncItemBegin(&mut self, pszfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn SyncItemResult(&mut self, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, perrorinfo: ::core::option::Option<IOfflineFilesSyncErrorInfo>) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncProgressVtbl {
        unsafe extern "system" fn SyncItemBegin<Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncItemBegin(::core::mem::transmute_copy(&pszfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncItemResult<Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, perrorinfo: ::windows::core::RawPtr, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncItemResult(::core::mem::transmute_copy(&pszfile), ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute(&perrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesProgressVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SyncItemBegin: SyncItemBegin::<Impl, IMPL_OFFSET>,
            SyncItemResult: SyncItemResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesTransparentCacheInfoImpl: Sized {
    fn IsTransparentlyCached(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesTransparentCacheInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesTransparentCacheInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesTransparentCacheInfoVtbl {
        unsafe extern "system" fn IsTransparentlyCached<Impl: IOfflineFilesTransparentCacheInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransparentlyCached() {
                ::core::result::Result::Ok(ok__) => {
                    *pbtransparentlycached = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsTransparentlyCached: IsTransparentlyCached::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesTransparentCacheInfo as ::windows::core::Interface>::IID
    }
}

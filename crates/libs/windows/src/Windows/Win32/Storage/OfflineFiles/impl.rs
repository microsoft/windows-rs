pub trait IEnumOfflineFilesItems_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IOfflineFilesItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOfflineFilesItems>;
}
impl IEnumOfflineFilesItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>() -> IEnumOfflineFilesItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOfflineFilesItems as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOfflineFilesSettings_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IOfflineFilesSetting>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumOfflineFilesSettings>;
}
impl IEnumOfflineFilesSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>() -> IEnumOfflineFilesSettings_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOfflineFilesSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCache_Impl: Sized {
    fn Synchronize(&mut self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: &::core::option::Option<IOfflineFilesSyncConflictHandler>, piprogress: &::core::option::Option<IOfflineFilesSyncProgress>, psyncid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn DeleteItems(&mut self, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: &::core::option::Option<IOfflineFilesSimpleProgress>) -> ::windows::core::Result<()>;
    fn DeleteItemsForUser(&mut self, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: &::core::option::Option<IOfflineFilesSimpleProgress>) -> ::windows::core::Result<()>;
    fn Pin(&mut self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: &::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn Unpin(&mut self, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: &::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn GetEncryptionStatus(&mut self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Encrypt(&mut self, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: &::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn FindItem(&mut self, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32) -> ::windows::core::Result<IOfflineFilesItem>;
    fn FindItemEx(&mut self, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: &::core::option::Option<IOfflineFilesItemFilter>, pincludedirfilter: &::core::option::Option<IOfflineFilesItemFilter>, pexcludefilefilter: &::core::option::Option<IOfflineFilesItemFilter>, pexcludedirfilter: &::core::option::Option<IOfflineFilesItemFilter>, dwqueryflags: u32) -> ::windows::core::Result<IOfflineFilesItem>;
    fn RenameItem(&mut self, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDiskSpaceInformation(&mut self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::Result<()>;
    fn SetDiskSpaceLimits(&mut self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::Result<()>;
    fn ProcessAdminPinPolicy(&mut self, ppinprogress: &::core::option::Option<IOfflineFilesSyncProgress>, punpinprogress: &::core::option::Option<IOfflineFilesSyncProgress>) -> ::windows::core::Result<()>;
    fn GetSettingObject(&mut self, pszsettingname: super::super::Foundation::PWSTR) -> ::windows::core::Result<IOfflineFilesSetting>;
    fn EnumSettingObjects(&mut self) -> ::windows::core::Result<IEnumOfflineFilesSettings>;
    fn IsPathCacheable(&mut self, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>() -> IOfflineFilesCache_Vtbl {
        unsafe extern "system" fn Synchronize<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::windows::core::RawPtr, piprogress: ::windows::core::RawPtr, psyncid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Synchronize(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwsynccontrol), ::core::mem::transmute(&pisyncconflicthandler), ::core::mem::transmute(&piprogress), ::core::mem::transmute_copy(&psyncid)).into()
        }
        unsafe extern "system" fn DeleteItems<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteItems(::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&basync), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn DeleteItemsForUser<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteItemsForUser(::core::mem::transmute_copy(&pszuser), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&basync), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn Pin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pin(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&bdeep), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwpincontrolflags), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn Unpin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unpin(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&rgpszpaths), ::core::mem::transmute_copy(&cpaths), ::core::mem::transmute_copy(&bdeep), ::core::mem::transmute_copy(&basync), ::core::mem::transmute_copy(&dwpincontrolflags), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn GetEncryptionStatus<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEncryptionStatus(::core::mem::transmute_copy(&pbencrypted), ::core::mem::transmute_copy(&pbpartial)).into()
        }
        unsafe extern "system" fn Encrypt<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Encrypt(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&bencrypt), ::core::mem::transmute_copy(&dwencryptioncontrolflags), ::core::mem::transmute_copy(&basync), ::core::mem::transmute(&piprogress)).into()
        }
        unsafe extern "system" fn FindItem<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindItem(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemEx<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindItemEx(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute(&pincludefilefilter), ::core::mem::transmute(&pincludedirfilter), ::core::mem::transmute(&pexcludefilefilter), ::core::mem::transmute(&pexcludedirfilter), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameItem<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenameItem(::core::mem::transmute_copy(&pszpathoriginal), ::core::mem::transmute_copy(&pszpathnew), ::core::mem::transmute_copy(&breplaceifexists)).into()
        }
        unsafe extern "system" fn GetLocation<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDiskSpaceInformation(::core::mem::transmute_copy(&pcbvolumetotal), ::core::mem::transmute_copy(&pcblimit), ::core::mem::transmute_copy(&pcbused), ::core::mem::transmute_copy(&pcbunpinnedlimit), ::core::mem::transmute_copy(&pcbunpinnedused)).into()
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDiskSpaceLimits(::core::mem::transmute_copy(&cblimit), ::core::mem::transmute_copy(&cbunpinnedlimit)).into()
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinprogress: ::windows::core::RawPtr, punpinprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessAdminPinPolicy(::core::mem::transmute(&ppinprogress), ::core::mem::transmute(&punpinprogress)).into()
        }
        unsafe extern "system" fn GetSettingObject<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsettingname: super::super::Foundation::PWSTR, ppsetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSettingObject(::core::mem::transmute_copy(&pszsettingname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSettingObjects<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumSettingObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathCacheable<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPathCacheable(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&pbcacheable), ::core::mem::transmute_copy(&psharecachingmode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Synchronize: Synchronize::<Identity, Impl, OFFSET>,
            DeleteItems: DeleteItems::<Identity, Impl, OFFSET>,
            DeleteItemsForUser: DeleteItemsForUser::<Identity, Impl, OFFSET>,
            Pin: Pin::<Identity, Impl, OFFSET>,
            Unpin: Unpin::<Identity, Impl, OFFSET>,
            GetEncryptionStatus: GetEncryptionStatus::<Identity, Impl, OFFSET>,
            Encrypt: Encrypt::<Identity, Impl, OFFSET>,
            FindItem: FindItem::<Identity, Impl, OFFSET>,
            FindItemEx: FindItemEx::<Identity, Impl, OFFSET>,
            RenameItem: RenameItem::<Identity, Impl, OFFSET>,
            GetLocation: GetLocation::<Identity, Impl, OFFSET>,
            GetDiskSpaceInformation: GetDiskSpaceInformation::<Identity, Impl, OFFSET>,
            SetDiskSpaceLimits: SetDiskSpaceLimits::<Identity, Impl, OFFSET>,
            ProcessAdminPinPolicy: ProcessAdminPinPolicy::<Identity, Impl, OFFSET>,
            GetSettingObject: GetSettingObject::<Identity, Impl, OFFSET>,
            EnumSettingObjects: EnumSettingObjects::<Identity, Impl, OFFSET>,
            IsPathCacheable: IsPathCacheable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCache2_Impl: Sized + IOfflineFilesCache_Impl {
    fn RenameItemEx(&mut self, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesCache2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache2_Impl, const OFFSET: isize>() -> IOfflineFilesCache2_Vtbl {
        unsafe extern "system" fn RenameItemEx<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenameItemEx(::core::mem::transmute_copy(&pszpathoriginal), ::core::mem::transmute_copy(&pszpathnew), ::core::mem::transmute_copy(&breplaceifexists)).into()
        }
        Self { base: IOfflineFilesCache_Vtbl::new::<Identity, Impl, OFFSET>(), RenameItemEx: RenameItemEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesCache2 as ::windows::core::Interface>::IID || iid == &<IOfflineFilesCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesChangeInfo_Impl: Sized {
    fn IsDirty(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsDeletedOffline(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsCreatedOffline(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedData(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedAttributes(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsLocallyModifiedTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesChangeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>() -> IOfflineFilesChangeInfo_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdirty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeletedOffline<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDeletedOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdeletedoffline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCreatedOffline<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCreatedOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *pbcreatedoffline = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedData<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLocallyModifiedData() {
                ::core::result::Result::Ok(ok__) => {
                    *pblocallymodifieddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLocallyModifiedAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pblocallymodifiedattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsLocallyModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pblocallymodifiedtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            IsDeletedOffline: IsDeletedOffline::<Identity, Impl, OFFSET>,
            IsCreatedOffline: IsCreatedOffline::<Identity, Impl, OFFSET>,
            IsLocallyModifiedData: IsLocallyModifiedData::<Identity, Impl, OFFSET>,
            IsLocallyModifiedAttributes: IsLocallyModifiedAttributes::<Identity, Impl, OFFSET>,
            IsLocallyModifiedTime: IsLocallyModifiedTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesChangeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesConnectionInfo_Impl: Sized {
    fn GetConnectState(&mut self, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::core::Result<()>;
    fn SetConnectState(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::core::Result<()>;
    fn TransitionOnline(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::Result<()>;
    fn TransitionOffline(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesConnectionInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>() -> IOfflineFilesConnectionInfo_Vtbl {
        unsafe extern "system" fn GetConnectState<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectState(::core::mem::transmute_copy(&pconnectstate), ::core::mem::transmute_copy(&pofflinereason)).into()
        }
        unsafe extern "system" fn SetConnectState<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectState(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&connectstate)).into()
        }
        unsafe extern "system" fn TransitionOnline<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransitionOnline(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn TransitionOffline<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransitionOffline(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&bforceopenfilesclosed)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbopenfilespreventedtransition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnectState: GetConnectState::<Identity, Impl, OFFSET>,
            SetConnectState: SetConnectState::<Identity, Impl, OFFSET>,
            TransitionOnline: TransitionOnline::<Identity, Impl, OFFSET>,
            TransitionOffline: TransitionOffline::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesConnectionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesDirectoryItem_Impl: Sized + IOfflineFilesItem_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesDirectoryItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirectoryItem_Impl, const OFFSET: isize>() -> IOfflineFilesDirectoryItem_Vtbl {
        Self { base: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesDirectoryItem as ::windows::core::Interface>::IID || iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
pub trait IOfflineFilesDirtyInfo_Impl: Sized {
    fn LocalDirtyByteCount(&mut self) -> ::windows::core::Result<i64>;
    fn RemoteDirtyByteCount(&mut self) -> ::windows::core::Result<i64>;
}
impl IOfflineFilesDirtyInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>() -> IOfflineFilesDirtyInfo_Vtbl {
        unsafe extern "system" fn LocalDirtyByteCount<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalDirtyByteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirtybytecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirtyInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RemoteDirtyByteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirtybytecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LocalDirtyByteCount: LocalDirtyByteCount::<Identity, Impl, OFFSET>,
            RemoteDirtyByteCount: RemoteDirtyByteCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesDirtyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesErrorInfo_Impl: Sized {
    fn GetRawData(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::BYTE_BLOB>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOfflineFilesErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>() -> IOfflineFilesErrorInfo_Vtbl {
        unsafe extern "system" fn GetRawData<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRawData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppblob = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRawData: GetRawData::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents_Impl: Sized {
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
impl IOfflineFilesEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>() -> IOfflineFilesEvents_Vtbl {
        unsafe extern "system" fn CacheMoved<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CacheMoved(::core::mem::transmute_copy(&pszoldpath), ::core::mem::transmute_copy(&psznewpath)).into()
        }
        unsafe extern "system" fn CacheIsFull<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CacheIsFull().into()
        }
        unsafe extern "system" fn CacheIsCorrupted<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CacheIsCorrupted().into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enabled(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn EncryptionChanged<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EncryptionChanged(::core::mem::transmute_copy(&bwasencrypted), ::core::mem::transmute_copy(&bwaspartial), ::core::mem::transmute_copy(&bisencrypted), ::core::mem::transmute_copy(&bispartial)).into()
        }
        unsafe extern "system" fn SyncBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncBegin(::core::mem::transmute_copy(&rsyncid)).into()
        }
        unsafe extern "system" fn SyncFileResult<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncFileResult(::core::mem::transmute_copy(&rsyncid), ::core::mem::transmute_copy(&pszfile), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn SyncConflictRecAdded<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncConflictRecAdded(::core::mem::transmute_copy(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncConflictRecUpdated(::core::mem::transmute_copy(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncConflictRecRemoved(::core::mem::transmute_copy(&pszconflictpath), ::core::mem::transmute_copy(&pftconflictdatetime), ::core::mem::transmute_copy(&conflictsyncstate)).into()
        }
        unsafe extern "system" fn SyncEnd<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncEnd(::core::mem::transmute_copy(&rsyncid), ::core::mem::transmute_copy(&hrresult)).into()
        }
        unsafe extern "system" fn NetTransportArrived<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NetTransportArrived().into()
        }
        unsafe extern "system" fn NoNetTransports<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NoNetTransports().into()
        }
        unsafe extern "system" fn ItemDisconnected<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemDisconnected(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemReconnected<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemReconnected(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemAvailableOffline<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemAvailableOffline(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemNotAvailableOffline(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemPinned<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemPinned(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemNotPinned<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemNotPinned(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemModified<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemModified(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype), ::core::mem::transmute_copy(&bmodifieddata), ::core::mem::transmute_copy(&bmodifiedattributes)).into()
        }
        unsafe extern "system" fn ItemAddedToCache<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemAddedToCache(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemDeletedFromCache<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemDeletedFromCache(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn ItemRenamed<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemRenamed(::core::mem::transmute_copy(&pszoldpath), ::core::mem::transmute_copy(&psznewpath), ::core::mem::transmute_copy(&itemtype)).into()
        }
        unsafe extern "system" fn DataLost<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DataLost().into()
        }
        unsafe extern "system" fn Ping<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Ping().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CacheMoved: CacheMoved::<Identity, Impl, OFFSET>,
            CacheIsFull: CacheIsFull::<Identity, Impl, OFFSET>,
            CacheIsCorrupted: CacheIsCorrupted::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            EncryptionChanged: EncryptionChanged::<Identity, Impl, OFFSET>,
            SyncBegin: SyncBegin::<Identity, Impl, OFFSET>,
            SyncFileResult: SyncFileResult::<Identity, Impl, OFFSET>,
            SyncConflictRecAdded: SyncConflictRecAdded::<Identity, Impl, OFFSET>,
            SyncConflictRecUpdated: SyncConflictRecUpdated::<Identity, Impl, OFFSET>,
            SyncConflictRecRemoved: SyncConflictRecRemoved::<Identity, Impl, OFFSET>,
            SyncEnd: SyncEnd::<Identity, Impl, OFFSET>,
            NetTransportArrived: NetTransportArrived::<Identity, Impl, OFFSET>,
            NoNetTransports: NoNetTransports::<Identity, Impl, OFFSET>,
            ItemDisconnected: ItemDisconnected::<Identity, Impl, OFFSET>,
            ItemReconnected: ItemReconnected::<Identity, Impl, OFFSET>,
            ItemAvailableOffline: ItemAvailableOffline::<Identity, Impl, OFFSET>,
            ItemNotAvailableOffline: ItemNotAvailableOffline::<Identity, Impl, OFFSET>,
            ItemPinned: ItemPinned::<Identity, Impl, OFFSET>,
            ItemNotPinned: ItemNotPinned::<Identity, Impl, OFFSET>,
            ItemModified: ItemModified::<Identity, Impl, OFFSET>,
            ItemAddedToCache: ItemAddedToCache::<Identity, Impl, OFFSET>,
            ItemDeletedFromCache: ItemDeletedFromCache::<Identity, Impl, OFFSET>,
            ItemRenamed: ItemRenamed::<Identity, Impl, OFFSET>,
            DataLost: DataLost::<Identity, Impl, OFFSET>,
            Ping: Ping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents2_Impl: Sized + IOfflineFilesEvents_Impl {
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
impl IOfflineFilesEvents2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>() -> IOfflineFilesEvents2_Vtbl {
        unsafe extern "system" fn ItemReconnectBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemReconnectBegin().into()
        }
        unsafe extern "system" fn ItemReconnectEnd<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ItemReconnectEnd().into()
        }
        unsafe extern "system" fn CacheEvictBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CacheEvictBegin().into()
        }
        unsafe extern "system" fn CacheEvictEnd<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CacheEvictEnd().into()
        }
        unsafe extern "system" fn BackgroundSyncBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundSyncBegin(::core::mem::transmute_copy(&dwsynccontrolflags)).into()
        }
        unsafe extern "system" fn BackgroundSyncEnd<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackgroundSyncEnd(::core::mem::transmute_copy(&dwsynccontrolflags)).into()
        }
        unsafe extern "system" fn PolicyChangeDetected<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PolicyChangeDetected().into()
        }
        unsafe extern "system" fn PreferenceChangeDetected<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreferenceChangeDetected().into()
        }
        unsafe extern "system" fn SettingsChangesApplied<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SettingsChangesApplied().into()
        }
        Self {
            base: IOfflineFilesEvents_Vtbl::new::<Identity, Impl, OFFSET>(),
            ItemReconnectBegin: ItemReconnectBegin::<Identity, Impl, OFFSET>,
            ItemReconnectEnd: ItemReconnectEnd::<Identity, Impl, OFFSET>,
            CacheEvictBegin: CacheEvictBegin::<Identity, Impl, OFFSET>,
            CacheEvictEnd: CacheEvictEnd::<Identity, Impl, OFFSET>,
            BackgroundSyncBegin: BackgroundSyncBegin::<Identity, Impl, OFFSET>,
            BackgroundSyncEnd: BackgroundSyncEnd::<Identity, Impl, OFFSET>,
            PolicyChangeDetected: PolicyChangeDetected::<Identity, Impl, OFFSET>,
            PreferenceChangeDetected: PreferenceChangeDetected::<Identity, Impl, OFFSET>,
            SettingsChangesApplied: SettingsChangesApplied::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents2 as ::windows::core::Interface>::IID || iid == &<IOfflineFilesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents3_Impl: Sized + IOfflineFilesEvents_Impl + IOfflineFilesEvents2_Impl {
    fn TransparentCacheItemNotify(&mut self, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn PrefetchFileBegin(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn PrefetchFileEnd(&mut self, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>() -> IOfflineFilesEvents3_Vtbl {
        unsafe extern "system" fn TransparentCacheItemNotify<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TransparentCacheItemNotify(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&eventtype), ::core::mem::transmute_copy(&itemtype), ::core::mem::transmute_copy(&bmodifieddata), ::core::mem::transmute_copy(&bmodifiedattributes), ::core::mem::transmute_copy(&pzsoldpath)).into()
        }
        unsafe extern "system" fn PrefetchFileBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrefetchFileBegin(::core::mem::transmute_copy(&pszpath)).into()
        }
        unsafe extern "system" fn PrefetchFileEnd<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrefetchFileEnd(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base: IOfflineFilesEvents2_Vtbl::new::<Identity, Impl, OFFSET>(),
            TransparentCacheItemNotify: TransparentCacheItemNotify::<Identity, Impl, OFFSET>,
            PrefetchFileBegin: PrefetchFileBegin::<Identity, Impl, OFFSET>,
            PrefetchFileEnd: PrefetchFileEnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents3 as ::windows::core::Interface>::IID || iid == &<IOfflineFilesEvents as ::windows::core::Interface>::IID || iid == &<IOfflineFilesEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents4_Impl: Sized + IOfflineFilesEvents_Impl + IOfflineFilesEvents2_Impl + IOfflineFilesEvents3_Impl {
    fn PrefetchCloseHandleBegin(&mut self) -> ::windows::core::Result<()>;
    fn PrefetchCloseHandleEnd(&mut self, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents4_Impl, const OFFSET: isize>() -> IOfflineFilesEvents4_Vtbl {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrefetchCloseHandleBegin().into()
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrefetchCloseHandleEnd(::core::mem::transmute_copy(&dwclosedhandlecount), ::core::mem::transmute_copy(&dwopenhandlecount), ::core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base: IOfflineFilesEvents3_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrefetchCloseHandleBegin: PrefetchCloseHandleBegin::<Identity, Impl, OFFSET>,
            PrefetchCloseHandleEnd: PrefetchCloseHandleEnd::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents4 as ::windows::core::Interface>::IID || iid == &<IOfflineFilesEvents as ::windows::core::Interface>::IID || iid == &<IOfflineFilesEvents2 as ::windows::core::Interface>::IID || iid == &<IOfflineFilesEvents3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEventsFilter_Impl: Sized {
    fn GetPathFilter(&mut self, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::core::Result<()>;
    fn GetIncludedEvents(&mut self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::Result<()>;
    fn GetExcludedEvents(&mut self, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEventsFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>() -> IOfflineFilesEventsFilter_Vtbl {
        unsafe extern "system" fn GetPathFilter<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPathFilter(::core::mem::transmute_copy(&ppszfilter), ::core::mem::transmute_copy(&pmatch)).into()
        }
        unsafe extern "system" fn GetIncludedEvents<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIncludedEvents(::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)).into()
        }
        unsafe extern "system" fn GetExcludedEvents<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetExcludedEvents(::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPathFilter: GetPathFilter::<Identity, Impl, OFFSET>,
            GetIncludedEvents: GetIncludedEvents::<Identity, Impl, OFFSET>,
            GetExcludedEvents: GetExcludedEvents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEventsFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileItem_Impl: Sized + IOfflineFilesItem_Impl {
    fn IsSparse(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsEncrypted(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileItem_Impl, const OFFSET: isize>() -> IOfflineFilesFileItem_Vtbl {
        unsafe extern "system" fn IsSparse<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSparse() {
                ::core::result::Result::Ok(ok__) => {
                    *pbissparse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEncrypted<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEncrypted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisencrypted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsSparse: IsSparse::<Identity, Impl, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesFileItem as ::windows::core::Interface>::IID || iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileSysInfo_Impl: Sized {
    fn GetAttributes(&mut self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows::core::Result<u32>;
    fn GetTimes(&mut self, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetFileSize(&mut self, copy: OFFLINEFILES_ITEM_COPY) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesFileSysInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>() -> IOfflineFilesFileSysInfo_Vtbl {
        unsafe extern "system" fn GetAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributes(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTimes(::core::mem::transmute_copy(&copy), ::core::mem::transmute_copy(&pftcreationtime), ::core::mem::transmute_copy(&pftlastwritetime), ::core::mem::transmute_copy(&pftchangetime), ::core::mem::transmute_copy(&pftlastaccesstime)).into()
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&copy)) {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAttributes: GetAttributes::<Identity, Impl, OFFSET>,
            GetTimes: GetTimes::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesFileSysInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesGhostInfo_Impl: Sized {
    fn IsGhosted(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesGhostInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesGhostInfo_Impl, const OFFSET: isize>() -> IOfflineFilesGhostInfo_Vtbl {
        unsafe extern "system" fn IsGhosted<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesGhostInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsGhosted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbghosted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsGhosted: IsGhosted::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesGhostInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItem_Impl: Sized {
    fn GetItemType(&mut self) -> ::windows::core::Result<OFFLINEFILES_ITEM_TYPE>;
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetParentItem(&mut self) -> ::windows::core::Result<IOfflineFilesItem>;
    fn Refresh(&mut self, dwqueryflags: u32) -> ::windows::core::Result<()>;
    fn IsMarkedForDeletion(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>() -> IOfflineFilesItem_Vtbl {
        unsafe extern "system" fn GetItemType<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *pitemtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh(::core::mem::transmute_copy(&dwqueryflags)).into()
        }
        unsafe extern "system" fn IsMarkedForDeletion<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMarkedForDeletion() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmarkedfordeletion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItemType: GetItemType::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            IsMarkedForDeletion: IsMarkedForDeletion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
pub trait IOfflineFilesItemContainer_Impl: Sized {
    fn EnumItems(&mut self, dwqueryflags: u32) -> ::windows::core::Result<IEnumOfflineFilesItems>;
    fn EnumItemsEx(&mut self, pincludefilefilter: &::core::option::Option<IOfflineFilesItemFilter>, pincludedirfilter: &::core::option::Option<IOfflineFilesItemFilter>, pexcludefilefilter: &::core::option::Option<IOfflineFilesItemFilter>, pexcludedirfilter: &::core::option::Option<IOfflineFilesItemFilter>, dwenumflags: u32, dwqueryflags: u32) -> ::windows::core::Result<IEnumOfflineFilesItems>;
}
impl IOfflineFilesItemContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: isize>() -> IOfflineFilesItemContainer_Vtbl {
        unsafe extern "system" fn EnumItems<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumItems(::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsEx<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumItemsEx(::core::mem::transmute(&pincludefilefilter), ::core::mem::transmute(&pincludedirfilter), ::core::mem::transmute(&pexcludefilefilter), ::core::mem::transmute(&pexcludedirfilter), ::core::mem::transmute_copy(&dwenumflags), ::core::mem::transmute_copy(&dwqueryflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumItems: EnumItems::<Identity, Impl, OFFSET>,
            EnumItemsEx: EnumItemsEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItemContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItemFilter_Impl: Sized {
    fn GetFilterFlags(&mut self, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::core::Result<()>;
    fn GetTimeFilter(&mut self, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::core::Result<()>;
    fn GetPatternFilter(&mut self, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesItemFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>() -> IOfflineFilesItemFilter_Vtbl {
        unsafe extern "system" fn GetFilterFlags<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFilterFlags(::core::mem::transmute_copy(&pullflags), ::core::mem::transmute_copy(&pullmask)).into()
        }
        unsafe extern "system" fn GetTimeFilter<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTimeFilter(::core::mem::transmute_copy(&pfttime), ::core::mem::transmute_copy(&pbevaltimeofday), ::core::mem::transmute_copy(&ptimetype), ::core::mem::transmute_copy(&pcompare)).into()
        }
        unsafe extern "system" fn GetPatternFilter<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPatternFilter(::core::mem::transmute_copy(&pszpattern), ::core::mem::transmute_copy(&cchpattern)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFilterFlags: GetFilterFlags::<Identity, Impl, OFFSET>,
            GetTimeFilter: GetTimeFilter::<Identity, Impl, OFFSET>,
            GetPatternFilter: GetPatternFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItemFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfo_Impl: Sized {
    fn IsPinned(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsPinnedForUser(&mut self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinnedForUserByPolicy(&mut self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinnedForComputer(&mut self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsPinnedForFolderRedirection(&mut self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesPinInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>() -> IOfflineFilesPinInfo_Vtbl {
        unsafe extern "system" fn IsPinned<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPinned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbpinned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForUser<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPinnedForUser(::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPinnedForUserByPolicy(::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForComputer<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPinnedForComputer(::core::mem::transmute_copy(&pbpinnedforcomputer), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsPinnedForFolderRedirection(::core::mem::transmute_copy(&pbpinnedforfolderredirection), ::core::mem::transmute_copy(&pbinherit)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsPinned: IsPinned::<Identity, Impl, OFFSET>,
            IsPinnedForUser: IsPinnedForUser::<Identity, Impl, OFFSET>,
            IsPinnedForUserByPolicy: IsPinnedForUserByPolicy::<Identity, Impl, OFFSET>,
            IsPinnedForComputer: IsPinnedForComputer::<Identity, Impl, OFFSET>,
            IsPinnedForFolderRedirection: IsPinnedForFolderRedirection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfo2_Impl: Sized + IOfflineFilesPinInfo_Impl {
    fn IsPartlyPinned(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesPinInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo2_Impl, const OFFSET: isize>() -> IOfflineFilesPinInfo2_Vtbl {
        unsafe extern "system" fn IsPartlyPinned<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsPartlyPinned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbpartlypinned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IOfflineFilesPinInfo_Vtbl::new::<Identity, Impl, OFFSET>(), IsPartlyPinned: IsPartlyPinned::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo2 as ::windows::core::Interface>::IID || iid == &<IOfflineFilesPinInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesProgress_Impl: Sized {
    fn Begin(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn QueryAbort(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn End(&mut self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>() -> IOfflineFilesProgress_Vtbl {
        unsafe extern "system" fn Begin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Begin() {
                ::core::result::Result::Ok(ok__) => {
                    *pbabort = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAbort<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *pbabort = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).End(::core::mem::transmute_copy(&hrresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, Impl, OFFSET>,
            QueryAbort: QueryAbort::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesServerItem_Impl: Sized + IOfflineFilesItem_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesServerItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesServerItem_Impl, const OFFSET: isize>() -> IOfflineFilesServerItem_Vtbl {
        Self { base: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesServerItem as ::windows::core::Interface>::IID || iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IOfflineFilesSetting_Impl: Sized {
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
impl IOfflineFilesSetting_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>() -> IOfflineFilesSetting_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueType<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreference<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPreference(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPreferenceScope<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreferenceScope() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreference<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPreference(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn DeletePreference<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePreference(::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPolicy(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&dwscope)).into()
        }
        unsafe extern "system" fn GetPolicyScope<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPolicyScope() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSetting_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pbsetbypolicy)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetValueType: GetValueType::<Identity, Impl, OFFSET>,
            GetPreference: GetPreference::<Identity, Impl, OFFSET>,
            GetPreferenceScope: GetPreferenceScope::<Identity, Impl, OFFSET>,
            SetPreference: SetPreference::<Identity, Impl, OFFSET>,
            DeletePreference: DeletePreference::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            GetPolicyScope: GetPolicyScope::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSetting as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareInfo_Impl: Sized {
    fn GetShareItem(&mut self) -> ::windows::core::Result<IOfflineFilesShareItem>;
    fn GetShareCachingMode(&mut self) -> ::windows::core::Result<OFFLINEFILES_CACHING_MODE>;
    fn IsShareDfsJunction(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesShareInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>() -> IOfflineFilesShareInfo_Vtbl {
        unsafe extern "system" fn GetShareItem<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshareitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetShareItem() {
                ::core::result::Result::Ok(ok__) => {
                    *ppshareitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShareCachingMode<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetShareCachingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pcachingmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShareDfsJunction<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsShareDfsJunction() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisdfsjunction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetShareItem: GetShareItem::<Identity, Impl, OFFSET>,
            GetShareCachingMode: GetShareCachingMode::<Identity, Impl, OFFSET>,
            IsShareDfsJunction: IsShareDfsJunction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesShareInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareItem_Impl: Sized + IOfflineFilesItem_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesShareItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareItem_Impl, const OFFSET: isize>() -> IOfflineFilesShareItem_Vtbl {
        Self { base: IOfflineFilesItem_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesShareItem as ::windows::core::Interface>::IID || iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSimpleProgress_Impl: Sized + IOfflineFilesProgress_Impl {
    fn ItemBegin(&mut self, pszfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn ItemResult(&mut self, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSimpleProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>() -> IOfflineFilesSimpleProgress_Vtbl {
        unsafe extern "system" fn ItemBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemBegin(::core::mem::transmute_copy(&pszfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemResult<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSimpleProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemResult(::core::mem::transmute_copy(&pszfile), ::core::mem::transmute_copy(&hrresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesProgress_Vtbl::new::<Identity, Impl, OFFSET>(),
            ItemBegin: ItemBegin::<Identity, Impl, OFFSET>,
            ItemResult: ItemResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSimpleProgress as ::windows::core::Interface>::IID || iid == &<IOfflineFilesProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspend_Impl: Sized {
    fn SuspendRoot(&mut self, bsuspend: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSuspend_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspend_Impl, const OFFSET: isize>() -> IOfflineFilesSuspend_Vtbl {
        unsafe extern "system" fn SuspendRoot<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspend_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SuspendRoot(::core::mem::transmute_copy(&bsuspend)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SuspendRoot: SuspendRoot::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSuspend as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspendInfo_Impl: Sized {
    fn IsSuspended(&mut self, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSuspendInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendInfo_Impl, const OFFSET: isize>() -> IOfflineFilesSuspendInfo_Vtbl {
        unsafe extern "system" fn IsSuspended<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSuspended(::core::mem::transmute_copy(&pbsuspended), ::core::mem::transmute_copy(&pbsuspendedroot)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsSuspended: IsSuspended::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSuspendInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncConflictHandler_Impl: Sized {
    fn ResolveConflict(&mut self, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncConflictHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: isize>() -> IOfflineFilesSyncConflictHandler_Vtbl {
        unsafe extern "system" fn ResolveConflict<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncConflictHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveConflict(::core::mem::transmute_copy(&pszpath), ::core::mem::transmute_copy(&fstateknown), ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&fchangedetails), ::core::mem::transmute_copy(&pconflictresolution), ::core::mem::transmute_copy(&ppsznewname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ResolveConflict: ResolveConflict::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncConflictHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesSyncErrorInfo_Impl: Sized + IOfflineFilesErrorInfo_Impl {
    fn GetSyncOperation(&mut self) -> ::windows::core::Result<OFFLINEFILES_SYNC_OPERATION>;
    fn GetItemChangeFlags(&mut self) -> ::windows::core::Result<u32>;
    fn InfoEnumerated(&mut self, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn InfoAvailable(&mut self, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocalInfo(&mut self) -> ::windows::core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetRemoteInfo(&mut self) -> ::windows::core::Result<IOfflineFilesSyncErrorItemInfo>;
    fn GetOriginalInfo(&mut self) -> ::windows::core::Result<IOfflineFilesSyncErrorItemInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOfflineFilesSyncErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>() -> IOfflineFilesSyncErrorInfo_Vtbl {
        unsafe extern "system" fn GetSyncOperation<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *psyncop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemChangeFlags<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitemchangeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemChangeFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwitemchangeflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoEnumerated<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InfoEnumerated(::core::mem::transmute_copy(&pblocalenumerated), ::core::mem::transmute_copy(&pbremoteenumerated), ::core::mem::transmute_copy(&pboriginalenumerated)).into()
        }
        unsafe extern "system" fn InfoAvailable<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InfoAvailable(::core::mem::transmute_copy(&pblocalinfo), ::core::mem::transmute_copy(&pbremoteinfo), ::core::mem::transmute_copy(&pboriginalinfo)).into()
        }
        unsafe extern "system" fn GetLocalInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocalInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRemoteInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalInfo<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOriginalInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesErrorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncOperation: GetSyncOperation::<Identity, Impl, OFFSET>,
            GetItemChangeFlags: GetItemChangeFlags::<Identity, Impl, OFFSET>,
            InfoEnumerated: InfoEnumerated::<Identity, Impl, OFFSET>,
            InfoAvailable: InfoAvailable::<Identity, Impl, OFFSET>,
            GetLocalInfo: GetLocalInfo::<Identity, Impl, OFFSET>,
            GetRemoteInfo: GetRemoteInfo::<Identity, Impl, OFFSET>,
            GetOriginalInfo: GetOriginalInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorInfo as ::windows::core::Interface>::IID || iid == &<IOfflineFilesErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncErrorItemInfo_Impl: Sized {
    fn GetFileAttributes(&mut self) -> ::windows::core::Result<u32>;
    fn GetFileTimes(&mut self, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncErrorItemInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>() -> IOfflineFilesSyncErrorItemInfo_Vtbl {
        unsafe extern "system" fn GetFileAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileTimes<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFileTimes(::core::mem::transmute_copy(&pftlastwrite), ::core::mem::transmute_copy(&pftchange)).into()
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *psize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFileAttributes: GetFileAttributes::<Identity, Impl, OFFSET>,
            GetFileTimes: GetFileTimes::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorItemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncProgress_Impl: Sized + IOfflineFilesProgress_Impl {
    fn SyncItemBegin(&mut self, pszfile: super::super::Foundation::PWSTR) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
    fn SyncItemResult(&mut self, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, perrorinfo: &::core::option::Option<IOfflineFilesSyncErrorInfo>) -> ::windows::core::Result<OFFLINEFILES_OP_RESPONSE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncProgress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>() -> IOfflineFilesSyncProgress_Vtbl {
        unsafe extern "system" fn SyncItemBegin<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SyncItemBegin(::core::mem::transmute_copy(&pszfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncItemResult<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncProgress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, perrorinfo: ::windows::core::RawPtr, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SyncItemResult(::core::mem::transmute_copy(&pszfile), ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute(&perrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *presponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IOfflineFilesProgress_Vtbl::new::<Identity, Impl, OFFSET>(),
            SyncItemBegin: SyncItemBegin::<Identity, Impl, OFFSET>,
            SyncItemResult: SyncItemResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncProgress as ::windows::core::Interface>::IID || iid == &<IOfflineFilesProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesTransparentCacheInfo_Impl: Sized {
    fn IsTransparentlyCached(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesTransparentCacheInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: isize>() -> IOfflineFilesTransparentCacheInfo_Vtbl {
        unsafe extern "system" fn IsTransparentlyCached<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesTransparentCacheInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTransparentlyCached() {
                ::core::result::Result::Ok(ok__) => {
                    *pbtransparentlycached = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsTransparentlyCached: IsTransparentlyCached::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesTransparentCacheInfo as ::windows::core::Interface>::IID
    }
}

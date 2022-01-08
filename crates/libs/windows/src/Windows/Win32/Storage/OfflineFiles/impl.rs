pub trait IEnumOfflineFilesItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumOfflineFilesItems {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IEnumOfflineFilesItems";
}
impl IEnumOfflineFilesItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>() -> IEnumOfflineFilesItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumOfflineFilesItems>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumOfflineFilesSettingsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumOfflineFilesSettings {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IEnumOfflineFilesSettings";
}
impl IEnumOfflineFilesSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>() -> IEnumOfflineFilesSettingsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumOfflineFilesSettings>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesCacheImpl: Sized {
    fn Synchronize();
    fn DeleteItems();
    fn DeleteItemsForUser();
    fn Pin();
    fn Unpin();
    fn GetEncryptionStatus();
    fn Encrypt();
    fn FindItem();
    fn FindItemEx();
    fn RenameItem();
    fn GetLocation();
    fn GetDiskSpaceInformation();
    fn SetDiskSpaceLimits();
    fn ProcessAdminPinPolicy();
    fn GetSettingObject();
    fn EnumSettingObjects();
    fn IsPathCacheable();
}
impl ::windows::core::RuntimeName for IOfflineFilesCache {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesCache";
}
impl IOfflineFilesCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCacheImpl, const OFFSET: isize>() -> IOfflineFilesCacheVtbl {
        unsafe extern "system" fn Synchronize<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::windows::core::RawPtr, piprogress: ::windows::core::RawPtr, psyncid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Synchronize(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&rgpszpaths as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cpaths,
                &*(&basync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwsynccontrol,
                &*(&pisyncconflicthandler as *const <IOfflineFilesSyncConflictHandler as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncConflictHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&piprogress as *const <IOfflineFilesSyncProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncProgress as ::windows::core::DefaultType>::DefaultType),
                &*(&psyncid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItems<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteItems(
                &*(&rgpszpaths as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cpaths,
                dwflags,
                &*(&basync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&piprogress as *const <IOfflineFilesSimpleProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSimpleProgress as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItemsForUser<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteItemsForUser(
                &*(&pszuser as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&rgpszpaths as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cpaths,
                dwflags,
                &*(&basync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&piprogress as *const <IOfflineFilesSimpleProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSimpleProgress as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pin<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pin(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&rgpszpaths as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cpaths,
                &*(&bdeep as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&basync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwpincontrolflags,
                &*(&piprogress as *const <IOfflineFilesSyncProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncProgress as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpin<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unpin(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&rgpszpaths as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cpaths,
                &*(&bdeep as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&basync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwpincontrolflags,
                &*(&piprogress as *const <IOfflineFilesSyncProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncProgress as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncryptionStatus<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncryptionStatus(::core::mem::transmute_copy(&pbencrypted), ::core::mem::transmute_copy(&pbpartial)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Encrypt<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Encrypt(
                &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&bencrypt as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwencryptioncontrolflags,
                &*(&basync as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&piprogress as *const <IOfflineFilesSyncProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncProgress as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItem<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItem(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwqueryflags, ::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindItemEx<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemEx(
                &*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pincludefilefilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                &*(&pincludedirfilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                &*(&pexcludefilefilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                &*(&pexcludedirfilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                dwqueryflags,
                ::core::mem::transmute_copy(&ppitem),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameItem<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameItem(
                &*(&pszpathoriginal as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszpathnew as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&breplaceifexists as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocation<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocation(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiskSpaceInformation(::core::mem::transmute_copy(&pcbvolumetotal), ::core::mem::transmute_copy(&pcblimit), ::core::mem::transmute_copy(&pcbused), ::core::mem::transmute_copy(&pcbunpinnedlimit), ::core::mem::transmute_copy(&pcbunpinnedused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDiskSpaceLimits(cblimit, cbunpinnedlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinprogress: ::windows::core::RawPtr, punpinprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessAdminPinPolicy(&*(&ppinprogress as *const <IOfflineFilesSyncProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncProgress as ::windows::core::DefaultType>::DefaultType), &*(&punpinprogress as *const <IOfflineFilesSyncProgress as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncProgress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSettingObject<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsettingname: super::super::Foundation::PWSTR, ppsetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSettingObject(&*(&pszsettingname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsetting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumSettingObjects<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumSettingObjects(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPathCacheable<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPathCacheable(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbcacheable), ::core::mem::transmute_copy(&psharecachingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOfflineFilesCache>,
            ::windows::core::GetTrustLevel,
            Synchronize::<Impl, OFFSET>,
            DeleteItems::<Impl, OFFSET>,
            DeleteItemsForUser::<Impl, OFFSET>,
            Pin::<Impl, OFFSET>,
            Unpin::<Impl, OFFSET>,
            GetEncryptionStatus::<Impl, OFFSET>,
            Encrypt::<Impl, OFFSET>,
            FindItem::<Impl, OFFSET>,
            FindItemEx::<Impl, OFFSET>,
            RenameItem::<Impl, OFFSET>,
            GetLocation::<Impl, OFFSET>,
            GetDiskSpaceInformation::<Impl, OFFSET>,
            SetDiskSpaceLimits::<Impl, OFFSET>,
            ProcessAdminPinPolicy::<Impl, OFFSET>,
            GetSettingObject::<Impl, OFFSET>,
            EnumSettingObjects::<Impl, OFFSET>,
            IsPathCacheable::<Impl, OFFSET>,
        )
    }
}
pub trait IOfflineFilesCache2Impl: Sized + IOfflineFilesCacheImpl {
    fn RenameItemEx();
}
impl ::windows::core::RuntimeName for IOfflineFilesCache2 {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesCache2";
}
impl IOfflineFilesCache2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache2Impl, const OFFSET: isize>() -> IOfflineFilesCache2Vtbl {
        unsafe extern "system" fn RenameItemEx<Impl: IOfflineFilesCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameItemEx(
                &*(&pszpathoriginal as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszpathnew as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&breplaceifexists as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesCache2>, ::windows::core::GetTrustLevel, RenameItemEx::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesChangeInfoImpl: Sized {
    fn IsDirty();
    fn IsDeletedOffline();
    fn IsCreatedOffline();
    fn IsLocallyModifiedData();
    fn IsLocallyModifiedAttributes();
    fn IsLocallyModifiedTime();
}
impl ::windows::core::RuntimeName for IOfflineFilesChangeInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesChangeInfo";
}
impl IOfflineFilesChangeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>() -> IOfflineFilesChangeInfoVtbl {
        unsafe extern "system" fn IsDirty<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty(::core::mem::transmute_copy(&pbdirty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeletedOffline<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeletedOffline(::core::mem::transmute_copy(&pbdeletedoffline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCreatedOffline<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCreatedOffline(::core::mem::transmute_copy(&pbcreatedoffline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedData<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocallyModifiedData(::core::mem::transmute_copy(&pblocallymodifieddata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocallyModifiedAttributes(::core::mem::transmute_copy(&pblocallymodifiedattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocallyModifiedTime(::core::mem::transmute_copy(&pblocallymodifiedtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOfflineFilesChangeInfo>,
            ::windows::core::GetTrustLevel,
            IsDirty::<Impl, OFFSET>,
            IsDeletedOffline::<Impl, OFFSET>,
            IsCreatedOffline::<Impl, OFFSET>,
            IsLocallyModifiedData::<Impl, OFFSET>,
            IsLocallyModifiedAttributes::<Impl, OFFSET>,
            IsLocallyModifiedTime::<Impl, OFFSET>,
        )
    }
}
pub trait IOfflineFilesConnectionInfoImpl: Sized {
    fn GetConnectState();
    fn SetConnectState();
    fn TransitionOnline();
    fn TransitionOffline();
}
impl ::windows::core::RuntimeName for IOfflineFilesConnectionInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesConnectionInfo";
}
impl IOfflineFilesConnectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>() -> IOfflineFilesConnectionInfoVtbl {
        unsafe extern "system" fn GetConnectState<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectState(::core::mem::transmute_copy(&pconnectstate), ::core::mem::transmute_copy(&pofflinereason)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectState<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectState(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, connectstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitionOnline<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitionOnline(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitionOffline<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitionOffline(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&bforceopenfilesclosed as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbopenfilespreventedtransition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesConnectionInfo>, ::windows::core::GetTrustLevel, GetConnectState::<Impl, OFFSET>, SetConnectState::<Impl, OFFSET>, TransitionOnline::<Impl, OFFSET>, TransitionOffline::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesDirectoryItemImpl: Sized + IOfflineFilesItemImpl {}
impl ::windows::core::RuntimeName for IOfflineFilesDirectoryItem {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesDirectoryItem";
}
impl IOfflineFilesDirectoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirectoryItemImpl, const OFFSET: isize>() -> IOfflineFilesDirectoryItemVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesDirectoryItem>, ::windows::core::GetTrustLevel)
    }
}
pub trait IOfflineFilesDirtyInfoImpl: Sized {
    fn LocalDirtyByteCount();
    fn RemoteDirtyByteCount();
}
impl ::windows::core::RuntimeName for IOfflineFilesDirtyInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesDirtyInfo";
}
impl IOfflineFilesDirtyInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>() -> IOfflineFilesDirtyInfoVtbl {
        unsafe extern "system" fn LocalDirtyByteCount<Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalDirtyByteCount(::core::mem::transmute_copy(&pdirtybytecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteDirtyByteCount(::core::mem::transmute_copy(&pdirtybytecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesDirtyInfo>, ::windows::core::GetTrustLevel, LocalDirtyByteCount::<Impl, OFFSET>, RemoteDirtyByteCount::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesErrorInfoImpl: Sized {
    fn GetRawData();
    fn GetDescription();
}
impl ::windows::core::RuntimeName for IOfflineFilesErrorInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesErrorInfo";
}
impl IOfflineFilesErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>() -> IOfflineFilesErrorInfoVtbl {
        unsafe extern "system" fn GetRawData<Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRawData(::core::mem::transmute_copy(&ppblob)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&ppszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesErrorInfo>, ::windows::core::GetTrustLevel, GetRawData::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesEventsImpl: Sized {
    fn CacheMoved();
    fn CacheIsFull();
    fn CacheIsCorrupted();
    fn Enabled();
    fn EncryptionChanged();
    fn SyncBegin();
    fn SyncFileResult();
    fn SyncConflictRecAdded();
    fn SyncConflictRecUpdated();
    fn SyncConflictRecRemoved();
    fn SyncEnd();
    fn NetTransportArrived();
    fn NoNetTransports();
    fn ItemDisconnected();
    fn ItemReconnected();
    fn ItemAvailableOffline();
    fn ItemNotAvailableOffline();
    fn ItemPinned();
    fn ItemNotPinned();
    fn ItemModified();
    fn ItemAddedToCache();
    fn ItemDeletedFromCache();
    fn ItemRenamed();
    fn DataLost();
    fn Ping();
}
impl ::windows::core::RuntimeName for IOfflineFilesEvents {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesEvents";
}
impl IOfflineFilesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsImpl, const OFFSET: isize>() -> IOfflineFilesEventsVtbl {
        unsafe extern "system" fn CacheMoved<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheMoved(&*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&psznewpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheIsFull<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheIsFull() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheIsCorrupted<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheIsCorrupted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled(&*(&benabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptionChanged<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncryptionChanged(
                &*(&bwasencrypted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bwaspartial as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bisencrypted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bispartial as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncBegin<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncBegin(&*(&rsyncid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncFileResult<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: &::windows::core::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncFileResult(&*(&rsyncid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncConflictRecAdded<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncConflictRecAdded(&*(&pszconflictpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pftconflictdatetime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType), conflictsyncstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncConflictRecUpdated(&*(&pszconflictpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pftconflictdatetime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType), conflictsyncstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncConflictRecRemoved(&*(&pszconflictpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pftconflictdatetime as *const <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::FILETIME as ::windows::core::DefaultType>::DefaultType), conflictsyncstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncEnd<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: &::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncEnd(&*(&rsyncid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetTransportArrived<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetTransportArrived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoNetTransports<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoNetTransports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemDisconnected<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemDisconnected(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemReconnected<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemReconnected(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemAvailableOffline<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemAvailableOffline(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemNotAvailableOffline(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemPinned<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemPinned(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemNotPinned<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemNotPinned(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemModified<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemModified(
                &*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                itemtype,
                &*(&bmodifieddata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bmodifiedattributes as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemAddedToCache<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemAddedToCache(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemDeletedFromCache<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemDeletedFromCache(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemRenamed<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemRenamed(&*(&pszoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&psznewpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), itemtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataLost<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataLost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ping<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOfflineFilesEvents>,
            ::windows::core::GetTrustLevel,
            CacheMoved::<Impl, OFFSET>,
            CacheIsFull::<Impl, OFFSET>,
            CacheIsCorrupted::<Impl, OFFSET>,
            Enabled::<Impl, OFFSET>,
            EncryptionChanged::<Impl, OFFSET>,
            SyncBegin::<Impl, OFFSET>,
            SyncFileResult::<Impl, OFFSET>,
            SyncConflictRecAdded::<Impl, OFFSET>,
            SyncConflictRecUpdated::<Impl, OFFSET>,
            SyncConflictRecRemoved::<Impl, OFFSET>,
            SyncEnd::<Impl, OFFSET>,
            NetTransportArrived::<Impl, OFFSET>,
            NoNetTransports::<Impl, OFFSET>,
            ItemDisconnected::<Impl, OFFSET>,
            ItemReconnected::<Impl, OFFSET>,
            ItemAvailableOffline::<Impl, OFFSET>,
            ItemNotAvailableOffline::<Impl, OFFSET>,
            ItemPinned::<Impl, OFFSET>,
            ItemNotPinned::<Impl, OFFSET>,
            ItemModified::<Impl, OFFSET>,
            ItemAddedToCache::<Impl, OFFSET>,
            ItemDeletedFromCache::<Impl, OFFSET>,
            ItemRenamed::<Impl, OFFSET>,
            DataLost::<Impl, OFFSET>,
            Ping::<Impl, OFFSET>,
        )
    }
}
pub trait IOfflineFilesEvents2Impl: Sized + IOfflineFilesEventsImpl {
    fn ItemReconnectBegin();
    fn ItemReconnectEnd();
    fn CacheEvictBegin();
    fn CacheEvictEnd();
    fn BackgroundSyncBegin();
    fn BackgroundSyncEnd();
    fn PolicyChangeDetected();
    fn PreferenceChangeDetected();
    fn SettingsChangesApplied();
}
impl ::windows::core::RuntimeName for IOfflineFilesEvents2 {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesEvents2";
}
impl IOfflineFilesEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>() -> IOfflineFilesEvents2Vtbl {
        unsafe extern "system" fn ItemReconnectBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemReconnectBegin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemReconnectEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemReconnectEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheEvictBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheEvictBegin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheEvictEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheEvictEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundSyncBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSyncBegin(dwsynccontrolflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundSyncEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundSyncEnd(dwsynccontrolflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyChangeDetected<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyChangeDetected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferenceChangeDetected<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferenceChangeDetected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettingsChangesApplied<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SettingsChangesApplied() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOfflineFilesEvents2>,
            ::windows::core::GetTrustLevel,
            ItemReconnectBegin::<Impl, OFFSET>,
            ItemReconnectEnd::<Impl, OFFSET>,
            CacheEvictBegin::<Impl, OFFSET>,
            CacheEvictEnd::<Impl, OFFSET>,
            BackgroundSyncBegin::<Impl, OFFSET>,
            BackgroundSyncEnd::<Impl, OFFSET>,
            PolicyChangeDetected::<Impl, OFFSET>,
            PreferenceChangeDetected::<Impl, OFFSET>,
            SettingsChangesApplied::<Impl, OFFSET>,
        )
    }
}
pub trait IOfflineFilesEvents3Impl: Sized + IOfflineFilesEvents2Impl + IOfflineFilesEventsImpl {
    fn TransparentCacheItemNotify();
    fn PrefetchFileBegin();
    fn PrefetchFileEnd();
}
impl ::windows::core::RuntimeName for IOfflineFilesEvents3 {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesEvents3";
}
impl IOfflineFilesEvents3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>() -> IOfflineFilesEvents3Vtbl {
        unsafe extern "system" fn TransparentCacheItemNotify<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransparentCacheItemNotify(
                &*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                eventtype,
                itemtype,
                &*(&bmodifieddata as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&bmodifiedattributes as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pzsoldpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchFileBegin<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchFileBegin(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchFileEnd<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchFileEnd(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesEvents3>, ::windows::core::GetTrustLevel, TransparentCacheItemNotify::<Impl, OFFSET>, PrefetchFileBegin::<Impl, OFFSET>, PrefetchFileEnd::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesEvents4Impl: Sized + IOfflineFilesEvents3Impl + IOfflineFilesEvents2Impl + IOfflineFilesEventsImpl {
    fn PrefetchCloseHandleBegin();
    fn PrefetchCloseHandleEnd();
}
impl ::windows::core::RuntimeName for IOfflineFilesEvents4 {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesEvents4";
}
impl IOfflineFilesEvents4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>() -> IOfflineFilesEvents4Vtbl {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchCloseHandleBegin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefetchCloseHandleEnd(dwclosedhandlecount, dwopenhandlecount, hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesEvents4>, ::windows::core::GetTrustLevel, PrefetchCloseHandleBegin::<Impl, OFFSET>, PrefetchCloseHandleEnd::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesEventsFilterImpl: Sized {
    fn GetPathFilter();
    fn GetIncludedEvents();
    fn GetExcludedEvents();
}
impl ::windows::core::RuntimeName for IOfflineFilesEventsFilter {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesEventsFilter";
}
impl IOfflineFilesEventsFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>() -> IOfflineFilesEventsFilterVtbl {
        unsafe extern "system" fn GetPathFilter<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathFilter(::core::mem::transmute_copy(&ppszfilter), ::core::mem::transmute_copy(&pmatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncludedEvents<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludedEvents(celements, ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExcludedEvents<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExcludedEvents(celements, ::core::mem::transmute_copy(&prgevents), ::core::mem::transmute_copy(&pcevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesEventsFilter>, ::windows::core::GetTrustLevel, GetPathFilter::<Impl, OFFSET>, GetIncludedEvents::<Impl, OFFSET>, GetExcludedEvents::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesFileItemImpl: Sized + IOfflineFilesItemImpl {
    fn IsSparse();
    fn IsEncrypted();
}
impl ::windows::core::RuntimeName for IOfflineFilesFileItem {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesFileItem";
}
impl IOfflineFilesFileItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>() -> IOfflineFilesFileItemVtbl {
        unsafe extern "system" fn IsSparse<Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSparse(::core::mem::transmute_copy(&pbissparse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEncrypted<Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEncrypted(::core::mem::transmute_copy(&pbisencrypted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesFileItem>, ::windows::core::GetTrustLevel, IsSparse::<Impl, OFFSET>, IsEncrypted::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesFileSysInfoImpl: Sized {
    fn GetAttributes();
    fn GetTimes();
    fn GetFileSize();
}
impl ::windows::core::RuntimeName for IOfflineFilesFileSysInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesFileSysInfo";
}
impl IOfflineFilesFileSysInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>() -> IOfflineFilesFileSysInfoVtbl {
        unsafe extern "system" fn GetAttributes<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributes(copy, ::core::mem::transmute_copy(&pdwattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimes<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimes(copy, ::core::mem::transmute_copy(&pftcreationtime), ::core::mem::transmute_copy(&pftlastwritetime), ::core::mem::transmute_copy(&pftchangetime), ::core::mem::transmute_copy(&pftlastaccesstime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize(copy, ::core::mem::transmute_copy(&psize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesFileSysInfo>, ::windows::core::GetTrustLevel, GetAttributes::<Impl, OFFSET>, GetTimes::<Impl, OFFSET>, GetFileSize::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesGhostInfoImpl: Sized {
    fn IsGhosted();
}
impl ::windows::core::RuntimeName for IOfflineFilesGhostInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesGhostInfo";
}
impl IOfflineFilesGhostInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesGhostInfoImpl, const OFFSET: isize>() -> IOfflineFilesGhostInfoVtbl {
        unsafe extern "system" fn IsGhosted<Impl: IOfflineFilesGhostInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGhosted(::core::mem::transmute_copy(&pbghosted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesGhostInfo>, ::windows::core::GetTrustLevel, IsGhosted::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesItemImpl: Sized {
    fn GetItemType();
    fn GetPath();
    fn GetParentItem();
    fn Refresh();
    fn IsMarkedForDeletion();
}
impl ::windows::core::RuntimeName for IOfflineFilesItem {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesItem";
}
impl IOfflineFilesItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemImpl, const OFFSET: isize>() -> IOfflineFilesItemVtbl {
        unsafe extern "system" fn GetItemType<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemType(::core::mem::transmute_copy(&pitemtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentItem<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentItem(::core::mem::transmute_copy(&ppitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh(dwqueryflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMarkedForDeletion<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMarkedForDeletion(::core::mem::transmute_copy(&pbmarkedfordeletion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesItem>, ::windows::core::GetTrustLevel, GetItemType::<Impl, OFFSET>, GetPath::<Impl, OFFSET>, GetParentItem::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, IsMarkedForDeletion::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesItemContainerImpl: Sized {
    fn EnumItems();
    fn EnumItemsEx();
}
impl ::windows::core::RuntimeName for IOfflineFilesItemContainer {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesItemContainer";
}
impl IOfflineFilesItemContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>() -> IOfflineFilesItemContainerVtbl {
        unsafe extern "system" fn EnumItems<Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItems(dwqueryflags, ::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumItemsEx<Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumItemsEx(
                &*(&pincludefilefilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                &*(&pincludedirfilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                &*(&pexcludefilefilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                &*(&pexcludedirfilter as *const <IOfflineFilesItemFilter as ::windows::core::Abi>::Abi as *const <IOfflineFilesItemFilter as ::windows::core::DefaultType>::DefaultType),
                dwenumflags,
                dwqueryflags,
                ::core::mem::transmute_copy(&ppenum),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesItemContainer>, ::windows::core::GetTrustLevel, EnumItems::<Impl, OFFSET>, EnumItemsEx::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesItemFilterImpl: Sized {
    fn GetFilterFlags();
    fn GetTimeFilter();
    fn GetPatternFilter();
}
impl ::windows::core::RuntimeName for IOfflineFilesItemFilter {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesItemFilter";
}
impl IOfflineFilesItemFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>() -> IOfflineFilesItemFilterVtbl {
        unsafe extern "system" fn GetFilterFlags<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilterFlags(::core::mem::transmute_copy(&pullflags), ::core::mem::transmute_copy(&pullmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeFilter<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimeFilter(::core::mem::transmute_copy(&pfttime), ::core::mem::transmute_copy(&pbevaltimeofday), ::core::mem::transmute_copy(&ptimetype), ::core::mem::transmute_copy(&pcompare)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternFilter<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternFilter(::core::mem::transmute_copy(&pszpattern), cchpattern) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesItemFilter>, ::windows::core::GetTrustLevel, GetFilterFlags::<Impl, OFFSET>, GetTimeFilter::<Impl, OFFSET>, GetPatternFilter::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesPinInfoImpl: Sized {
    fn IsPinned();
    fn IsPinnedForUser();
    fn IsPinnedForUserByPolicy();
    fn IsPinnedForComputer();
    fn IsPinnedForFolderRedirection();
}
impl ::windows::core::RuntimeName for IOfflineFilesPinInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesPinInfo";
}
impl IOfflineFilesPinInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>() -> IOfflineFilesPinInfoVtbl {
        unsafe extern "system" fn IsPinned<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinned(::core::mem::transmute_copy(&pbpinned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForUser<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinnedForUser(::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinnedForUserByPolicy(::core::mem::transmute_copy(&pbpinnedforuser), ::core::mem::transmute_copy(&pbinherit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForComputer<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinnedForComputer(::core::mem::transmute_copy(&pbpinnedforcomputer), ::core::mem::transmute_copy(&pbinherit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPinnedForFolderRedirection(::core::mem::transmute_copy(&pbpinnedforfolderredirection), ::core::mem::transmute_copy(&pbinherit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesPinInfo>, ::windows::core::GetTrustLevel, IsPinned::<Impl, OFFSET>, IsPinnedForUser::<Impl, OFFSET>, IsPinnedForUserByPolicy::<Impl, OFFSET>, IsPinnedForComputer::<Impl, OFFSET>, IsPinnedForFolderRedirection::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesPinInfo2Impl: Sized + IOfflineFilesPinInfoImpl {
    fn IsPartlyPinned();
}
impl ::windows::core::RuntimeName for IOfflineFilesPinInfo2 {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesPinInfo2";
}
impl IOfflineFilesPinInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo2Impl, const OFFSET: isize>() -> IOfflineFilesPinInfo2Vtbl {
        unsafe extern "system" fn IsPartlyPinned<Impl: IOfflineFilesPinInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPartlyPinned(::core::mem::transmute_copy(&pbpartlypinned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesPinInfo2>, ::windows::core::GetTrustLevel, IsPartlyPinned::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesProgressImpl: Sized {
    fn Begin();
    fn QueryAbort();
    fn End();
}
impl ::windows::core::RuntimeName for IOfflineFilesProgress {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesProgress";
}
impl IOfflineFilesProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgressImpl, const OFFSET: isize>() -> IOfflineFilesProgressVtbl {
        unsafe extern "system" fn Begin<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Begin(::core::mem::transmute_copy(&pbabort)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAbort<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAbort(::core::mem::transmute_copy(&pbabort)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn End<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).End(hrresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesProgress>, ::windows::core::GetTrustLevel, Begin::<Impl, OFFSET>, QueryAbort::<Impl, OFFSET>, End::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesServerItemImpl: Sized + IOfflineFilesItemImpl {}
impl ::windows::core::RuntimeName for IOfflineFilesServerItem {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesServerItem";
}
impl IOfflineFilesServerItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesServerItemImpl, const OFFSET: isize>() -> IOfflineFilesServerItemVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesServerItem>, ::windows::core::GetTrustLevel)
    }
}
pub trait IOfflineFilesSettingImpl: Sized {
    fn GetName();
    fn GetValueType();
    fn GetPreference();
    fn GetPreferenceScope();
    fn SetPreference();
    fn DeletePreference();
    fn GetPolicy();
    fn GetPolicyScope();
    fn GetValue();
}
impl ::windows::core::RuntimeName for IOfflineFilesSetting {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSetting";
}
impl IOfflineFilesSettingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSettingImpl, const OFFSET: isize>() -> IOfflineFilesSettingVtbl {
        unsafe extern "system" fn GetName<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&ppszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueType<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValueType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreference(::core::mem::transmute_copy(&pvarvalue), dwscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreferenceScope<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferenceScope(::core::mem::transmute_copy(&pdwscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPreference(&*(&pvarvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), dwscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePreference(dwscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicy<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicy(::core::mem::transmute_copy(&pvarvalue), dwscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyScope<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPolicyScope(::core::mem::transmute_copy(&pdwscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pbsetbypolicy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOfflineFilesSetting>,
            ::windows::core::GetTrustLevel,
            GetName::<Impl, OFFSET>,
            GetValueType::<Impl, OFFSET>,
            GetPreference::<Impl, OFFSET>,
            GetPreferenceScope::<Impl, OFFSET>,
            SetPreference::<Impl, OFFSET>,
            DeletePreference::<Impl, OFFSET>,
            GetPolicy::<Impl, OFFSET>,
            GetPolicyScope::<Impl, OFFSET>,
            GetValue::<Impl, OFFSET>,
        )
    }
}
pub trait IOfflineFilesShareInfoImpl: Sized {
    fn GetShareItem();
    fn GetShareCachingMode();
    fn IsShareDfsJunction();
}
impl ::windows::core::RuntimeName for IOfflineFilesShareInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesShareInfo";
}
impl IOfflineFilesShareInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>() -> IOfflineFilesShareInfoVtbl {
        unsafe extern "system" fn GetShareItem<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshareitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShareItem(::core::mem::transmute_copy(&ppshareitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShareCachingMode<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShareCachingMode(::core::mem::transmute_copy(&pcachingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShareDfsJunction<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShareDfsJunction(::core::mem::transmute_copy(&pbisdfsjunction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesShareInfo>, ::windows::core::GetTrustLevel, GetShareItem::<Impl, OFFSET>, GetShareCachingMode::<Impl, OFFSET>, IsShareDfsJunction::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesShareItemImpl: Sized + IOfflineFilesItemImpl {}
impl ::windows::core::RuntimeName for IOfflineFilesShareItem {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesShareItem";
}
impl IOfflineFilesShareItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareItemImpl, const OFFSET: isize>() -> IOfflineFilesShareItemVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesShareItem>, ::windows::core::GetTrustLevel)
    }
}
pub trait IOfflineFilesSimpleProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn ItemBegin();
    fn ItemResult();
}
impl ::windows::core::RuntimeName for IOfflineFilesSimpleProgress {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSimpleProgress";
}
impl IOfflineFilesSimpleProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>() -> IOfflineFilesSimpleProgressVtbl {
        unsafe extern "system" fn ItemBegin<Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemBegin(&*(&pszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemResult<Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemResult(&*(&pszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrresult, ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesSimpleProgress>, ::windows::core::GetTrustLevel, ItemBegin::<Impl, OFFSET>, ItemResult::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesSuspendImpl: Sized {
    fn SuspendRoot();
}
impl ::windows::core::RuntimeName for IOfflineFilesSuspend {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSuspend";
}
impl IOfflineFilesSuspendVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendImpl, const OFFSET: isize>() -> IOfflineFilesSuspendVtbl {
        unsafe extern "system" fn SuspendRoot<Impl: IOfflineFilesSuspendImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendRoot(&*(&bsuspend as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesSuspend>, ::windows::core::GetTrustLevel, SuspendRoot::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesSuspendInfoImpl: Sized {
    fn IsSuspended();
}
impl ::windows::core::RuntimeName for IOfflineFilesSuspendInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSuspendInfo";
}
impl IOfflineFilesSuspendInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendInfoImpl, const OFFSET: isize>() -> IOfflineFilesSuspendInfoVtbl {
        unsafe extern "system" fn IsSuspended<Impl: IOfflineFilesSuspendInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuspended(::core::mem::transmute_copy(&pbsuspended), ::core::mem::transmute_copy(&pbsuspendedroot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesSuspendInfo>, ::windows::core::GetTrustLevel, IsSuspended::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesSyncConflictHandlerImpl: Sized {
    fn ResolveConflict();
}
impl ::windows::core::RuntimeName for IOfflineFilesSyncConflictHandler {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSyncConflictHandler";
}
impl IOfflineFilesSyncConflictHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncConflictHandlerImpl, const OFFSET: isize>() -> IOfflineFilesSyncConflictHandlerVtbl {
        unsafe extern "system" fn ResolveConflict<Impl: IOfflineFilesSyncConflictHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveConflict(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), fstateknown, state, fchangedetails, ::core::mem::transmute_copy(&pconflictresolution), ::core::mem::transmute_copy(&ppsznewname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesSyncConflictHandler>, ::windows::core::GetTrustLevel, ResolveConflict::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesSyncErrorInfoImpl: Sized + IOfflineFilesErrorInfoImpl {
    fn GetSyncOperation();
    fn GetItemChangeFlags();
    fn InfoEnumerated();
    fn InfoAvailable();
    fn GetLocalInfo();
    fn GetRemoteInfo();
    fn GetOriginalInfo();
}
impl ::windows::core::RuntimeName for IOfflineFilesSyncErrorInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSyncErrorInfo";
}
impl IOfflineFilesSyncErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>() -> IOfflineFilesSyncErrorInfoVtbl {
        unsafe extern "system" fn GetSyncOperation<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncOperation(::core::mem::transmute_copy(&psyncop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemChangeFlags<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitemchangeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemChangeFlags(::core::mem::transmute_copy(&pdwitemchangeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoEnumerated<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfoEnumerated(::core::mem::transmute_copy(&pblocalenumerated), ::core::mem::transmute_copy(&pbremoteenumerated), ::core::mem::transmute_copy(&pboriginalenumerated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfoAvailable<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfoAvailable(::core::mem::transmute_copy(&pblocalinfo), ::core::mem::transmute_copy(&pbremoteinfo), ::core::mem::transmute_copy(&pboriginalinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalInfo(::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteInfo(::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalInfo(::core::mem::transmute_copy(&ppinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IOfflineFilesSyncErrorInfo>,
            ::windows::core::GetTrustLevel,
            GetSyncOperation::<Impl, OFFSET>,
            GetItemChangeFlags::<Impl, OFFSET>,
            InfoEnumerated::<Impl, OFFSET>,
            InfoAvailable::<Impl, OFFSET>,
            GetLocalInfo::<Impl, OFFSET>,
            GetRemoteInfo::<Impl, OFFSET>,
            GetOriginalInfo::<Impl, OFFSET>,
        )
    }
}
pub trait IOfflineFilesSyncErrorItemInfoImpl: Sized {
    fn GetFileAttributes();
    fn GetFileTimes();
    fn GetFileSize();
}
impl ::windows::core::RuntimeName for IOfflineFilesSyncErrorItemInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSyncErrorItemInfo";
}
impl IOfflineFilesSyncErrorItemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>() -> IOfflineFilesSyncErrorItemInfoVtbl {
        unsafe extern "system" fn GetFileAttributes<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileAttributes(::core::mem::transmute_copy(&pdwattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileTimes<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileTimes(::core::mem::transmute_copy(&pftlastwrite), ::core::mem::transmute_copy(&pftchange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize(::core::mem::transmute_copy(&psize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesSyncErrorItemInfo>, ::windows::core::GetTrustLevel, GetFileAttributes::<Impl, OFFSET>, GetFileTimes::<Impl, OFFSET>, GetFileSize::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesSyncProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn SyncItemBegin();
    fn SyncItemResult();
}
impl ::windows::core::RuntimeName for IOfflineFilesSyncProgress {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesSyncProgress";
}
impl IOfflineFilesSyncProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>() -> IOfflineFilesSyncProgressVtbl {
        unsafe extern "system" fn SyncItemBegin<Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncItemBegin(&*(&pszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncItemResult<Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, perrorinfo: ::windows::core::RawPtr, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncItemResult(&*(&pszfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrresult, &*(&perrorinfo as *const <IOfflineFilesSyncErrorInfo as ::windows::core::Abi>::Abi as *const <IOfflineFilesSyncErrorInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesSyncProgress>, ::windows::core::GetTrustLevel, SyncItemBegin::<Impl, OFFSET>, SyncItemResult::<Impl, OFFSET>)
    }
}
pub trait IOfflineFilesTransparentCacheInfoImpl: Sized {
    fn IsTransparentlyCached();
}
impl ::windows::core::RuntimeName for IOfflineFilesTransparentCacheInfo {
    const NAME: &'static str = "Windows.Win32.Storage.OfflineFiles.IOfflineFilesTransparentCacheInfo";
}
impl IOfflineFilesTransparentCacheInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesTransparentCacheInfoImpl, const OFFSET: isize>() -> IOfflineFilesTransparentCacheInfoVtbl {
        unsafe extern "system" fn IsTransparentlyCached<Impl: IOfflineFilesTransparentCacheInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransparentlyCached(::core::mem::transmute_copy(&pbtransparentlycached)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IOfflineFilesTransparentCacheInfo>, ::windows::core::GetTrustLevel, IsTransparentlyCached::<Impl, OFFSET>)
    }
}

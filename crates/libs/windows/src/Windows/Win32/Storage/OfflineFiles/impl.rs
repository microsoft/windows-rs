pub trait IEnumOfflineFilesItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumOfflineFilesItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOfflineFilesItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOfflineFilesItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOfflineFilesItems as ::windows::core::Interface>::IID
    }
}
pub trait IEnumOfflineFilesSettingsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumOfflineFilesSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumOfflineFilesSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumOfflineFilesSettingsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumOfflineFilesSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumOfflineFilesSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCacheImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesCacheVtbl {
        unsafe extern "system" fn Synchronize<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, basync: super::super::Foundation::BOOL, dwsynccontrol: u32, pisyncconflicthandler: ::windows::core::RawPtr, piprogress: ::windows::core::RawPtr, psyncid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItems<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteItemsForUser<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuser: super::super::Foundation::PWSTR, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, dwflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pin<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unpin<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, rgpszpaths: *const super::super::Foundation::PWSTR, cpaths: u32, bdeep: super::super::Foundation::BOOL, basync: super::super::Foundation::BOOL, dwpincontrolflags: u32, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEncryptionStatus<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Encrypt<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, bencrypt: super::super::Foundation::BOOL, dwencryptioncontrolflags: u32, basync: super::super::Foundation::BOOL, piprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindItem<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindItemEx<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwqueryflags: u32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RenameItem<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocation<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDiskSpaceInformation<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDiskSpaceLimits<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessAdminPinPolicy<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinprogress: ::windows::core::RawPtr, punpinprogress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSettingObject<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsettingname: super::super::Foundation::PWSTR, ppsetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumSettingObjects<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPathCacheable<Impl: IOfflineFilesCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Synchronize::<Impl, IMPL_OFFSET>,
            DeleteItems::<Impl, IMPL_OFFSET>,
            DeleteItemsForUser::<Impl, IMPL_OFFSET>,
            Pin::<Impl, IMPL_OFFSET>,
            Unpin::<Impl, IMPL_OFFSET>,
            GetEncryptionStatus::<Impl, IMPL_OFFSET>,
            Encrypt::<Impl, IMPL_OFFSET>,
            FindItem::<Impl, IMPL_OFFSET>,
            FindItemEx::<Impl, IMPL_OFFSET>,
            RenameItem::<Impl, IMPL_OFFSET>,
            GetLocation::<Impl, IMPL_OFFSET>,
            GetDiskSpaceInformation::<Impl, IMPL_OFFSET>,
            SetDiskSpaceLimits::<Impl, IMPL_OFFSET>,
            ProcessAdminPinPolicy::<Impl, IMPL_OFFSET>,
            GetSettingObject::<Impl, IMPL_OFFSET>,
            EnumSettingObjects::<Impl, IMPL_OFFSET>,
            IsPathCacheable::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesCache as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesCache2Impl: Sized + IOfflineFilesCacheImpl {
    fn RenameItemEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesCache2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesCache2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesCache2Vtbl {
        unsafe extern "system" fn RenameItemEx<Impl: IOfflineFilesCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpathoriginal: super::super::Foundation::PWSTR, pszpathnew: super::super::Foundation::PWSTR, breplaceifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Synchronize::<Impl, IMPL_OFFSET>,
            DeleteItems::<Impl, IMPL_OFFSET>,
            DeleteItemsForUser::<Impl, IMPL_OFFSET>,
            Pin::<Impl, IMPL_OFFSET>,
            Unpin::<Impl, IMPL_OFFSET>,
            GetEncryptionStatus::<Impl, IMPL_OFFSET>,
            Encrypt::<Impl, IMPL_OFFSET>,
            FindItem::<Impl, IMPL_OFFSET>,
            FindItemEx::<Impl, IMPL_OFFSET>,
            RenameItem::<Impl, IMPL_OFFSET>,
            GetLocation::<Impl, IMPL_OFFSET>,
            GetDiskSpaceInformation::<Impl, IMPL_OFFSET>,
            SetDiskSpaceLimits::<Impl, IMPL_OFFSET>,
            ProcessAdminPinPolicy::<Impl, IMPL_OFFSET>,
            GetSettingObject::<Impl, IMPL_OFFSET>,
            EnumSettingObjects::<Impl, IMPL_OFFSET>,
            IsPathCacheable::<Impl, IMPL_OFFSET>,
            RenameItemEx::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesCache2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesChangeInfoImpl: Sized {
    fn IsDirty();
    fn IsDeletedOffline();
    fn IsCreatedOffline();
    fn IsLocallyModifiedData();
    fn IsLocallyModifiedAttributes();
    fn IsLocallyModifiedTime();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesChangeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesChangeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesChangeInfoVtbl {
        unsafe extern "system" fn IsDirty<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdirty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDeletedOffline<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeletedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCreatedOffline<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcreatedoffline: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocallyModifiedData<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifieddata: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocallyModifiedAttributes<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedattributes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLocallyModifiedTime<Impl: IOfflineFilesChangeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocallymodifiedtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, IsDeletedOffline::<Impl, IMPL_OFFSET>, IsCreatedOffline::<Impl, IMPL_OFFSET>, IsLocallyModifiedData::<Impl, IMPL_OFFSET>, IsLocallyModifiedAttributes::<Impl, IMPL_OFFSET>, IsLocallyModifiedTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesChangeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesConnectionInfoImpl: Sized {
    fn GetConnectState();
    fn SetConnectState();
    fn TransitionOnline();
    fn TransitionOffline();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesConnectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesConnectionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesConnectionInfoVtbl {
        unsafe extern "system" fn GetConnectState<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectstate: *mut OFFLINEFILES_CONNECT_STATE, pofflinereason: *mut OFFLINEFILES_OFFLINE_REASON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectState<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, connectstate: OFFLINEFILES_CONNECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransitionOnline<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransitionOffline<Impl: IOfflineFilesConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, bforceopenfilesclosed: super::super::Foundation::BOOL, pbopenfilespreventedtransition: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetConnectState::<Impl, IMPL_OFFSET>, SetConnectState::<Impl, IMPL_OFFSET>, TransitionOnline::<Impl, IMPL_OFFSET>, TransitionOffline::<Impl, IMPL_OFFSET>)
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemType::<Impl, IMPL_OFFSET>, GetPath::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, IsMarkedForDeletion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesDirectoryItem as ::windows::core::Interface>::IID
    }
}
pub trait IOfflineFilesDirtyInfoImpl: Sized {
    fn LocalDirtyByteCount();
    fn RemoteDirtyByteCount();
}
impl IOfflineFilesDirtyInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesDirtyInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesDirtyInfoVtbl {
        unsafe extern "system" fn LocalDirtyByteCount<Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoteDirtyByteCount<Impl: IOfflineFilesDirtyInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtybytecount: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, LocalDirtyByteCount::<Impl, IMPL_OFFSET>, RemoteDirtyByteCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesDirtyInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesErrorInfoImpl: Sized {
    fn GetRawData();
    fn GetDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOfflineFilesErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesErrorInfoVtbl {
        unsafe extern "system" fn GetRawData<Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppblob: *mut *mut super::super::System::Com::BYTE_BLOB) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IOfflineFilesErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRawData::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEventsVtbl {
        unsafe extern "system" fn CacheMoved<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CacheIsFull<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CacheIsCorrupted<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EncryptionChanged<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bwasencrypted: super::super::Foundation::BOOL, bwaspartial: super::super::Foundation::BOOL, bisencrypted: super::super::Foundation::BOOL, bispartial: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncBegin<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncFileResult<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncConflictRecAdded<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncConflictRecUpdated<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncConflictRecRemoved<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconflictpath: super::super::Foundation::PWSTR, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncEnd<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NetTransportArrived<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NoNetTransports<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemDisconnected<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemReconnected<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemAvailableOffline<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemNotAvailableOffline<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemPinned<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemNotPinned<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemModified<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemAddedToCache<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemDeletedFromCache<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemRenamed<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszoldpath: super::super::Foundation::PWSTR, psznewpath: super::super::Foundation::PWSTR, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataLost<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Ping<Impl: IOfflineFilesEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CacheMoved::<Impl, IMPL_OFFSET>,
            CacheIsFull::<Impl, IMPL_OFFSET>,
            CacheIsCorrupted::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            EncryptionChanged::<Impl, IMPL_OFFSET>,
            SyncBegin::<Impl, IMPL_OFFSET>,
            SyncFileResult::<Impl, IMPL_OFFSET>,
            SyncConflictRecAdded::<Impl, IMPL_OFFSET>,
            SyncConflictRecUpdated::<Impl, IMPL_OFFSET>,
            SyncConflictRecRemoved::<Impl, IMPL_OFFSET>,
            SyncEnd::<Impl, IMPL_OFFSET>,
            NetTransportArrived::<Impl, IMPL_OFFSET>,
            NoNetTransports::<Impl, IMPL_OFFSET>,
            ItemDisconnected::<Impl, IMPL_OFFSET>,
            ItemReconnected::<Impl, IMPL_OFFSET>,
            ItemAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemNotAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemPinned::<Impl, IMPL_OFFSET>,
            ItemNotPinned::<Impl, IMPL_OFFSET>,
            ItemModified::<Impl, IMPL_OFFSET>,
            ItemAddedToCache::<Impl, IMPL_OFFSET>,
            ItemDeletedFromCache::<Impl, IMPL_OFFSET>,
            ItemRenamed::<Impl, IMPL_OFFSET>,
            DataLost::<Impl, IMPL_OFFSET>,
            Ping::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEvents2Vtbl {
        unsafe extern "system" fn ItemReconnectBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemReconnectEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CacheEvictBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CacheEvictEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundSyncBegin<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackgroundSyncEnd<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsynccontrolflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PolicyChangeDetected<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreferenceChangeDetected<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SettingsChangesApplied<Impl: IOfflineFilesEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CacheMoved::<Impl, IMPL_OFFSET>,
            CacheIsFull::<Impl, IMPL_OFFSET>,
            CacheIsCorrupted::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            EncryptionChanged::<Impl, IMPL_OFFSET>,
            SyncBegin::<Impl, IMPL_OFFSET>,
            SyncFileResult::<Impl, IMPL_OFFSET>,
            SyncConflictRecAdded::<Impl, IMPL_OFFSET>,
            SyncConflictRecUpdated::<Impl, IMPL_OFFSET>,
            SyncConflictRecRemoved::<Impl, IMPL_OFFSET>,
            SyncEnd::<Impl, IMPL_OFFSET>,
            NetTransportArrived::<Impl, IMPL_OFFSET>,
            NoNetTransports::<Impl, IMPL_OFFSET>,
            ItemDisconnected::<Impl, IMPL_OFFSET>,
            ItemReconnected::<Impl, IMPL_OFFSET>,
            ItemAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemNotAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemPinned::<Impl, IMPL_OFFSET>,
            ItemNotPinned::<Impl, IMPL_OFFSET>,
            ItemModified::<Impl, IMPL_OFFSET>,
            ItemAddedToCache::<Impl, IMPL_OFFSET>,
            ItemDeletedFromCache::<Impl, IMPL_OFFSET>,
            ItemRenamed::<Impl, IMPL_OFFSET>,
            DataLost::<Impl, IMPL_OFFSET>,
            Ping::<Impl, IMPL_OFFSET>,
            ItemReconnectBegin::<Impl, IMPL_OFFSET>,
            ItemReconnectEnd::<Impl, IMPL_OFFSET>,
            CacheEvictBegin::<Impl, IMPL_OFFSET>,
            CacheEvictEnd::<Impl, IMPL_OFFSET>,
            BackgroundSyncBegin::<Impl, IMPL_OFFSET>,
            BackgroundSyncEnd::<Impl, IMPL_OFFSET>,
            PolicyChangeDetected::<Impl, IMPL_OFFSET>,
            PreferenceChangeDetected::<Impl, IMPL_OFFSET>,
            SettingsChangesApplied::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents3Impl: Sized + IOfflineFilesEvents2Impl + IOfflineFilesEventsImpl {
    fn TransparentCacheItemNotify();
    fn PrefetchFileBegin();
    fn PrefetchFileEnd();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEvents3Vtbl {
        unsafe extern "system" fn TransparentCacheItemNotify<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: super::super::Foundation::BOOL, bmodifiedattributes: super::super::Foundation::BOOL, pzsoldpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrefetchFileBegin<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrefetchFileEnd<Impl: IOfflineFilesEvents3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CacheMoved::<Impl, IMPL_OFFSET>,
            CacheIsFull::<Impl, IMPL_OFFSET>,
            CacheIsCorrupted::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            EncryptionChanged::<Impl, IMPL_OFFSET>,
            SyncBegin::<Impl, IMPL_OFFSET>,
            SyncFileResult::<Impl, IMPL_OFFSET>,
            SyncConflictRecAdded::<Impl, IMPL_OFFSET>,
            SyncConflictRecUpdated::<Impl, IMPL_OFFSET>,
            SyncConflictRecRemoved::<Impl, IMPL_OFFSET>,
            SyncEnd::<Impl, IMPL_OFFSET>,
            NetTransportArrived::<Impl, IMPL_OFFSET>,
            NoNetTransports::<Impl, IMPL_OFFSET>,
            ItemDisconnected::<Impl, IMPL_OFFSET>,
            ItemReconnected::<Impl, IMPL_OFFSET>,
            ItemAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemNotAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemPinned::<Impl, IMPL_OFFSET>,
            ItemNotPinned::<Impl, IMPL_OFFSET>,
            ItemModified::<Impl, IMPL_OFFSET>,
            ItemAddedToCache::<Impl, IMPL_OFFSET>,
            ItemDeletedFromCache::<Impl, IMPL_OFFSET>,
            ItemRenamed::<Impl, IMPL_OFFSET>,
            DataLost::<Impl, IMPL_OFFSET>,
            Ping::<Impl, IMPL_OFFSET>,
            ItemReconnectBegin::<Impl, IMPL_OFFSET>,
            ItemReconnectEnd::<Impl, IMPL_OFFSET>,
            CacheEvictBegin::<Impl, IMPL_OFFSET>,
            CacheEvictEnd::<Impl, IMPL_OFFSET>,
            BackgroundSyncBegin::<Impl, IMPL_OFFSET>,
            BackgroundSyncEnd::<Impl, IMPL_OFFSET>,
            PolicyChangeDetected::<Impl, IMPL_OFFSET>,
            PreferenceChangeDetected::<Impl, IMPL_OFFSET>,
            SettingsChangesApplied::<Impl, IMPL_OFFSET>,
            TransparentCacheItemNotify::<Impl, IMPL_OFFSET>,
            PrefetchFileBegin::<Impl, IMPL_OFFSET>,
            PrefetchFileEnd::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEvents4Impl: Sized + IOfflineFilesEvents3Impl + IOfflineFilesEvents2Impl + IOfflineFilesEventsImpl {
    fn PrefetchCloseHandleBegin();
    fn PrefetchCloseHandleEnd();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEvents4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEvents4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEvents4Vtbl {
        unsafe extern "system" fn PrefetchCloseHandleBegin<Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrefetchCloseHandleEnd<Impl: IOfflineFilesEvents4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclosedhandlecount: u32, dwopenhandlecount: u32, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CacheMoved::<Impl, IMPL_OFFSET>,
            CacheIsFull::<Impl, IMPL_OFFSET>,
            CacheIsCorrupted::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            EncryptionChanged::<Impl, IMPL_OFFSET>,
            SyncBegin::<Impl, IMPL_OFFSET>,
            SyncFileResult::<Impl, IMPL_OFFSET>,
            SyncConflictRecAdded::<Impl, IMPL_OFFSET>,
            SyncConflictRecUpdated::<Impl, IMPL_OFFSET>,
            SyncConflictRecRemoved::<Impl, IMPL_OFFSET>,
            SyncEnd::<Impl, IMPL_OFFSET>,
            NetTransportArrived::<Impl, IMPL_OFFSET>,
            NoNetTransports::<Impl, IMPL_OFFSET>,
            ItemDisconnected::<Impl, IMPL_OFFSET>,
            ItemReconnected::<Impl, IMPL_OFFSET>,
            ItemAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemNotAvailableOffline::<Impl, IMPL_OFFSET>,
            ItemPinned::<Impl, IMPL_OFFSET>,
            ItemNotPinned::<Impl, IMPL_OFFSET>,
            ItemModified::<Impl, IMPL_OFFSET>,
            ItemAddedToCache::<Impl, IMPL_OFFSET>,
            ItemDeletedFromCache::<Impl, IMPL_OFFSET>,
            ItemRenamed::<Impl, IMPL_OFFSET>,
            DataLost::<Impl, IMPL_OFFSET>,
            Ping::<Impl, IMPL_OFFSET>,
            ItemReconnectBegin::<Impl, IMPL_OFFSET>,
            ItemReconnectEnd::<Impl, IMPL_OFFSET>,
            CacheEvictBegin::<Impl, IMPL_OFFSET>,
            CacheEvictEnd::<Impl, IMPL_OFFSET>,
            BackgroundSyncBegin::<Impl, IMPL_OFFSET>,
            BackgroundSyncEnd::<Impl, IMPL_OFFSET>,
            PolicyChangeDetected::<Impl, IMPL_OFFSET>,
            PreferenceChangeDetected::<Impl, IMPL_OFFSET>,
            SettingsChangesApplied::<Impl, IMPL_OFFSET>,
            TransparentCacheItemNotify::<Impl, IMPL_OFFSET>,
            PrefetchFileBegin::<Impl, IMPL_OFFSET>,
            PrefetchFileEnd::<Impl, IMPL_OFFSET>,
            PrefetchCloseHandleBegin::<Impl, IMPL_OFFSET>,
            PrefetchCloseHandleEnd::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEvents4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesEventsFilterImpl: Sized {
    fn GetPathFilter();
    fn GetIncludedEvents();
    fn GetExcludedEvents();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesEventsFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesEventsFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesEventsFilterVtbl {
        unsafe extern "system" fn GetPathFilter<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilter: *mut super::super::Foundation::PWSTR, pmatch: *mut OFFLINEFILES_PATHFILTER_MATCH) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIncludedEvents<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExcludedEvents<Impl: IOfflineFilesEventsFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celements: u32, prgevents: *mut OFFLINEFILES_EVENTS, pcevents: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPathFilter::<Impl, IMPL_OFFSET>, GetIncludedEvents::<Impl, IMPL_OFFSET>, GetExcludedEvents::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesEventsFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileItemImpl: Sized + IOfflineFilesItemImpl {
    fn IsSparse();
    fn IsEncrypted();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesFileItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesFileItemVtbl {
        unsafe extern "system" fn IsSparse<Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbissparse: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEncrypted<Impl: IOfflineFilesFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisencrypted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemType::<Impl, IMPL_OFFSET>, GetPath::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, IsMarkedForDeletion::<Impl, IMPL_OFFSET>, IsSparse::<Impl, IMPL_OFFSET>, IsEncrypted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesFileItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesFileSysInfoImpl: Sized {
    fn GetAttributes();
    fn GetTimes();
    fn GetFileSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesFileSysInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesFileSysInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesFileSysInfoVtbl {
        unsafe extern "system" fn GetAttributes<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimes<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, pftcreationtime: *mut super::super::Foundation::FILETIME, pftlastwritetime: *mut super::super::Foundation::FILETIME, pftchangetime: *mut super::super::Foundation::FILETIME, pftlastaccesstime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: IOfflineFilesFileSysInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copy: OFFLINEFILES_ITEM_COPY, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAttributes::<Impl, IMPL_OFFSET>, GetTimes::<Impl, IMPL_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesFileSysInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesGhostInfoImpl: Sized {
    fn IsGhosted();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesGhostInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesGhostInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesGhostInfoVtbl {
        unsafe extern "system" fn IsGhosted<Impl: IOfflineFilesGhostInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbghosted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsGhosted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesGhostInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItemImpl: Sized {
    fn GetItemType();
    fn GetPath();
    fn GetParentItem();
    fn Refresh();
    fn IsMarkedForDeletion();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesItemVtbl {
        unsafe extern "system" fn GetItemType<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: *mut OFFLINEFILES_ITEM_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentItem<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsMarkedForDeletion<Impl: IOfflineFilesItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmarkedfordeletion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemType::<Impl, IMPL_OFFSET>, GetPath::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, IsMarkedForDeletion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItem as ::windows::core::Interface>::IID
    }
}
pub trait IOfflineFilesItemContainerImpl: Sized {
    fn EnumItems();
    fn EnumItemsEx();
}
impl IOfflineFilesItemContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesItemContainerVtbl {
        unsafe extern "system" fn EnumItems<Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumItemsEx<Impl: IOfflineFilesItemContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pincludefilefilter: ::windows::core::RawPtr, pincludedirfilter: ::windows::core::RawPtr, pexcludefilefilter: ::windows::core::RawPtr, pexcludedirfilter: ::windows::core::RawPtr, dwenumflags: u32, dwqueryflags: u32, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnumItems::<Impl, IMPL_OFFSET>, EnumItemsEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItemContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesItemFilterImpl: Sized {
    fn GetFilterFlags();
    fn GetTimeFilter();
    fn GetPatternFilter();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesItemFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesItemFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesItemFilterVtbl {
        unsafe extern "system" fn GetFilterFlags<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullflags: *mut u64, pullmask: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeFilter<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfttime: *mut super::super::Foundation::FILETIME, pbevaltimeofday: *mut super::super::Foundation::BOOL, ptimetype: *mut OFFLINEFILES_ITEM_TIME, pcompare: *mut OFFLINEFILES_COMPARE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPatternFilter<Impl: IOfflineFilesItemFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpattern: super::super::Foundation::PWSTR, cchpattern: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFilterFlags::<Impl, IMPL_OFFSET>, GetTimeFilter::<Impl, IMPL_OFFSET>, GetPatternFilter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesItemFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfoImpl: Sized {
    fn IsPinned();
    fn IsPinnedForUser();
    fn IsPinnedForUserByPolicy();
    fn IsPinnedForComputer();
    fn IsPinnedForFolderRedirection();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesPinInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesPinInfoVtbl {
        unsafe extern "system" fn IsPinned<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPinnedForUser<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPinnedForUserByPolicy<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPinnedForComputer<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPinnedForFolderRedirection<Impl: IOfflineFilesPinInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsPinned::<Impl, IMPL_OFFSET>, IsPinnedForUser::<Impl, IMPL_OFFSET>, IsPinnedForUserByPolicy::<Impl, IMPL_OFFSET>, IsPinnedForComputer::<Impl, IMPL_OFFSET>, IsPinnedForFolderRedirection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesPinInfo2Impl: Sized + IOfflineFilesPinInfoImpl {
    fn IsPartlyPinned();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesPinInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesPinInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesPinInfo2Vtbl {
        unsafe extern "system" fn IsPartlyPinned<Impl: IOfflineFilesPinInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpartlypinned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsPinned::<Impl, IMPL_OFFSET>, IsPinnedForUser::<Impl, IMPL_OFFSET>, IsPinnedForUserByPolicy::<Impl, IMPL_OFFSET>, IsPinnedForComputer::<Impl, IMPL_OFFSET>, IsPinnedForFolderRedirection::<Impl, IMPL_OFFSET>, IsPartlyPinned::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesPinInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesProgressImpl: Sized {
    fn Begin();
    fn QueryAbort();
    fn End();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesProgressVtbl {
        unsafe extern "system" fn Begin<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAbort<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbabort: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn End<Impl: IOfflineFilesProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin::<Impl, IMPL_OFFSET>, QueryAbort::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>)
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemType::<Impl, IMPL_OFFSET>, GetPath::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, IsMarkedForDeletion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesServerItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IOfflineFilesSettingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSettingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSettingVtbl {
        unsafe extern "system" fn GetName<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValueType<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut OFFLINEFILES_SETTING_VALUE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreferenceScope<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeletePreference<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicy<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, dwscope: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPolicyScope<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscope: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IOfflineFilesSettingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT, pbsetbypolicy: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetName::<Impl, IMPL_OFFSET>, GetValueType::<Impl, IMPL_OFFSET>, GetPreference::<Impl, IMPL_OFFSET>, GetPreferenceScope::<Impl, IMPL_OFFSET>, SetPreference::<Impl, IMPL_OFFSET>, DeletePreference::<Impl, IMPL_OFFSET>, GetPolicy::<Impl, IMPL_OFFSET>, GetPolicyScope::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSetting as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesShareInfoImpl: Sized {
    fn GetShareItem();
    fn GetShareCachingMode();
    fn IsShareDfsJunction();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesShareInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesShareInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesShareInfoVtbl {
        unsafe extern "system" fn GetShareItem<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppshareitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShareCachingMode<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsShareDfsJunction<Impl: IOfflineFilesShareInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisdfsjunction: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetShareItem::<Impl, IMPL_OFFSET>, GetShareCachingMode::<Impl, IMPL_OFFSET>, IsShareDfsJunction::<Impl, IMPL_OFFSET>)
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetItemType::<Impl, IMPL_OFFSET>, GetPath::<Impl, IMPL_OFFSET>, GetParentItem::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, IsMarkedForDeletion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesShareItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSimpleProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn ItemBegin();
    fn ItemResult();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSimpleProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSimpleProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSimpleProgressVtbl {
        unsafe extern "system" fn ItemBegin<Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemResult<Impl: IOfflineFilesSimpleProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin::<Impl, IMPL_OFFSET>, QueryAbort::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, ItemBegin::<Impl, IMPL_OFFSET>, ItemResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSimpleProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspendImpl: Sized {
    fn SuspendRoot();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSuspendVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSuspendVtbl {
        unsafe extern "system" fn SuspendRoot<Impl: IOfflineFilesSuspendImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsuspend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SuspendRoot::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSuspend as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSuspendInfoImpl: Sized {
    fn IsSuspended();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSuspendInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSuspendInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSuspendInfoVtbl {
        unsafe extern "system" fn IsSuspended<Impl: IOfflineFilesSuspendInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsuspended: *mut super::super::Foundation::BOOL, pbsuspendedroot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsSuspended::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSuspendInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncConflictHandlerImpl: Sized {
    fn ResolveConflict();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncConflictHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncConflictHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncConflictHandlerVtbl {
        unsafe extern "system" fn ResolveConflict<Impl: IOfflineFilesSyncConflictHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR, fstateknown: u32, state: OFFLINEFILES_SYNC_STATE, fchangedetails: u32, pconflictresolution: *mut OFFLINEFILES_SYNC_CONFLICT_RESOLVE, ppsznewname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ResolveConflict::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncConflictHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOfflineFilesSyncErrorInfoImpl: Sized + IOfflineFilesErrorInfoImpl {
    fn GetSyncOperation();
    fn GetItemChangeFlags();
    fn InfoEnumerated();
    fn InfoAvailable();
    fn GetLocalInfo();
    fn GetRemoteInfo();
    fn GetOriginalInfo();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IOfflineFilesSyncErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncErrorInfoVtbl {
        unsafe extern "system" fn GetSyncOperation<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psyncop: *mut OFFLINEFILES_SYNC_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetItemChangeFlags<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwitemchangeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InfoEnumerated<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalenumerated: *mut super::super::Foundation::BOOL, pbremoteenumerated: *mut super::super::Foundation::BOOL, pboriginalenumerated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InfoAvailable<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblocalinfo: *mut super::super::Foundation::BOOL, pbremoteinfo: *mut super::super::Foundation::BOOL, pboriginalinfo: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRemoteInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginalInfo<Impl: IOfflineFilesSyncErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetRawData::<Impl, IMPL_OFFSET>,
            GetDescription::<Impl, IMPL_OFFSET>,
            GetSyncOperation::<Impl, IMPL_OFFSET>,
            GetItemChangeFlags::<Impl, IMPL_OFFSET>,
            InfoEnumerated::<Impl, IMPL_OFFSET>,
            InfoAvailable::<Impl, IMPL_OFFSET>,
            GetLocalInfo::<Impl, IMPL_OFFSET>,
            GetRemoteInfo::<Impl, IMPL_OFFSET>,
            GetOriginalInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncErrorItemInfoImpl: Sized {
    fn GetFileAttributes();
    fn GetFileTimes();
    fn GetFileSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncErrorItemInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncErrorItemInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncErrorItemInfoVtbl {
        unsafe extern "system" fn GetFileAttributes<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileTimes<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftlastwrite: *mut super::super::Foundation::FILETIME, pftchange: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: IOfflineFilesSyncErrorItemInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psize: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFileAttributes::<Impl, IMPL_OFFSET>, GetFileTimes::<Impl, IMPL_OFFSET>, GetFileSize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncErrorItemInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesSyncProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn SyncItemBegin();
    fn SyncItemResult();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesSyncProgressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesSyncProgressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesSyncProgressVtbl {
        unsafe extern "system" fn SyncItemBegin<Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncItemResult<Impl: IOfflineFilesSyncProgressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfile: super::super::Foundation::PWSTR, hrresult: ::windows::core::HRESULT, perrorinfo: ::windows::core::RawPtr, presponse: *mut OFFLINEFILES_OP_RESPONSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin::<Impl, IMPL_OFFSET>, QueryAbort::<Impl, IMPL_OFFSET>, End::<Impl, IMPL_OFFSET>, SyncItemBegin::<Impl, IMPL_OFFSET>, SyncItemResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesSyncProgress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOfflineFilesTransparentCacheInfoImpl: Sized {
    fn IsTransparentlyCached();
}
#[cfg(feature = "Win32_Foundation")]
impl IOfflineFilesTransparentCacheInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineFilesTransparentCacheInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineFilesTransparentCacheInfoVtbl {
        unsafe extern "system" fn IsTransparentlyCached<Impl: IOfflineFilesTransparentCacheInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbtransparentlycached: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsTransparentlyCached::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineFilesTransparentCacheInfo as ::windows::core::Interface>::IID
    }
}

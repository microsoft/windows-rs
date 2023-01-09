impl ::core::cmp::PartialEq for IEnumOfflineFilesItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOfflineFilesItems {}
impl ::core::fmt::Debug for IEnumOfflineFilesItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOfflineFilesItems").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumOfflineFilesSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumOfflineFilesSettings {}
impl ::core::fmt::Debug for IEnumOfflineFilesSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumOfflineFilesSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesCache {}
impl ::core::fmt::Debug for IOfflineFilesCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesCache").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesCache2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesCache2 {}
impl ::core::fmt::Debug for IOfflineFilesCache2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesCache2").field(&self.0).finish()
    }
}
impl IOfflineFilesCache2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Synchronize<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows::core::PCWSTR], basync: P1, dwsynccontrol: u32, pisyncconflicthandler: P2, piprogress: P3, psyncid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncConflictHandler>>,
        P3: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Synchronize)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, basync.into(), dwsynccontrol, pisyncconflicthandler.into().abi(), piprogress.into().abi(), ::core::mem::transmute(psyncid.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteItems<P0, P1>(&self, rgpszpaths: &[::windows::core::PCWSTR], dwflags: u32, basync: P0, piprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSimpleProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteItems)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, dwflags, basync.into(), piprogress.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteItemsForUser<P0, P1, P2>(&self, pszuser: P0, rgpszpaths: &[::windows::core::PCWSTR], dwflags: u32, basync: P1, piprogress: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSimpleProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteItemsForUser)(::windows::core::Vtable::as_raw(self), pszuser.into().abi(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, dwflags, basync.into(), piprogress.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Pin<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows::core::PCWSTR], bdeep: P1, basync: P2, dwpincontrolflags: u32, piprogress: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Pin)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, bdeep.into(), basync.into(), dwpincontrolflags, piprogress.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unpin<P0, P1, P2, P3>(&self, hwndparent: P0, rgpszpaths: &[::windows::core::PCWSTR], bdeep: P1, basync: P2, dwpincontrolflags: u32, piprogress: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Unpin)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ::core::mem::transmute(rgpszpaths.as_ptr()), rgpszpaths.len() as _, bdeep.into(), basync.into(), dwpincontrolflags, piprogress.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEncryptionStatus(&self, pbencrypted: *mut super::super::Foundation::BOOL, pbpartial: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEncryptionStatus)(::windows::core::Vtable::as_raw(self), pbencrypted, pbpartial).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Encrypt<P0, P1, P2, P3>(&self, hwndparent: P0, bencrypt: P1, dwencryptioncontrolflags: u32, basync: P2, piprogress: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Encrypt)(::windows::core::Vtable::as_raw(self), hwndparent.into(), bencrypt.into(), dwencryptioncontrolflags, basync.into(), piprogress.into().abi()).ok()
    }
    pub unsafe fn FindItem<P0>(&self, pszpath: P0, dwqueryflags: u32) -> ::windows::core::Result<IOfflineFilesItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindItem)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), dwqueryflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindItemEx<P0, P1, P2, P3, P4>(&self, pszpath: P0, pincludefilefilter: P1, pincludedirfilter: P2, pexcludefilefilter: P3, pexcludedirfilter: P4, dwqueryflags: u32) -> ::windows::core::Result<IOfflineFilesItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IOfflineFilesItemFilter>>,
        P2: ::std::convert::Into<::windows::core::InParam<IOfflineFilesItemFilter>>,
        P3: ::std::convert::Into<::windows::core::InParam<IOfflineFilesItemFilter>>,
        P4: ::std::convert::Into<::windows::core::InParam<IOfflineFilesItemFilter>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindItemEx)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), pincludefilefilter.into().abi(), pincludedirfilter.into().abi(), pexcludefilefilter.into().abi(), pexcludedirfilter.into().abi(), dwqueryflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenameItem<P0, P1, P2>(&self, pszpathoriginal: P0, pszpathnew: P1, breplaceifexists: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RenameItem)(::windows::core::Vtable::as_raw(self), pszpathoriginal.into().abi(), pszpathnew.into().abi(), breplaceifexists.into()).ok()
    }
    pub unsafe fn GetLocation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLocation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDiskSpaceInformation(&self, pcbvolumetotal: *mut u64, pcblimit: *mut u64, pcbused: *mut u64, pcbunpinnedlimit: *mut u64, pcbunpinnedused: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDiskSpaceInformation)(::windows::core::Vtable::as_raw(self), pcbvolumetotal, pcblimit, pcbused, pcbunpinnedlimit, pcbunpinnedused).ok()
    }
    pub unsafe fn SetDiskSpaceLimits(&self, cblimit: u64, cbunpinnedlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDiskSpaceLimits)(::windows::core::Vtable::as_raw(self), cblimit, cbunpinnedlimit).ok()
    }
    pub unsafe fn ProcessAdminPinPolicy<P0, P1>(&self, ppinprogress: P0, punpinprogress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncProgress>>,
        P1: ::std::convert::Into<::windows::core::InParam<IOfflineFilesSyncProgress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ProcessAdminPinPolicy)(::windows::core::Vtable::as_raw(self), ppinprogress.into().abi(), punpinprogress.into().abi()).ok()
    }
    pub unsafe fn GetSettingObject<P0>(&self, pszsettingname: P0) -> ::windows::core::Result<IOfflineFilesSetting>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSettingObject)(::windows::core::Vtable::as_raw(self), pszsettingname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumSettingObjects(&self) -> ::windows::core::Result<IEnumOfflineFilesSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumSettingObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathCacheable<P0>(&self, pszpath: P0, pbcacheable: *mut super::super::Foundation::BOOL, psharecachingmode: *mut OFFLINEFILES_CACHING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IsPathCacheable)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), pbcacheable, psharecachingmode).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesChangeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesChangeInfo {}
impl ::core::fmt::Debug for IOfflineFilesChangeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesChangeInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesConnectionInfo {}
impl ::core::fmt::Debug for IOfflineFilesConnectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesConnectionInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesDirectoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesDirectoryItem {}
impl ::core::fmt::Debug for IOfflineFilesDirectoryItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesDirectoryItem").field(&self.0).finish()
    }
}
impl IOfflineFilesDirectoryItem {
    pub unsafe fn GetItemType(&self) -> ::windows::core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows::core::Result<IOfflineFilesItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh)(::windows::core::Vtable::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsMarkedForDeletion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesDirtyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesDirtyInfo {}
impl ::core::fmt::Debug for IOfflineFilesDirtyInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesDirtyInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesErrorInfo {}
impl ::core::fmt::Debug for IOfflineFilesErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesErrorInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesEvents {}
impl ::core::fmt::Debug for IOfflineFilesEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesEvents2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesEvents2 {}
impl ::core::fmt::Debug for IOfflineFilesEvents2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesEvents2").field(&self.0).finish()
    }
}
impl IOfflineFilesEvents2 {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CacheMoved)(::windows::core::Vtable::as_raw(self), pszoldpath.into().abi(), psznewpath.into().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CacheIsFull)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CacheIsCorrupted)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Enabled)(::windows::core::Vtable::as_raw(self), benabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EncryptionChanged)(::windows::core::Vtable::as_raw(self), bwasencrypted.into(), bwaspartial.into(), bisencrypted.into(), bispartial.into()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SyncBegin)(::windows::core::Vtable::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows::core::GUID, pszfile: P0, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SyncFileResult)(::windows::core::Vtable::as_raw(self), rsyncid, pszfile.into().abi(), hrresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SyncConflictRecAdded)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SyncConflictRecUpdated)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SyncConflictRecRemoved)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SyncEnd)(::windows::core::Vtable::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NetTransportArrived)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NoNetTransports)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemDisconnected)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemReconnected)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemAvailableOffline)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemNotAvailableOffline)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemPinned)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemNotPinned)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemModified)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype, bmodifieddata.into(), bmodifiedattributes.into()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemAddedToCache)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemDeletedFromCache)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ItemRenamed)(::windows::core::Vtable::as_raw(self), pszoldpath.into().abi(), psznewpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DataLost)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Ping)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesEvents3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesEvents3 {}
impl ::core::fmt::Debug for IOfflineFilesEvents3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesEvents3").field(&self.0).finish()
    }
}
impl IOfflineFilesEvents3 {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CacheMoved)(::windows::core::Vtable::as_raw(self), pszoldpath.into().abi(), psznewpath.into().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CacheIsFull)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CacheIsCorrupted)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Enabled)(::windows::core::Vtable::as_raw(self), benabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EncryptionChanged)(::windows::core::Vtable::as_raw(self), bwasencrypted.into(), bwaspartial.into(), bisencrypted.into(), bispartial.into()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SyncBegin)(::windows::core::Vtable::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows::core::GUID, pszfile: P0, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SyncFileResult)(::windows::core::Vtable::as_raw(self), rsyncid, pszfile.into().abi(), hrresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SyncConflictRecAdded)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SyncConflictRecUpdated)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SyncConflictRecRemoved)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SyncEnd)(::windows::core::Vtable::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NetTransportArrived)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NoNetTransports)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemDisconnected)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemReconnected)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemAvailableOffline)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemNotAvailableOffline)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemPinned)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemNotPinned)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemModified)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype, bmodifieddata.into(), bmodifiedattributes.into()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemAddedToCache)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemDeletedFromCache)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemRenamed)(::windows::core::Vtable::as_raw(self), pszoldpath.into().abi(), psznewpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DataLost)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Ping)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ItemReconnectBegin)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ItemReconnectEnd)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CacheEvictBegin)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictEnd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CacheEvictEnd)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BackgroundSyncBegin)(::windows::core::Vtable::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BackgroundSyncEnd)(::windows::core::Vtable::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PolicyChangeDetected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PreferenceChangeDetected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SettingsChangesApplied)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesEvents4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesEvents4 {}
impl ::core::fmt::Debug for IOfflineFilesEvents4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesEvents4").field(&self.0).finish()
    }
}
impl IOfflineFilesEvents4 {
    pub unsafe fn CacheMoved<P0, P1>(&self, pszoldpath: P0, psznewpath: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CacheMoved)(::windows::core::Vtable::as_raw(self), pszoldpath.into().abi(), psznewpath.into().abi()).ok()
    }
    pub unsafe fn CacheIsFull(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CacheIsFull)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheIsCorrupted(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CacheIsCorrupted)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled<P0>(&self, benabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Enabled)(::windows::core::Vtable::as_raw(self), benabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EncryptionChanged<P0, P1, P2, P3>(&self, bwasencrypted: P0, bwaspartial: P1, bisencrypted: P2, bispartial: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EncryptionChanged)(::windows::core::Vtable::as_raw(self), bwasencrypted.into(), bwaspartial.into(), bisencrypted.into(), bispartial.into()).ok()
    }
    pub unsafe fn SyncBegin(&self, rsyncid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SyncBegin)(::windows::core::Vtable::as_raw(self), rsyncid).ok()
    }
    pub unsafe fn SyncFileResult<P0>(&self, rsyncid: *const ::windows::core::GUID, pszfile: P0, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SyncFileResult)(::windows::core::Vtable::as_raw(self), rsyncid, pszfile.into().abi(), hrresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecAdded<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SyncConflictRecAdded)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecUpdated<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SyncConflictRecUpdated)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConflictRecRemoved<P0>(&self, pszconflictpath: P0, pftconflictdatetime: *const super::super::Foundation::FILETIME, conflictsyncstate: OFFLINEFILES_SYNC_STATE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SyncConflictRecRemoved)(::windows::core::Vtable::as_raw(self), pszconflictpath.into().abi(), pftconflictdatetime, conflictsyncstate).ok()
    }
    pub unsafe fn SyncEnd(&self, rsyncid: *const ::windows::core::GUID, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SyncEnd)(::windows::core::Vtable::as_raw(self), rsyncid, hrresult).ok()
    }
    pub unsafe fn NetTransportArrived(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NetTransportArrived)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn NoNetTransports(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NoNetTransports)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemDisconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemDisconnected)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemReconnected<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemReconnected)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemAvailableOffline)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotAvailableOffline<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemNotAvailableOffline)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemPinned)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemNotPinned<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemNotPinned)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ItemModified<P0, P1, P2>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemModified)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype, bmodifieddata.into(), bmodifiedattributes.into()).ok()
    }
    pub unsafe fn ItemAddedToCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemAddedToCache)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemDeletedFromCache<P0>(&self, pszpath: P0, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemDeletedFromCache)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn ItemRenamed<P0, P1>(&self, pszoldpath: P0, psznewpath: P1, itemtype: OFFLINEFILES_ITEM_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ItemRenamed)(::windows::core::Vtable::as_raw(self), pszoldpath.into().abi(), psznewpath.into().abi(), itemtype).ok()
    }
    pub unsafe fn DataLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DataLost)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Ping(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Ping)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemReconnectBegin)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ItemReconnectEnd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ItemReconnectEnd)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictBegin(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CacheEvictBegin)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CacheEvictEnd(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CacheEvictEnd)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn BackgroundSyncBegin(&self, dwsynccontrolflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.BackgroundSyncBegin)(::windows::core::Vtable::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn BackgroundSyncEnd(&self, dwsynccontrolflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.BackgroundSyncEnd)(::windows::core::Vtable::as_raw(self), dwsynccontrolflags).ok()
    }
    pub unsafe fn PolicyChangeDetected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PolicyChangeDetected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PreferenceChangeDetected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PreferenceChangeDetected)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SettingsChangesApplied(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SettingsChangesApplied)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransparentCacheItemNotify<P0, P1, P2, P3>(&self, pszpath: P0, eventtype: OFFLINEFILES_EVENTS, itemtype: OFFLINEFILES_ITEM_TYPE, bmodifieddata: P1, bmodifiedattributes: P2, pzsoldpath: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TransparentCacheItemNotify)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), eventtype, itemtype, bmodifieddata.into(), bmodifiedattributes.into(), pzsoldpath.into().abi()).ok()
    }
    pub unsafe fn PrefetchFileBegin<P0>(&self, pszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PrefetchFileBegin)(::windows::core::Vtable::as_raw(self), pszpath.into().abi()).ok()
    }
    pub unsafe fn PrefetchFileEnd<P0>(&self, pszpath: P0, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PrefetchFileEnd)(::windows::core::Vtable::as_raw(self), pszpath.into().abi(), hrresult).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesEventsFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesEventsFilter {}
impl ::core::fmt::Debug for IOfflineFilesEventsFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesEventsFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesFileItem {}
impl ::core::fmt::Debug for IOfflineFilesFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesFileItem").field(&self.0).finish()
    }
}
impl IOfflineFilesFileItem {
    pub unsafe fn GetItemType(&self) -> ::windows::core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows::core::Result<IOfflineFilesItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh)(::windows::core::Vtable::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsMarkedForDeletion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesFileSysInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesFileSysInfo {}
impl ::core::fmt::Debug for IOfflineFilesFileSysInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesFileSysInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesGhostInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesGhostInfo {}
impl ::core::fmt::Debug for IOfflineFilesGhostInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesGhostInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesItem {}
impl ::core::fmt::Debug for IOfflineFilesItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesItemContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesItemContainer {}
impl ::core::fmt::Debug for IOfflineFilesItemContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesItemContainer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesItemFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesItemFilter {}
impl ::core::fmt::Debug for IOfflineFilesItemFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesItemFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesPinInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesPinInfo {}
impl ::core::fmt::Debug for IOfflineFilesPinInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesPinInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesPinInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesPinInfo2 {}
impl ::core::fmt::Debug for IOfflineFilesPinInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesPinInfo2").field(&self.0).finish()
    }
}
impl IOfflineFilesPinInfo2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinned(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsPinned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForUser(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsPinnedForUser)(::windows::core::Vtable::as_raw(self), pbpinnedforuser, pbinherit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForUserByPolicy(&self, pbpinnedforuser: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsPinnedForUserByPolicy)(::windows::core::Vtable::as_raw(self), pbpinnedforuser, pbinherit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForComputer(&self, pbpinnedforcomputer: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsPinnedForComputer)(::windows::core::Vtable::as_raw(self), pbpinnedforcomputer, pbinherit).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPinnedForFolderRedirection(&self, pbpinnedforfolderredirection: *mut super::super::Foundation::BOOL, pbinherit: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsPinnedForFolderRedirection)(::windows::core::Vtable::as_raw(self), pbpinnedforfolderredirection, pbinherit).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesProgress {}
impl ::core::fmt::Debug for IOfflineFilesProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesServerItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesServerItem {}
impl ::core::fmt::Debug for IOfflineFilesServerItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesServerItem").field(&self.0).finish()
    }
}
impl IOfflineFilesServerItem {
    pub unsafe fn GetItemType(&self) -> ::windows::core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows::core::Result<IOfflineFilesItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh)(::windows::core::Vtable::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsMarkedForDeletion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSetting {}
impl ::core::fmt::Debug for IOfflineFilesSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSetting").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesShareInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesShareInfo {}
impl ::core::fmt::Debug for IOfflineFilesShareInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesShareInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesShareItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesShareItem {}
impl ::core::fmt::Debug for IOfflineFilesShareItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesShareItem").field(&self.0).finish()
    }
}
impl IOfflineFilesShareItem {
    pub unsafe fn GetItemType(&self) -> ::windows::core::Result<OFFLINEFILES_ITEM_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetParentItem(&self) -> ::windows::core::Result<IOfflineFilesItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentItem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Refresh(&self, dwqueryflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Refresh)(::windows::core::Vtable::as_raw(self), dwqueryflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMarkedForDeletion(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsMarkedForDeletion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSimpleProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSimpleProgress {}
impl ::core::fmt::Debug for IOfflineFilesSimpleProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSimpleProgress").field(&self.0).finish()
    }
}
impl IOfflineFilesSimpleProgress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Begin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryAbort(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryAbort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn End(&self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self), hrresult).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSuspend {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSuspend {}
impl ::core::fmt::Debug for IOfflineFilesSuspend {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSuspend").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSuspendInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSuspendInfo {}
impl ::core::fmt::Debug for IOfflineFilesSuspendInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSuspendInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSyncConflictHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSyncConflictHandler {}
impl ::core::fmt::Debug for IOfflineFilesSyncConflictHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSyncConflictHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSyncErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSyncErrorInfo {}
impl ::core::fmt::Debug for IOfflineFilesSyncErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSyncErrorInfo").field(&self.0).finish()
    }
}
impl IOfflineFilesSyncErrorInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRawData(&self) -> ::windows::core::Result<*mut super::super::System::Com::BYTE_BLOB> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRawData)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSyncErrorItemInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSyncErrorItemInfo {}
impl ::core::fmt::Debug for IOfflineFilesSyncErrorItemInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSyncErrorItemInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesSyncProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesSyncProgress {}
impl ::core::fmt::Debug for IOfflineFilesSyncProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesSyncProgress").field(&self.0).finish()
    }
}
impl IOfflineFilesSyncProgress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Begin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryAbort(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryAbort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn End(&self, hrresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self), hrresult).ok()
    }
}
impl ::core::cmp::PartialEq for IOfflineFilesTransparentCacheInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOfflineFilesTransparentCacheInfo {}
impl ::core::fmt::Debug for IOfflineFilesTransparentCacheInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOfflineFilesTransparentCacheInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_CACHING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_CACHING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_CACHING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_COMPARE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_COMPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_COMPARE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_CONNECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_CONNECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_CONNECT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_EVENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_EVENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_EVENTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_ITEM_COPY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_ITEM_COPY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_ITEM_COPY").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_ITEM_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_ITEM_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_ITEM_TIME").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_ITEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_OFFLINE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_OFFLINE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_OFFLINE_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_OP_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_OP_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_OP_RESPONSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_PATHFILTER_MATCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_PATHFILTER_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_PATHFILTER_MATCH").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_SETTING_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_SETTING_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SETTING_VALUE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_SYNC_CONFLICT_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SYNC_CONFLICT_RESOLVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_SYNC_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_SYNC_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SYNC_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLINEFILES_SYNC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLINEFILES_SYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLINEFILES_SYNC_STATE").field(&self.0).finish()
    }
}
